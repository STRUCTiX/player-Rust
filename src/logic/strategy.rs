use crate::models::{game_state::GameState, player_action::PlayerAction};

pub fn decide(game_state: GameState) -> Vec<PlayerAction> {
    let mut ogermeister: Vec<PlayerAction> = vec![];

    for oger in &game_state.bases {
        if oger.player != game_state.game.player {
            continue;
        }

        for target in &game_state.bases {
            if target.player == game_state.game.player {
                continue;
            }

            let sub_oger = PlayerAction {
                src: oger.uid,
                dest: target.uid,
                amount: target.population * 2 + 4,
            };

            ogermeister.push(sub_oger);
        }
    }

    ogermeister
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decide_test() {
        let want = vec![PlayerAction::default()];

        let result = decide(GameState::default());

        assert!(want == result)
    }
}
