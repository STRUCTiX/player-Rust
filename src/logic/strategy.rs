use crate::models::{board_action::BoardAction, player_action::PlayerAction};

pub fn decide(board_action: BoardAction) -> PlayerAction {
    // TODO: place your player logic here.
    PlayerAction {
        src: 0,
        dest: 0,
        amount: 0,
    }
}
