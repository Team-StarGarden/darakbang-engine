use actix::dev::{MessageResponse, ResponseChannel};
use actix::prelude::*;
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

impl<OkBody, ErrorKind> PacketResult<OkBody, ErrorKind>
where
    OkBody: Serialize,
    ErrorKind: Serialize,
{
    pub fn is_ok(&self) -> bool {
        matches!(self, &PacketResult::Ok(_))
    }
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

impl<A, M, OkBody, ErrorKind> MessageResponse<A, M> for PacketResult<OkBody, ErrorKind>
where
    A: Actor,
    M: Message<Result = PacketResult<OkBody, ErrorKind>>,
    OkBody: Serialize + fmt::Debug + 'static,
    ErrorKind: Serialize + fmt::Debug + 'static,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}
