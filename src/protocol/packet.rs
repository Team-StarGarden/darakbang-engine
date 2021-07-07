use crate::protocol::*;
use crate::websocket::WsSession;
use actix::{Handler, Message, MessageResponse};
use serde::*;

#[derive(Debug, Serialize, MessageResponse)]
#[serde(untagged)]
pub enum PacketServer {
    Common(CommonPacketServer),
}

impl Message for PacketServer {
    type Result = ();
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PacketClient {
    Common(CommonPacketClient),
}

impl Message for PacketClient {
    type Result = PacketServer;
}

impl Handler<PacketClient> for WsSession {
    type Result = PacketServer;

    fn handle(&mut self, msg: PacketClient, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            PacketClient::Common(common) => PacketServer::Common(self.handle(common, ctx)),
        }
    }
}
