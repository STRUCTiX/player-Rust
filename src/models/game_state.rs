use serde::Deserialize;
use crate::models::base::Base;
use crate::models::board_action::BoardAction;
use crate::models::game::Game;
use crate::models::game_config::GameConfig;

#[derive(Deserialize)]
pub struct GameState {
    pub actions: Vec<BoardAction>, // list of all actions in progress
    pub bases: Vec<Base>,          // list of all bases
    pub config: GameConfig,        // settings for this game
    pub game: Game,                // information about the game
}