#[macro_use] extern crate failure;
#[macro_use] extern crate log;


mod conf;
mod errors;

pub use conf::*;
pub use errors::*;

use std::net::Ipv4Addr;
use std::fmt;


#[derive(Debug, Clone)]
pub struct AppTraceRoute{
    pub route_id: u16,

    pub src: Ipv4Addr,
    pub dst: Ipv4Addr,
}

impl AppTraceRoute{
    pub fn new(route_id:u16, src: Ipv4Addr, dst: Ipv4Addr) -> Self{
        AppTraceRoute{route_id, src, dst}
    }
}

impl fmt::Display for AppTraceRoute{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "route[{}]: {} -> {}", self.route_id, self.src, self.dst)
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AppHop{
    pub route_id: u16,
    pub hop: Ipv4Addr,
    pub ttl:u8,
    pub rtt: u16,
}
impl AppHop{
    pub fn new(route_id: u16, hop:Ipv4Addr, ttl:u8, rtt:u16) -> Self{
        AppHop{route_id, hop, ttl, rtt}
    }
}
impl fmt::Display for AppHop{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hop:[{}] {} {}",self.route_id, self.hop, self.ttl)
    }
}