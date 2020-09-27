use env_logger::{self, Env};
use spytrap::errors::*;
use spytrap::json;
use spytrap::ioc;
use std::collections::HashSet;
use std::io::{self, BufRead};

// this function must not error or panic
fn process(line: &[u8], iocs: &HashSet<String>) {
    if let Ok(pkt) = json::parse(line) {
        let names = pkt.get_names();
        for (src, name) in names {
            if iocs.contains(&name) {
                warn!("detected({}): {:?}", src.as_str(), name);
            } else {
                debug!("observed({}): {:?}", src.as_str(), name);
            }
        }
    }
}

fn main() -> Result<()> {
    env_logger::init_from_env(Env::default()
        .default_filter_or("spytrap=info"));

    let iocs = ioc::load("network.csv")
        .context("Failed to load iocs")?;
    info!("Loaded {} known IOCs", iocs.len());

    let mut line = Vec::new();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    loop {
        // TODO: skip utf8 decode
        stdin.read_until(b'\n', &mut line)?;
        if line.is_empty() {
            break;
        }
        process(&line, &iocs);
        line.clear();
    }

    Ok(())
}
