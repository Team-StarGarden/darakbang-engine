use actix::Message;

pub mod common;

pub trait Packet: Message<Result = ()> {}
