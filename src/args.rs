use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    Start(Start),
    Send(Send),
    Sniff(Sniff),
    Stream(Stream),
    Screen(Screen),
    Hotspot(Hotspot),
}

#[derive(Debug, Parser)]
pub struct Start {
    #[clap(short, default_value="hostapd.conf")]
    pub file: String,
    #[clap(short='i', default_value="en0")]
    pub device: String,
    #[clap(short='x', default_value="cat")]
    pub screen: String,
    #[clap(short='S', default_value="foo.sock")]
    pub socket: String,
    #[clap(short, long, default_value="ioc.yaml")]
    pub rules: String,
}

#[derive(Debug, Parser)]
pub struct Send {
    pub value: String,
    #[clap(short='S', default_value="foo.sock")]
    pub socket: String,
}

#[derive(Debug, Parser)]
pub struct Sniff {
    #[clap(short='i', default_value="en0")]
    pub device: String,
}

#[derive(Debug, Parser)]
pub struct Stream {
    #[clap(short, long, default_value="./ioc.yaml")]
    pub rules: String,
}

#[derive(Debug, Parser)]
pub struct Screen {
    #[clap(short='x', default_value="cat")]
    pub screen: String,
}

#[derive(Debug, Parser)]
pub struct Hotspot {
    #[clap(short, default_value="hostapd.conf")]
    pub file: String,
}
