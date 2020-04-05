use crate::core::{DestroyRoom, RoomId, RoomManager, UserId};
use crate::websocket::WsSession;
use actix::prelude::*;
use std::collections::HashMap;

pub struct Room {
    manager: Addr<RoomManager>,
    id: RoomId,
    capacity: usize,
    player_order: Vec<UserId>,
    player_addr: HashMap<UserId, Addr<WsSession>>,
}

impl Actor for Room {
    type Context = Context<Self>;
}

impl Room {
    pub fn new(
        manager: Addr<RoomManager>,
        id: RoomId,
        capacity: usize,
        owner_id: UserId,
        owner_addr: Addr<WsSession>,
    ) -> Self {
        Self {
            manager,
            id,
            capacity,
            player_order: vec![owner_id],
            player_addr: {
                let mut map = HashMap::new();
                map.insert(owner_id, owner_addr);
                map
            },
        }
    }
}

pub struct JoinRoom {
    pub user_id: UserId,
    pub user_addr: Addr<WsSession>,
}

pub enum JoinRoomError {
    Full,
    AlreadyJoined,
}

impl Message for JoinRoom {
    type Result = Result<(), JoinRoomError>;
}

impl Handler<JoinRoom> for Room {
    type Result = <JoinRoom as Message>::Result;

    fn handle(&mut self, msg: JoinRoom, _ctx: &mut Self::Context) -> Self::Result {
        if self.player_order.len() >= self.capacity {
            Err(JoinRoomError::Full)
        } else if self.player_addr.contains_key(&msg.user_id) {
            Err(JoinRoomError::AlreadyJoined)
        } else {
            self.player_order.push(msg.user_id);
            self.player_addr.insert(msg.user_id, msg.user_addr);
            Ok(())
        }
    }
}

pub struct QuitRoom {
    pub user_id: UserId,
}

impl Message for QuitRoom {
    type Result = ();
}

impl Handler<QuitRoom> for Room {
    type Result = <QuitRoom as Message>::Result;

    fn handle(&mut self, msg: QuitRoom, _ctx: &mut Self::Context) -> Self::Result {
        self.player_order.retain(|id| id != &msg.user_id);
        self.player_addr.remove(&msg.user_id);

        if self.player_order.is_empty() {
            self.manager.do_send(DestroyRoom { room_id: self.id });
        }
    }
}
