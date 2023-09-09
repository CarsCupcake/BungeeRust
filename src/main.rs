pub mod file;
pub mod net;
mod util;

use std::collections::HashMap;
use std::net::TcpListener;
use tokio::spawn;
use crate::file::configuration::Serializable;
use crate::net::connection::Player;
use anyhow::Result;

#[tokio::main]
async fn main() {
    println!("Booting up");
    let def1 = String::from("0.0.0.0");
    let def2 = String::from("25565");
    let mut map: HashMap<String, Box<dyn Serializable>> = HashMap::new();
    map.insert("ip".to_string(), Box::new(def1));
    map.insert("port".to_string(), Box::new(def2));
    let config = file::configuration::of("config.yml".to_string(), map);
    config.save("./config.yml");
    let ip = config.get("ip".to_string()).unwrap().get_string();
    let port = config.get("port".to_string()).unwrap().get_string();
    unsafe { net::load(); }
    println!("Binding to {}:{}", ip, port);
    let channel = Channel::new(ip, port).await;
    unsafe { channel.main().await.expect("Error main channel!"); }
}

pub struct Channel {
    port: TcpListener,
}

impl Channel {
    pub async fn new(ip: &str, port: &str) -> Channel {
        Channel { port: TcpListener::bind(format!("{}:{}", ip, port)).unwrap() }
    }
    async unsafe fn main(&self) -> Result<()> {
        loop {
            let (steam, _) = self.port.accept().unwrap();
            spawn(Player::handle(steam));
        }
    }
}
