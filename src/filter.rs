use prusti_contracts::*;
use crate::vector_spec::*;
use crate::structs::*;
use crate::option_spec::*;
use crate::result_spec::*;

pub enum FilterProtocol {
    Tcp = 0,
    Udp = 1,
    Sctp = 2,
    Other = 3
}

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum L5FilterPriority {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7
}

struct FilterParameters {
    source_ip: [u8; 4],
    dest_ip: [u8; 4],
    source_port: u16,
    dest_port: u16,
    protocol: FilterProtocol,
    priority: L5FilterPriority
}
