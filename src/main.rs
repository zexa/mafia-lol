mod lobby;
mod game;

use crate::lobby::LobbyManager;
use crate::lobby::LobbyPlayerManager;

fn main() {
    let mut lobby = LobbyManager::create_lobby();

    let player = LobbyPlayerManager::create_player(
        "Alex".to_string()
    );

    LobbyManager::add_player(&mut lobby, player);

    //let mut role_pool: Vec::<PlayerRole> = Vec::<PlayerRole>::new();
    //role_pool.push(PlayerRole::Admin);

    //LobbyManager::set_role_pool(&mut lobby, role_pool);
    LobbyManager::start_game(&mut lobby);

    println!("{:?}", lobby);
}

