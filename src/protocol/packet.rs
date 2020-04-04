use crate::protocol::packet_kind::PacketKind;
use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum PacketResult<OkBody: Serialize, ErrorKind: Serialize> {
    Ok(OkBody),
    Err {
        kind: ErrorKind,
        description: Option<String>,
    },
}

#[derive(Serialize)]
pub struct ServerPacket<OkBody: Serialize, ErrorKind: Serialize> {
    ok: bool,
    kind: PacketKind,
    body: PacketResult<OkBody, ErrorKind>,
}
