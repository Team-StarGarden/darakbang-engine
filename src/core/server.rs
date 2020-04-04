use crate::core::{Room, RoomId, Session, UserId};
use actix::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct Server {
    sessions: HashMap<UserId, Session>,
    rooms: HashMap<RoomId, Room>,
}

impl Actor for Server {
    type Context = Context<Self>;
}
