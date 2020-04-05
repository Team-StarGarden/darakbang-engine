use crate::websocket::WsSession;
use packet_macro::packet;

mod goto;

pub use goto::*;

#[packet(namespace = "common", handler_target = "WsSession")]
#[derive(Debug)]
pub enum CommonPacket {
    #[packet(id = "goto")]
    Goto {
        request: goto::GotoPacketRequest,
        response: goto::GotoPacketResponse,
    },
}
