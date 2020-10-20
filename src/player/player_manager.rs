use crate::player::player::Player;
use crate::player::player_status::player_status::PlayerStatus;
use crate::player::player_role::player_role::PlayerRole;

pub struct PlayerManager;

impl PlayerManager {
    pub fn create_player(player_name: String) -> Player {
        Player {
            name: player_name,
            status: PlayerStatus::Dead,
            role: PlayerRole::Unassigned,
        }
    }
}

