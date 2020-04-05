use crate::core::{Room, RoomId, UserId};
use crate::websocket::WsSession;
use actix::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct RoomManager {
    rooms: HashMap<RoomId, Addr<Room>>,
    id_seed: usize,
}

impl Actor for RoomManager {
    type Context = Context<Self>;
}

pub struct GetRoomList {}

impl Message for GetRoomList {
    type Result = Result<Box<[RoomId]>, ()>;
}

impl Handler<GetRoomList> for RoomManager {
    type Result = <GetRoomList as Message>::Result;

    fn handle(&mut self, _msg: GetRoomList, _ctx: &mut Self::Context) -> Self::Result {
        Ok(self.rooms.keys().copied().collect())
    }
}

pub struct GetRoom {
    pub id: RoomId,
}

impl Message for GetRoom {
    type Result = Option<Addr<Room>>;
}

impl Handler<GetRoom> for RoomManager {
    type Result = <GetRoom as Message>::Result;

    fn handle(&mut self, msg: GetRoom, _ctx: &mut Self::Context) -> Self::Result {
        self.rooms.get(&msg.id).cloned()
    }
}

pub struct CreateRoom {
    pub owner_id: UserId,
    pub owner_addr: Addr<WsSession>,
}

impl Message for CreateRoom {
    type Result = Result<Addr<Room>, ()>;
}

impl Handler<CreateRoom> for RoomManager {
    type Result = <CreateRoom as Message>::Result;

    fn handle(
        &mut self,
        msg: CreateRoom,
        ctx: &mut Self::Context,
    ) -> <CreateRoom as Message>::Result {
        if self.rooms.len() >= 1 {
            Err(())
        } else {
            let room_id = self.id_seed;
            self.id_seed += 1;
            let new_room =
                Room::new(ctx.address(), room_id, 8, msg.owner_id, msg.owner_addr).start();
            self.rooms.insert(room_id, new_room.clone());
            Ok(new_room)
        }
    }
}

pub struct DestroyRoom {
    pub room_id: RoomId,
}

impl Message for DestroyRoom {
    type Result = ();
}

impl Handler<DestroyRoom> for RoomManager {
    type Result = <DestroyRoom as Message>::Result;

    fn handle(&mut self, msg: DestroyRoom, ctx: &mut Self::Context) -> Self::Result {
        self.rooms.remove(&msg.room_id);
    }
}
