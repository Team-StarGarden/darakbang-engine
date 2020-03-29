use crate::protocol::structure::*;
use actix::Message;
use serde::*;
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PacketResult<OkBody: Serialize, ErrorKind: Serialize> {
    Ok(OkBody),
    Err {
        kind: ErrorKind,
        description: Option<String>,
    },
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum PacketServer {
    Common(CommonPacketServer),
}

impl Message for PacketServer {
    type Result = ();
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum PacketClient {
    Common(CommonPacketClient),
}

impl Message for PacketClient {
    type Result = ();
}
