use crate::lobby::LobbyPlayer;
use crate::lobby::LobbyStatus;
use crate::game::GamePlayerRole;

#[derive(Debug)]
pub struct Lobby {
    pub id: String,
    pub players: Vec<LobbyPlayer>,
    pub logs: Vec<String>,
    pub status: LobbyStatus,
    pub role_pool: Vec<GamePlayerRole>,
}

impl Lobby {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}

