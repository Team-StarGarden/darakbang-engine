use crate::core::UserId;
use crate::websocket::WsSession;
use actix::prelude::*;
use std::collections::HashMap;

const MAX_SESSIONS: usize = 8;

#[derive(Default)]
pub struct Lobby {
    sessions: HashMap<UserId, Addr<WsSession>>,
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

pub struct Connect {
    pub user_id: UserId,
    pub user_addr: Addr<WsSession>,
}

pub enum ConnectError {
    Full,
    AlreadyConnected,
}

impl Message for Connect {
    type Result = Result<(), ConnectError>;
}

impl Handler<Connect> for Lobby {
    type Result = <Connect as Message>::Result;

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        if self.sessions.contains_key(&msg.user_id) {
            Err(ConnectError::AlreadyConnected)
        } else if self.sessions.len() >= MAX_SESSIONS {
            Err(ConnectError::Full)
        } else {
            self.sessions.insert(msg.user_id, msg.user_addr);
            Ok(())
        }
    }
}

pub struct Disconnect {
    pub user_id: UserId,
}

impl Message for Disconnect {
    type Result = ();
}

impl Handler<Disconnect> for Lobby {
    type Result = <Disconnect as Message>::Result;

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.user_id);
    }
}
