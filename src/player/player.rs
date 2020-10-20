use crate::player::player_status::player_status::PlayerStatus;
use crate::player::player_role::player_role::PlayerRole;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub status: PlayerStatus,
    pub role: PlayerRole,
}

impl Player {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

