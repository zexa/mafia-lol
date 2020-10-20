use crate::player::player_status::player_status::PlayerStatus;
use crate::player::player_status::player_status_description::PlayerStatusDescription;

pub struct PlayerStatusManager;

impl PlayerStatusManager {
    pub fn get_status_description(status: &PlayerStatus) -> PlayerStatusDescription {
        match status {
            Alive => {
                PlayerStatusDescription {
                    name: "Alive".to_string(),
                }
            },

            Dead => {
                PlayerStatusDescription {
                    name: "Dead".to_string(),
                }
            },
        }
    }
}

