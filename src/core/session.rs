use crate::core::{RoomId, UserId};
use crate::protocol::PacketServer;
use actix::Recipient;

pub struct Session {
    id: UserId,
    pipe: Recipient<PacketServer>,
    room: Option<RoomId>,
    name: String,
}
