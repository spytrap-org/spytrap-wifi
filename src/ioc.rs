use crate::errors::*;
use crate::suffix::SuffixTree;
use std::fs;
use std::path::Path;

pub fn load<P: AsRef<Path>>(path: P) -> Result<SuffixTree<String>> {
    let list = fs::read(path)?;
    parse_domain_iocs(&list)
}

fn parse_domain_iocs(buf: &[u8]) -> Result<SuffixTree<String>> {
    let mut tree = SuffixTree::new();
    let list = stalkerware_indicators::parse_from_buf(buf)?;

    for item in list {
        for domain in item.websites {
            debug!("Loaded ioc (website): {:?}", domain);
            tree.insert(&domain);
        }

        for domain in item.c2.domains {
            debug!("Loaded ioc (c2): {:?}", domain);
            tree.insert(&domain);
        }

        // TODO: ip addresses are not matched yet
    }

    Ok(tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_iocs() {
        let buf = br#"---
- name: OwnSpy
  names:
  - OwnSpy
  - WebDetetive
  packages:
  - com.ownspy.android
  - org.system.kernel
  certificates:
  - CA5304E94F4BC97DA9D147E76858DBF70AB8B4E6
  - 14A071616D4BC37F08BE865D375101F4C963777A
  websites:
  - mobileinnova.net
  - webdetetive.com.br
  c2:
    ips: []
    domains:
    - 6287970dd9.era3000.com
    - user.ownspy.es
"#;
        let iocs = parse_domain_iocs(buf).unwrap();

        let expected = &[
            "mobileinnova.net",
            "webdetetive.com.br",
            "6287970dd9.era3000.com",
            "user.ownspy.es",
        ];
        let expected = expected.into_iter()
            .map(|s| String::from(*s))
            .collect::<SuffixTree<_>>();
        assert_eq!(iocs, expected);
    }
}
