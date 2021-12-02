use crate::lobby::LobbyPlayer;
use crate::lobby::LobbyPlayerRole;

pub struct LobbyPlayerManager;

impl LobbyPlayerManager {
    pub fn create_player(player_name: String) -> LobbyPlayer {
        LobbyPlayer {
            name: player_name,
            role: LobbyPlayerRole::Player,
        }
    }
}

