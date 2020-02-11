#[macro_use] extern crate failure;
#[macro_use] extern crate log;


mod conf;
mod errors;

pub use conf::*;
pub use errors::*;

use std::net::Ipv4Addr;
use std::collections::BTreeSet;
use std::fmt;


#[derive(Debug, Clone)]
pub struct AppTraceRoute{
    // this_ip - mid - dst
    pub src: Ipv4Addr,
    pub dst: Ipv4Addr,
    pub trace: BTreeSet<AppHop>,
    pub pkt_id: u16,

    pub ttl:u8,
}

impl AppTraceRoute{
    pub fn new(src: Ipv4Addr, dst: Ipv4Addr, pkt_id:u16) -> Self{
        AppTraceRoute{src, dst, pkt_id, trace:BTreeSet::<AppHop>::new(), ttl:1u8}
    }
}

impl fmt::Display for AppTraceRoute{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let trace = self.trace.iter()
            .map(|e|format!("{},", e))
            .fold(String::new(), |mut a, e| {a.push_str(&e); a})
            ;
        write!(f, "route[{}]: {} -> [{}] -> {} [id:{}, seq/ttl:{}, discovered hops:{}]", self.pkt_id, self.src, trace, self.dst, self.pkt_id, self.ttl, self.trace.len())
    }
}




#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AppHop{
    pub dst: Ipv4Addr,
    pub ttl:u8,
    pub pkt_id: u16,
    pub hop: Ipv4Addr,
    pub rtt: u16,
}
impl AppHop{
    pub fn new(dst:Ipv4Addr, ttl:u8, pkt_id: u16, hop:Ipv4Addr, rtt:u16) -> Self{
        AppHop{dst, ttl,pkt_id, hop, rtt}
    }
}
impl fmt::Display for AppHop{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.ttl, self.hop)
    }
}