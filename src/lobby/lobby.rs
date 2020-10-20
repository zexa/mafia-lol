use crate::player::player::Player;
use crate::player::player_role::player_role::PlayerRole;
use crate::lobby::lobby_status::LobbyStatus;
use nanoid::nanoid;

#[derive(Debug)]
pub struct Lobby {
    pub id: String,
    pub players: Vec<Player>,
    pub logs: Vec<String>,
    pub status: LobbyStatus,
    pub role_pool: Vec<PlayerRole>,
}

impl Lobby {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}

