use crate::errors::*;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct IOC {
    #[serde(rename="#Type")]
    t: String,
    #[serde(rename="Indicator")]
    indicator: String,
    #[serde(rename="App")]
    app: String,
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<HashSet<String>> {
    let list = fs::read(path)?;
    parse_domain_iocs(&list)
}

fn parse_domain_iocs(buf: &[u8]) -> Result<HashSet<String>> {
    let mut rdr = csv::Reader::from_reader(buf);

    let mut iocs = HashSet::new();
    for result in rdr.deserialize() {
        let ioc: IOC = result?;
        if ioc.t == "domain" {
            debug!("Loaded ioc: {:?}", ioc);
            iocs.insert(ioc.indicator);
        }
    }
    Ok(iocs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_iocs() {
        let csv = br#"#Type,Indicator,App
domain,flushdata.1topspy.com,1TopSpy
domain,webservicesdb.mobiispy.com,Mobiispy
domain,hellospy.com,HelloSpy
domain,mobiispy.com,Mobiispy
domain,1topspy.com,1TopSpy
domain,flushdbd.maxxspy.com,Maxxspy
domain,maxxspy.com,Maxxspy
domain,flushdata2.hellospy.com,HelloSpy
foo,bar,asdf
domain,account.logger.mobi,Easy Logger
domain,97.logger.mobi,Easy Logger
"#;
        use std::iter::FromIterator;
        let iocs = parse_domain_iocs(csv).unwrap();

        let expected = &[
            "account.logger.mobi",
            "mobiispy.com",
            "flushdbd.maxxspy.com",
            "flushdata.1topspy.com",
            "hellospy.com",
            "1topspy.com",
            "flushdata2.hellospy.com",
            "maxxspy.com",
            "97.logger.mobi",
            "webservicesdb.mobiispy.com",
        ];
        let expected = HashSet::from_iter(expected.iter().cloned().map(String::from));
        assert_eq!(iocs, expected);
    }
}
