use crate::core::{Room, RoomId};
use actix::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct RoomManager {
    rooms: HashMap<RoomId, Room>,
}

impl Actor for RoomManager {
    type Context = Context<Self>;
}
