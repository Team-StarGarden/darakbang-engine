use actix::Message;

mod handler;
mod packet;
pub mod packet_kind;
mod structure;

pub trait Packet: Message<Result = ()> {}
