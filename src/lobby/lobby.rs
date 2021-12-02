use crate::lobby::LobbyPlayer;
use crate::lobby::LobbyStatus;

#[derive(Debug)]
pub struct Lobby {
    pub id: String,
    pub players: Vec<LobbyPlayer>,
    pub logs: Vec<String>,
    pub status: LobbyStatus,
    //pub role_pool: Vec<PlayerRole>,
}

impl Lobby {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}

