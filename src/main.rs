pub mod file;

use std::net::TcpListener;
#[tokio::main]
async fn main(){
    println!("Booting up");
    let config = file::configuration::of("config.yml".to_string());
    let def1 = String::from("0.0.0.0");
    let def2 = String::from("25565");
    let ip: &String = config.get_or_default("ip".to_string(), &def1).get_string();
    let port: &String = config.get_or_default("port".to_string(), &def2).get_string();
    println!("Binding to {}:{}",ip, port);
    let channel = Channel::new(ip, port);
    channel.await.main().await;
}
struct Channel {
    port: TcpListener
}

impl Channel {
    pub async fn new(ip: &str, port: &str) -> Channel {
        Channel{port: TcpListener::bind(format!("{}:{}", ip, port)).unwrap()}
    }
    async fn main(&self) {
        loop {

        }
    }
}
