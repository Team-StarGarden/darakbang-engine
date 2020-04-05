use crate::core::UserId;
use crate::websocket::WsSession;
use actix::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct Lobby {
    sessions: HashMap<UserId, WsSession>,
}

impl Actor for Lobby {
    type Context = Context<Self>;
}
