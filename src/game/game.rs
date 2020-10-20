pub struct Game {
    players: Vec<Player>,
    state: GameState,
    day: u8,
    wakeup_order: Vec<PlayerRole>,
}
