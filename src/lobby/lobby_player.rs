use crate::lobby::LobbyPlayerRole;

#[derive(Debug, Clone)]
pub struct LobbyPlayer {
    pub name: String,
    pub role: LobbyPlayerRole,
}

impl LobbyPlayer {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

