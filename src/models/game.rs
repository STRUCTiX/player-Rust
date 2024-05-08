use serde::Deserialize;

#[derive(Deserialize)]
pub struct Game {
    pub uid: u32,               // uid of game
    pub tick: u32,              // tick in game
    #[serde(rename="playerCount")]
    pub player_count: u32,      // number of players
    #[serde(rename="remainingPlayers")]
    pub remaining_players: u32, // number of players remaining
    pub player: u32,            // uid of your player
}