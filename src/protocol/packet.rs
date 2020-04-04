use crate::protocol::structure::*;
use actix::Message;
use serde::*;
use std::fmt;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PacketResult<OkBody: Serialize, ErrorKind: Serialize> {
    Ok(OkBody),
    Err {
        kind: ErrorKind,
        description: Option<String>,
    },
}

impl<OkBody, ErrorKind> fmt::Debug for PacketResult<OkBody, ErrorKind>
where
    OkBody: Serialize + fmt::Debug,
    ErrorKind: Serialize + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PacketResult::Ok(body) => f.debug_tuple("PacketResult::Ok").field(body).finish(),
            PacketResult::Err { kind, description } => f
                .debug_struct("PacketResult::Err")
                .field("kind", kind)
                .field("description", description)
                .finish(),
        }
    }
}

#[derive(Debug, Serialize)]
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
    type Result = ();
}
