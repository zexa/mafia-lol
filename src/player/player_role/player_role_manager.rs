use crate::player::player_role::player_role::PlayerRole;
use crate::player::player_role::player_role_alignment::PlayerRoleAlignment;
use crate::player::player_role::player_role_description::PlayerRoleDescription;

pub struct PlayerRoleManager;

impl PlayerRoleManager {
    pub fn get_role_description(player_role: &PlayerRole) -> PlayerRoleDescription {
        match player_role {
            Unassigned => {
                PlayerRoleDescription {
                    name: "Unassinged".to_string(),
                    alignment: PlayerRoleAlignment::Unassigned,
                    description: "A spectator".to_string(),
                }
            },
            Admin => {
                PlayerRoleDescription {
                    name: "Admin".to_string(),
                    alignment: PlayerRoleAlignment::Unassigned,
                    description: "The administrator".to_string(),
                }
            },
            Godfather => {
                PlayerRoleDescription {
                    name: "Godfather".to_string(),
                    alignment: PlayerRoleAlignment::Mafia,
                    description: "The mafia leader. Does not die when shot.".to_string(),
                }
            },
            Goon => {
                PlayerRoleDescription {
                    name: "Mafia Goon".to_string(),
                    alignment: PlayerRoleAlignment::Mafia,
                    description: "Slave to the leader. Can shoot people during night. Wakes up together with the godfather.".to_string(),
                }
            },
            Hooker => {
                PlayerRoleDescription {
                    name: "Hooker".to_string(),
                    alignment: PlayerRoleAlignment::Mafia,
                    description: "Can block the role effect of another player. Wakes up separately.".to_string(),
                }
            },
            Townie => {
                PlayerRoleDescription {
                    name: "Townie".to_string(),
                    alignment: PlayerRoleAlignment::Town,
                    description: "Just a useless existence. Does not wake up during the night.".to_string(),
                }
            },
            Medic => {
                PlayerRoleDescription {
                    name: "Medic".to_string(),
                    alignment: PlayerRoleAlignment::Town,
                    description: "Heals a player from a gunwound. Wakes up separately.".to_string(),
                }
            },
            Sherif => {
                PlayerRoleDescription {
                    name: "Sherif".to_string(),
                    alignment: PlayerRoleAlignment::Town,
                    description: "Can figure out weather a player has a gun during the night. Wakes up separately.".to_string(),
                }
            },
            Vigilante => {
                PlayerRoleDescription {
                    name: "Vigilante".to_string(),
                    alignment: PlayerRoleAlignment::Town,
                    description: "Can shoot a person during the night. Only has one bullet. Loses bullet if he shoots a member of the same alignment.".to_string(),
                }
            },
            SerialKiller => {
                PlayerRoleDescription {
                    name: "Serial Killer".to_string(),
                    alignment: PlayerRoleAlignment::SerialKiller,
                    description: "Can kill one player each night. Wins if he's the only player left.".to_string(),
                }
            },
            Alien => {
                PlayerRoleDescription {
                    name: "Alien".to_string(),
                    alignment: PlayerRoleAlignment::Alien,
                    description: "Has two states: activated and unactivated. Starts the game in the unactivated state. Gets activated if shot. Finds out weather it is activated during the night. Wins the game if he is kicked out of town while activated.".to_string(),
                }
            },
        }
    }
}

