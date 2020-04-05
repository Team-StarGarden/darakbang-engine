use crate::core::Server;
use actix::prelude::*;
use packet_macro::packet;

mod goto;

pub use goto::*;

#[packet(namespace = "common", handler_target = "Server")]
#[derive(Debug)]
pub enum CommonPacket {
    #[packet(id = "goto")]
    Goto {
        request: goto::GotoPacketRequest,
        response: goto::GotoPacketResponse,
    },
}
