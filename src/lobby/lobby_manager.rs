use nanoid::nanoid;
use crate::player::player::Player;
use crate::player::player_role::player_role::PlayerRole;
use crate::player::player_role::player_role_manager::PlayerRoleManager;
use crate::lobby::lobby::Lobby;
use crate::lobby::lobby_status::LobbyStatus;

pub struct LobbyManager;

impl LobbyManager {
    pub fn create_lobby() -> Lobby {
        let mut lobby = Lobby {
            id: nanoid!(5),
            players: Vec::<Player>::new(),
            logs: Vec::<String>::new(),
            status: LobbyStatus::Inactive,
            role_pool: Vec::<PlayerRole>::new(),
        };
        lobby.logs.push(format!("Lobby was created with id \"{}\"", lobby.get_id()));

        lobby
    }

    pub fn add_player(lobby: &mut Lobby, player: Player) -> &Lobby {
        lobby.players.push(player.clone());
        lobby.logs.push(format!("Player \"{}\" joined the lobby", player.get_name()));

        lobby
    }

    pub fn set_role_pool(lobby: &mut Lobby, role_pool: Vec<PlayerRole>) -> &Lobby {
        lobby.role_pool = role_pool.clone();
        lobby.logs.push(format!(
            "Lobby role pool set to \"{}\"", 
            role_pool
                .into_iter()
                .map(|role| PlayerRoleManager::get_role_description(&role).name)
                .collect::<Vec<String>>()
                .join(", ")
        ));

        lobby
    }

    pub fn start_game(lobby: &mut Lobby) -> &Lobby {
        lobby.status = LobbyStatus::Active;
        lobby.logs.push("Game has started!".to_string());

        lobby
    }
}

