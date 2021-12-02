#[derive(Debug, Clone)]
pub enum GamePlayerRole {
    Unassigned, // Specatator
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

