use crate::protocol::PacketResult;
use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct GotoPacketRequest {
    // game_category: Option<GameCategory>,
// room: Option<RoomId>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GotoPacketResponseError {
    IsRoomFull,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GotoPacketResponseOk {}

pub type GotoPacketResponse = PacketResult<GotoPacketResponseOk, GotoPacketResponseError>;
