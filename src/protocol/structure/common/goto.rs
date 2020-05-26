use serde::*;

use crate::protocol::PacketResult;

#[derive(Serialize, Deserialize)]
pub struct GotoPacketRequest {
    // game_category: Option<GameCategory>,
// room: Option<RoomId>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GotoPacketResponseError {
    IsRoomFull,
    Unknown,
}

#[derive(Serialize, Deserialize)]
pub struct GotoPacketResponseOk {}

pub type GotoPacketResponse = PacketResult<GotoPacketResponseOk, GotoPacketResponseError>;
