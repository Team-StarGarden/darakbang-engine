use crate::protocol::PacketResult;
use crate::websocket::WsSession;
use actix::{Handler, Message, MessageResponse};
use log::info;
use serde::*;

#[derive(Debug, Serialize, Deserialize, Message)]
#[rtype(GotoPacketResponse)]
pub struct GotoPacketRequest {
    // game_category: Option<GameCategory>,
// room: Option<RoomId>,
}

#[derive(Debug, Serialize, Deserialize, MessageResponse)]
#[serde(rename_all = "kebab-case")]
pub enum GotoPacketResponseError {
    IsRoomFull,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, MessageResponse)]
pub struct GotoPacketResponseOk {}

pub type GotoPacketResponse = PacketResult<GotoPacketResponseOk, GotoPacketResponseError>;

impl Handler<GotoPacketRequest> for WsSession {
    type Result = GotoPacketResponse;

    fn handle(&mut self, message: GotoPacketRequest, context: &mut Self::Context) -> Self::Result {
        info!("Test {:?}", message);

        PacketResult::Ok(GotoPacketResponseOk {})
    }
}