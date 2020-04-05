use crate::core::{Session, UserId};
use actix::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct Lobby {
    sessions: HashMap<UserId, Session>,
}

impl Actor for Lobby {
    type Context = Context<Self>;
}
