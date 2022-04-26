use clap::Parser;
use env_logger::Env;
use futures::FutureExt;
use futures::select;
use futures::{Sink, SinkExt, Stream, StreamExt};
use spytrap_wifi::args::{Args, SubCommand};
use spytrap_wifi::args::Start;
use spytrap_wifi::errors::*;
use spytrap_wifi::hostapd;
use spytrap_wifi::json;
use spytrap_wifi::ioc;
use spytrap_wifi::rpc;
use spytrap_wifi::stdio;
use spytrap_wifi::suffix::SuffixTree;
use std::process::Stdio;
use tokio::process::Command;
use tokio::io::AsyncWriteExt;
use tokio::io::{BufReader, AsyncBufReadExt};


// this function must not error or panic
async fn process<S: Sink<String> + Unpin>(line: &[u8], iocs: &SuffixTree<String>, sink: &mut S) {
    if let Ok(pkt) = json::parse(line) {
        let names = pkt.get_names();
        for (src, name) in names {
            if iocs.matches(&name) {
                warn!("detected({}): {:?}", src.as_str(), name);
                send(sink, format!("[!] detected({}): {:?}", src.as_str(), name)).await.ok();
            } else {
                debug!("observed({}): {:?}", src.as_str(), name);
            }
        }
    }
}

async fn send<S: Sink<String> + Unpin>(sink: &mut S, value: String) -> Result<()> {
    sink.send(value).await.map_err(|_| anyhow!("sink error"))
}

async fn stream<R: Stream<Item=String> + Unpin, S: Sink<String> + Unpin>(mut rx: R, tx: &mut S, path: &str) -> Result<()> {
    let iocs = ioc::load(path)
        .with_context(|| anyhow!("Failed to load iocs from {:?}", path))?;
    info!("Loaded {} known IOCs", iocs.len());

    while let Some(line) = rx.next().await {
        process(line.as_bytes(), &iocs, tx).await;
    }

    Ok(())
}

async fn hotspot<R: Stream<Item=String> + Unpin, S: Sink<String> + Unpin>(mut stream: R, mut sink: S, path: &str) -> Result<()> {
    loop {
        let ssid = "Starbucks WiFi";
        let pw = hostapd::pwgen();

        info!("Writing hostapd config");
        hostapd::write_config(path, "wlan1", ssid, &pw).await
            .context("Failed to write hostapd config")?;
        info!("Restarting hostapd");
        hostapd::restart().await.ok();

        let line = format!("[+] {:?} (pw: {})", ssid, pw);
        send(&mut sink, line).await?;

        // wait for signal
        if stream.next().await.is_none() {
            break;
        }
    }
    Ok(())
}

async fn screen<S: Stream<Item=String> + Unpin>(mut stream: S, bin: &str) -> Result<()> {
    info!("Spawn python script");

    let mut cmd = Command::new(bin);
    cmd.stdin(Stdio::piped());

    let mut child = cmd.spawn()
        .expect("failed to spawn command");

    let mut stdin = child.stdin.take()
        .expect("child did not have a handle to stdin");

    // Ensure the child process is spawned in the runtime so it can
    // make progress on its own while we await for any output.
    let join = tokio::spawn(async move {
        let status = child.wait().await
            .expect("child process encountered an error");

        error!("child status was: {}", status);
    });

    while let Some(item) = stream.next().await {
        info!("Sending to screen: {:?}", item);
        let line = format!("{}\n", item);
        stdin.write(line.as_bytes()).await?;
    }

    join.await.ok();

    Ok(())
}

async fn sniff<S: Sink<String> + Unpin>(mut sink: S, dev: &str) -> Result<()> {
    loop {
        info!("Spawning sniffglue");

        let mut cmd = Command::new("sniffglue");
        cmd.args(&["--json", dev]);

        // Specify that we want the command's standard output piped back to us.
        // By default, standard input/output/error will be inherited from the
        // current process (for example, this means that standard input will
        // come from the keyboard and standard output/error will go directly to
        // the terminal if this process is invoked from the command line).
        cmd.stdout(Stdio::piped());

        let mut child = cmd.spawn()
            .expect("failed to spawn command");

        let stdout = child.stdout.take()
            .expect("child did not have a handle to stdout");

        let mut reader = BufReader::new(stdout).lines();

        // Ensure the child process is spawned in the runtime so it can
        // make progress on its own while we await for any output.
        let join = tokio::spawn(async move {
            let status = child.wait().await
                .expect("child process encountered an error");

            error!("child status was: {}", status);
        });

        while let Some(line) = reader.next_line().await? {
            send(&mut sink, line).await?;
        }

        join.await.ok();
   }
}

async fn start(args: Start) -> Result<()> {
    let (mut screen_tx, screen_rx) = futures::channel::mpsc::channel(0);

    let (tx1, rx1) = futures::channel::mpsc::channel(256);
    let (tx2, rx2) = futures::channel::mpsc::channel(256);

    select! {
        rpc = rpc::spawn(&args.socket, tx1).fuse() => rpc,
        // rpc = stdio::stdin(tx1).fuse() => rpc,
        hotspot = hotspot(rx1, screen_tx.clone(), &args.file).fuse() => hotspot,

        sniff = sniff(tx2, &args.device).fuse() => sniff,
        stream = stream(rx2, &mut screen_tx, &args.rules).fuse() => stream,

        screen = screen(screen_rx, &args.screen).fuse() => screen,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default()
        .default_filter_or("spytrap=info"));

    let args = Args::parse();
    match args.subcommand {
        SubCommand::Start(args) => start(args).await,
        SubCommand::Send(args) => rpc::send(&args.socket, args.value).await,
        SubCommand::Sniff(args) => {
            let (tx, _rx) = futures::channel::mpsc::channel(256);
            sniff(tx, &args.device).await
        }
        SubCommand::Stream(args) => {
            let (tx1, rx1) = futures::channel::mpsc::channel(256);
            let (mut tx2, rx2) = futures::channel::mpsc::channel(256);
            select! {
                x = stdio::stdin(tx1).fuse() => x,
                x = stream(rx1, &mut tx2, &args.rules).fuse() => x,
                x = stdio::stdout(rx2).fuse() => x,
            }?;

            Ok(())
        }
        SubCommand::Screen(args) => {
            let (tx, rx) = futures::channel::mpsc::channel(256);
            select! {
                stdin = stdio::stdin(tx).fuse() => stdin,
                screen = screen(rx, &args.screen).fuse() => screen,
            }
        }
        SubCommand::Hotspot(args) => {
            let (tx1, rx1) = futures::channel::mpsc::channel(256);
            let (tx2, rx2) = futures::channel::mpsc::channel(256);
            select! {
                _stdin = stdio::stdin(tx1).fuse() => (),
                _hotspot = hotspot(rx1, tx2, &args.file).fuse() => (),
                _stdout = stdio::stdout(rx2).fuse() => (),
            }
            Ok(())
        }
    }
}
