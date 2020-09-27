use crate::errors::*;
use serde::Deserialize;

#[derive(Debug, PartialEq)]
pub enum Source {
    DNS,
    TLS,
    HTTP,
}

impl Source {
    pub fn as_str(&self) -> &'static str {
        match self {
            Source::DNS => "dns",
            Source::TLS => "tls",
            Source::HTTP => "http",
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum Pkt {
    Ether((Dummy, IP)),
}

impl Pkt {
    pub fn get_names(&self) -> Vec<(Source, String)> {
        match self {
            Pkt::Ether((_, ip)) => ip.get_names(),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Dummy {}

#[derive(Debug, PartialEq, Deserialize)]
pub enum IP {
    IPv4((Dummy, IPv4)),
}

impl IP {
    #[inline(always)]
    pub fn get_names(&self) -> Vec<(Source, String)> {
        match self {
            IP::IPv4((_, ipv4)) => ipv4.get_names(),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum IPv4 {
    TCP((Dummy, TCP)),
    UDP((Dummy, UDP)),
}

impl IPv4 {
    #[inline(always)]
    pub fn get_names(&self) -> Vec<(Source, String)> {
        match self {
            IPv4::TCP((_, tcp)) => tcp.get_names(),
            IPv4::UDP((_, udp)) => udp.get_names(),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum TCP {
    TLS(TLS),
    HTTP(HTTP),
}

impl TCP {
    #[inline(always)]
    pub fn get_names(&self) -> Vec<(Source, String)> {
        match self {
            TCP::TLS(tls) => tls.get_names(),
            TCP::HTTP(http) => http.get_names(),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum TLS {
    ClientHello(ClientHello),
}

impl TLS {
    #[inline(always)]
    pub fn get_names(&self) -> Vec<(Source, String)> {
        match self {
            TLS::ClientHello(ch) => ch.get_names(),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ClientHello {
    hostname: String,
}

impl ClientHello {
    #[inline(always)]
    pub fn get_names(&self) -> Vec<(Source, String)> {
        vec![(Source::TLS, self.hostname.clone())]
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct HTTP {
    host: String,
}

impl HTTP {
    #[inline(always)]
    pub fn get_names(&self) -> Vec<(Source, String)> {
        vec![(Source::HTTP, self.host.clone())]
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum UDP {
    DNS(DNS),
}

impl UDP {
    #[inline(always)]
    pub fn get_names(&self) -> Vec<(Source, String)> {
        match self {
            UDP::DNS(dns) => dns.get_names(),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum DNS {
    Request(DNSRequest),
}

impl DNS {
    #[inline(always)]
    pub fn get_names(&self) -> Vec<(Source, String)> {
        match self {
            DNS::Request(req) => req.get_names(),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct DNSRequest {
    questions: Vec<(String, String)>,
}

impl DNSRequest {
    #[inline(always)]
    pub fn get_names(&self) -> Vec<(Source, String)> {
        self.questions.iter()
            .map(|(_, name)| (Source::DNS, name.to_string()))
            .collect()
    }
}

pub fn parse(line: &[u8]) -> Result<Pkt> {
    let pkt = serde_json::from_slice(line)?;
    Ok(pkt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dns() {
        let line = br#"{"Ether":[{"source_mac":[10,20,30,40,50,60],"dest_mac":[70,80,90,100,110,120],"ethertype":"IPv4"},{"IPv4":[{"version":4,"ihl":20,"tos":0,"length":79,"id":14838,"flags":0,"fragment_offset":0,"ttl":64,"protocol":"UDP","chksum":1337,"source_addr":"192.168.1.3","dest_addr":"192.168.1.1"},{"UDP":[{"source_port":1337,"dest_port":53,"length":59,"checksum":1337},{"DNS":{"Request":{"questions":[["A","google.com"]]}}}]}]}]}"#;
        let pkt = parse(line).unwrap();
        assert_eq!(
            pkt,
            Pkt::Ether((
                Dummy {},
                IP::IPv4((
                    Dummy {},
                    IPv4::UDP((
                        Dummy {},
                        UDP::DNS(DNS::Request(
                            DNSRequest {
                                questions: vec![("A".to_string(), "google.com".to_string()),],
                            }
                        ))
                    ))
                ))
            ))
        );
    }

    #[test]
    fn extract_dns() {
        let pkt = Pkt::Ether((
            Dummy {},
            IP::IPv4((
                Dummy {},
                IPv4::UDP((
                    Dummy {},
                    UDP::DNS(DNS::Request(
                        DNSRequest {
                            questions: vec![("A".to_string(), "google.com".to_string()),],
                        }
                    ))
                ))
            ))
        ));
        assert_eq!(pkt.get_names(), vec![(Source::DNS, "google.com".to_string())]);
    }

    #[test]
    fn parse_sni() {
        let line = br#"{"Ether":[{"source_mac":[10,20,30,40,50,60],"dest_mac":[70,80,90,100,110,120],"ethertype":"IPv4"},{"IPv4":[{"version":4,"ihl":20,"tos":0,"length":569,"id":2281,"flags":2,"fragment_offset":0,"ttl":64,"protocol":"TCP","chksum":1337,"source_addr":"192.168.1.3","dest_addr":"142.250.102.138"},{"TCP":[{"source_port":1337,"dest_port":443,"sequence_no":1337,"ack_no":1337,"data_offset":8,"reserved":0,"flag_urg":false,"flag_ack":true,"flag_psh":true,"flag_rst":false,"flag_syn":false,"flag_fin":false,"window":504,"checksum":1337,"urgent_pointer":0,"options":null},{"TLS":{"ClientHello":{"version":"tls1.2","session_id":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=","hostname":"google.com"}}}]}]}]}"#;
        let pkt = parse(line).unwrap();
        assert_eq!(
            pkt,
            Pkt::Ether((
                Dummy {},
                IP::IPv4((
                    Dummy {},
                    IPv4::TCP((
                        Dummy {},
                        TCP::TLS(TLS::ClientHello(ClientHello {
                            hostname: "google.com".to_string(),
                        }))
                    ))
                ))
            ))
        );
    }

    #[test]
    fn extract_sni() {
        let pkt = Pkt::Ether((
            Dummy {},
            IP::IPv4((
                Dummy {},
                IPv4::TCP((
                    Dummy {},
                    TCP::TLS(TLS::ClientHello(ClientHello {
                        hostname: "google.com".to_string(),
                    }))
                ))
            ))
        ));
        assert_eq!(pkt.get_names(), vec![(Source::TLS, "google.com".to_string())]);
    }

    #[test]
    fn parse_http() {
        let line = br#"{"Ether":[{"source_mac":[10,20,30,40,50,60],"dest_mac":[70,80,90,100,110,120],"ethertype":"IPv4"},{"IPv4":[{"version":4,"ihl":20,"tos":0,"length":126,"id":19300,"flags":2,"fragment_offset":0,"ttl":64,"protocol":"TCP","chksum":1337,"source_addr":"192.168.1.3","dest_addr":"142.250.102.138"},{"TCP":[{"source_port":1337,"dest_port":80,"sequence_no":1337,"ack_no":1337,"data_offset":8,"reserved":0,"flag_urg":false,"flag_ack":true,"flag_psh":true,"flag_rst":false,"flag_syn":false,"flag_fin":false,"window":504,"checksum":1337,"urgent_pointer":0,"options":null},{"HTTP":{"method":"GET","uri":"/","version":"1.1","host":"google.com","agent":"curl/7.72.0","referer":null,"auth":null,"cookies":null}}]}]}]}"#;
        let pkt = parse(line).unwrap();
        assert_eq!(
            pkt,
            Pkt::Ether((
                Dummy {},
                IP::IPv4((
                    Dummy {},
                    IPv4::TCP((
                        Dummy {},
                        TCP::HTTP(HTTP {
                            host: "google.com".to_string(),
                        })
                    ))
                ))
            ))
        );
    }

    #[test]
    fn extract_http() {
        let pkt = Pkt::Ether((
            Dummy {},
            IP::IPv4((
                Dummy {},
                IPv4::TCP((
                    Dummy {},
                    TCP::HTTP(HTTP {
                        host: "google.com".to_string(),
                    })
                ))
            ))
        ));
        assert_eq!(pkt.get_names(), vec![(Source::HTTP, "google.com".to_string())]);
    }
}
