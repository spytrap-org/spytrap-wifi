use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    Start(Start),
    Send(Send),
    Sniff(Sniff),
    Stream(Stream),
    Screen(Screen),
    Hotspot(Hotspot),
}

#[derive(Debug, StructOpt)]
pub struct Start {
    #[structopt(short, default_value="hostapd.conf")]
    pub file: String,
    #[structopt(short="i", default_value="en0")]
    pub device: String,
    #[structopt(short="x", default_value="cat")]
    pub screen: String,
    #[structopt(short="S", default_value="foo.sock")]
    pub socket: String,
    #[structopt(short, long, default_value="ioc.yaml")]
    pub rules: String,
}

#[derive(Debug, StructOpt)]
pub struct Send {
    pub value: String,
    #[structopt(short="S", default_value="foo.sock")]
    pub socket: String,
}

#[derive(Debug, StructOpt)]
pub struct Sniff {
    #[structopt(short="i", default_value="en0")]
    pub device: String,
}

#[derive(Debug, StructOpt)]
pub struct Stream {
    #[structopt(short, long, default_value="ioc.yaml")]
    pub rules: String,
}

#[derive(Debug, StructOpt)]
pub struct Screen {
    #[structopt(short="x", default_value="cat")]
    pub screen: String,
}

#[derive(Debug, StructOpt)]
pub struct Hotspot {
    #[structopt(short, default_value="hostapd.conf")]
    pub file: String,
}
