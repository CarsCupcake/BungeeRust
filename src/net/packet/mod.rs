mod handshake;

use crate::net::connection::Connection;

pub enum Direcion {
    In,
    Out
}
pub enum PacketType {
    Play,
    Handshake,
    Login,
    Status,
}

pub struct  PacketParser {

}
pub trait Packet {
    fn send(channel: Box<&dyn Connection>);
    fn id() -> u8;
    fn handle();
}