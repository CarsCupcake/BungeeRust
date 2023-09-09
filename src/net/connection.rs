use std::io::Read;
use std::net::{TcpStream};
use anyhow::Result;
use crate::net;
use crate::net::connection::Stage::Off;
use crate::net::Server;

pub trait Connection {
    fn get_channel(&self) -> &Channel;
}

pub struct Channel {
    stream: TcpStream,
    stage: Stage
}
impl Channel {
    pub async fn listen(&mut self) -> Result<()> {
        loop {
            let mut buf: [u8; 1] = [0; 1];
            let result = self.stream.read(&mut buf);
            if result.is_err() {
                println!("Err!");
                continue
            }
            if result.unwrap() == 0 {
                println!("Connection closed!");
                return Ok(())
            }

        }
    }
}

pub(crate) struct Player<'a> {
    channel: Channel,
    server: &'a Server<'a>
}
impl Player<'_> {
    pub async unsafe fn handle(input: TcpStream) -> Result<()> {
        println!("{:?}", input);
        let channel = Channel{stage: Off, stream: input};
        let server = net::find_best();
        let player = Player{channel, server};
        server.add_player(player).await;
        Ok(())

    }
    //TODO: player stuff
    pub async fn init(&mut self) -> Result<()>{
        self.channel.listen().await
    }
}
impl Connection for Player<'_> {
    fn get_channel(&self) -> &Channel {
        &self.channel
    }

}

pub enum Stage {
    Off,
    Handshake,
    Login,
    Play
}