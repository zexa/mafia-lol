#[derive(Debug, Clone)]
pub struct GamePlayer {
    pub name: String,
    //pub status: GamePlayerStatus,
    //pub role: GamePlayerRole,
}

impl GamePlayer {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

