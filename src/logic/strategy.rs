use crate::models::{game_state::GameState, player_action::PlayerAction};
use crate::models::position::Position;

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

            let mut sub_oger = PlayerAction {
                src: oger.uid,
                dest: target.uid,
                amount: target.population * 2 + 4 + std::cmp::max(0, distance(&target.position, &oger.position) - game_state.config.paths.grace_period) * game_state.config.paths.death_rate,
            };

            if target.player == 0 {
                sub_oger.amount += 1;
            }

            ogermeister.push(sub_oger);
        }

        for target in &game_state.bases {
            if target.player == game_state.game.player && distance(&target.position, &oger.position) <= game_state.config.paths.grace_period {
                let sub_oger = PlayerAction {
                    src: oger.uid,
                    dest: target.uid,
                    amount: 3,
                };
    
                ogermeister.push(sub_oger);
            }
        }
    }

    ogermeister
}

fn distance(a: &Position, b: &Position) -> u32 {
    (((a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)) as f64).sqrt() as u32
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
