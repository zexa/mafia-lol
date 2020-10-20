extern crate nanoid;

mod player;
mod lobby;

use crate::lobby::lobby_manager::LobbyManager;
use crate::player::player_manager::PlayerManager;
use crate::player::player_role::player_role::PlayerRole;

fn main() {
    let mut lobby = LobbyManager::create_lobby();

    let mut player = PlayerManager::create_player(
        "Alex".to_string()
    );

    LobbyManager::add_player(&mut lobby, player);

    let mut role_pool: Vec::<PlayerRole> = Vec::<PlayerRole>::new();
    role_pool.push(PlayerRole::Admin);

    LobbyManager::set_role_pool(&mut lobby, role_pool);
    LobbyManager::start_game(&mut lobby);

    println!("{:?}", lobby);
}

