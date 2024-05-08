use crate::models::{board_action::BoardAction, player_action::PlayerAction};

pub fn decide(board_action: BoardAction) -> PlayerAction {
    PlayerAction {
        src: 0,
        dest: 0,
        amount: 0,
    }
}
