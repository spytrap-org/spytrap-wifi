use crate::errors::*;
use serde::Deserialize;
use crate::suffix::SuffixTree;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Deserialize)]
struct Ioc {
    names: Vec<String>,
    #[serde(default)]
    packages: Option<Vec<String>>,
    #[serde(default)]
    certificates: Option<Vec<String>>,
    #[serde(default)]
    websites: Option<Vec<String>>,
    #[serde(default)]
    c2: Option<Vec<String>>,
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<SuffixTree<String>> {
    let list = fs::read(path)?;
    parse_domain_iocs(&list)
}

fn parse_domain_iocs(buf: &[u8]) -> Result<SuffixTree<String>> {
    let mut tree = SuffixTree::new();

    let list = serde_yaml::from_slice::<Vec<Ioc>>(&buf)?;
    for item in list {
        if let Some(websites) = item.websites {
            for domain in websites {
                debug!("Loaded ioc (website): {:?}", domain);
                tree.insert(&domain);
            }
        }

        if let Some(c2) = item.c2 {
            for domain in c2 {
                debug!("Loaded ioc (c2): {:?}", domain);
                tree.insert(&domain);
            }
        }
    }

    Ok(tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_iocs() {
        let csv = br#"---
- names:
    - Foobar
  packages:
    - com.foobar.system
    - com.foobar
  websites:
    - foobar.com
    - checkout.whatever.com
  c2:
    - api.foobar.com
    - foobar.com
    - sneaky.example.com
"#;
        let iocs = parse_domain_iocs(csv).unwrap();

        let expected = &[
            "foobar.com",
            "checkout.whatever.com",
            "api.foobar.com",
            "foobar.com",
            "sneaky.example.com",
        ];
        let expected = expected.into_iter()
            .map(|s| String::from(*s))
            .collect::<SuffixTree<_>>();
        assert_eq!(iocs, expected);
    }
}
