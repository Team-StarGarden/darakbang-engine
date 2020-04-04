use crate::core::{Room, RoomId, Session, UserId};
use std::collections::HashMap;

pub struct Server {
    sessions: HashMap<UserId, Session>,
    rooms: HashMap<RoomId, Room>,
}
