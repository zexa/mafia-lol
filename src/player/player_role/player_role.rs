#[derive(Debug, Clone)]
pub enum PlayerRole {
    Unassigned,
    Admin,

    // Mafia
    Godfather,
    Goon,
    Hooker,

    // Townfolk
    Townie,
    Medic,
    Sherif,
    Vigilante,
    
    // Third-party
    SerialKiller,
    Alien,
}

