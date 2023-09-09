pub mod connection;
mod packet;

use std::collections::HashMap;
use crate::file;
use crate::net::connection::Player;

pub static mut SERVERS: Vec<Server> = Vec::new();
pub unsafe fn load() {
    let config = file::configuration::of("servers.yml".to_string(), HashMap::new());
    config.save("servers.yml");
    let input = config.element;
    for s in input.into_iter() {
        SERVERS.push(Server{ name: s.0,ip: s.1.get_int32(), players: Vec::new()})
    }

}

pub struct Server<'a> {
    pub name: String,
    pub ip: i32,
    players: Vec<Player<'a>>
}
impl Server<'_> {
    fn get_players(&self) -> &Vec<Player> {
        &self.players
    }
    async fn add_player(&self, mut player: Player<'_>) {
        player.init().await.expect("Error!");
    }
}
pub unsafe fn find_best() -> &'static mut Server<'static> {
    //TODO: add event
    SERVERS.get_mut(0).unwrap()
}

