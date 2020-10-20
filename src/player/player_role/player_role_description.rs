use crate::player::player_role::player_role_alignment::PlayerRoleAlignment;

#[derive(Debug, Clone)]
pub struct PlayerRoleDescription {
    pub name: String,
    pub alignment: PlayerRoleAlignment,
    pub description: String,
    pub ability: GameAbility
}

