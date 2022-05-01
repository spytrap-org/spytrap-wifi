use futures::Sink;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use futures::channel::mpsc::Sender;
use crate::errors::*;
use tokio::fs;
use tokio::net::UnixListener;
use tokio::net::UnixStream;
use tokio::io::AsyncWriteExt;
use tokio::io::{BufReader, AsyncBufReadExt};
use futures::SinkExt;

pub async fn send(path: &str, msg: String) -> Result<()> {
    let mut stream = UnixStream::connect(path).await?;
    let line = format!("{}\n", msg);
    stream.write_all(line.as_bytes()).await?;
    Ok(())
}

async fn handle<S: Sink<String> + Unpin>(mut sink: S, stream: UnixStream) -> Result<()> {
    let mut reader = BufReader::new(stream).lines();
    info!("got connection on unix domain socket");
    while let Some(line) = reader.next_line().await? {
        info!("socket: {:?}", line);
        sink.send(line).await.map_err(|_| anyhow!("sink error"))?;
    }
    Ok(())
}

pub async fn spawn(path: &str, tx: Sender<String>) -> Result<()> {
    let path = Path::new(path);
    if path.exists() {
        fs::remove_file(&path)
            .await
            .context("Failed to remove old socket")?;
    }

    println!("Binding rpc socket: {:?}", path.display());
    let listener = UnixListener::bind(&path)?;

    // TODO: is there a way we can drop this?
    // TODO: this is basically single user embedded, but still
    fs::set_permissions(&path, Permissions::from_mode(0o777))
        .await
        .context("Failed to make socket 0777")?;

    loop {
        let (stream, _addr) = listener.accept().await?;
        let tx = tx.clone();
        tokio::spawn(async move {
            if let Err(e) = handle(tx, stream).await {
                warn!("An error occured; error = {:#}", e);
            }
        });
    }
}
