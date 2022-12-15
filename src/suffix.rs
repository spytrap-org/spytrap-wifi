use std::collections::HashMap;
use std::iter::FromIterator;

fn split(domain: &str) -> Vec<String> {
    let mut v: Vec<_> = domain.split('.').map(String::from).collect();
    v.reverse();
    v
}

#[derive(Debug)]
pub struct SuffixTree<T> {
    hash: Option<HashMap<T, Box<SuffixTree<T>>>>,
}

impl<T> SuffixTree<T> {
    #[inline]
    pub fn new() -> SuffixTree<T> {
        SuffixTree::default()
    }

    pub fn len(&self) -> usize {
        let mut n = 0;
        if let Some(hash) = &self.hash {
            for v in hash.values() {
                n += v.len();
            }
            n
        } else {
            1
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.hash.is_none()
    }
}

impl<T> Default for SuffixTree<T> {
    fn default() -> SuffixTree<T> {
        SuffixTree {
            hash: Some(HashMap::new()),
        }
    }
}

impl PartialEq for SuffixTree<String> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.hash, &other.hash) {
            (Some(s), Some(o)) => s == o,
            (None, None) => true,
            _ => false,
        }
    }
}

impl FromIterator<String> for SuffixTree<String> {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item=String>,
    {
        let mut s = SuffixTree::new();
        for domain in iter {
            s.insert(&domain);
        }
        s
    }
}

impl SuffixTree<String> {
    pub fn insert(&mut self, domain: &str) {
        let mut s = self;
        for part in split(domain) {
            if let Some(hash) = &mut s.hash {
                if !hash.contains_key(&part) {
                    hash.insert(part.clone(), Box::new(SuffixTree::new()));
                }
                s = hash.get_mut(&part).unwrap();
            } else {
                return;
            }
        }
        s.hash = None;
    }

    pub fn matches(&self, domain: &str) -> bool {
        let mut s = self;
        for part in split(domain) {
            if let Some(hash) = &s.hash {
                if let Some(next) = hash.get(&part) {
                    s = next;
                } else {
                    return false;
                }
            }
        }
        s.hash.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let s = SuffixTree::new();
        let m = s.matches("github.com");
        assert!(!m);
    }

    #[test]
    fn exact() {
        let mut s = SuffixTree::new();
        s.insert("github.com");
        let m = s.matches("github.com");
        assert!(m);
    }

    #[test]
    fn subdomain() {
        let mut s = SuffixTree::new();
        s.insert("github.com");
        let m = s.matches("www.github.com");
        assert!(m);
    }

    #[test]
    fn lots_of_subdomains() {
        let mut s = SuffixTree::new();
        s.insert("github.com");
        let m = s.matches("a.b.c.d.e.f.g.h.i.github.com");
        assert!(m);
    }

    #[test]
    fn not_tld() {
        let mut s = SuffixTree::new();
        s.insert("github.com");
        let m = s.matches("com");
        assert!(!m);
    }

    #[test]
    fn not_other_domain() {
        let mut s = SuffixTree::new();
        s.insert("github.com");
        let m = s.matches("example.com");
        assert!(!m);
    }

    #[test]
    fn not_other_subdomain() {
        let mut s = SuffixTree::new();
        s.insert("foo.example.com");
        let m = s.matches("bar.example.com");
        assert!(!m);
    }

    #[test]
    fn len_0() {
        let s: SuffixTree<()> = SuffixTree::new();
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn len_1() {
        let mut s = SuffixTree::new();
        s.insert("github.com");
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn len_2() {
        let mut s = SuffixTree::new();
        s.insert("github.com");
        s.insert("example.com");
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn len_3() {
        let mut s = SuffixTree::new();
        s.insert("www.github.com");
        s.insert("github.com");
        s.insert("example.com");
        s.insert("www.example.com");
        s.insert("foobar.com");
        assert_eq!(s.len(), 3);
    }
}
