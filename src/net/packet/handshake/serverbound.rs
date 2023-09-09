use crate::net::connection::Connection;
use crate::net::packet::Packet;

struct Handshake {

}

impl Packet for Handshake {
    fn send(channel: Box<&dyn Connection>) {

    }

    fn id() -> u8 {
        0
    }

    fn handle() {

    }
}
