use crate::models::{game_state::GameState, player_action::PlayerAction};

pub fn decide(game_state: GameState) -> Vec<PlayerAction> {
    let mut ogermeister: Vec<PlayerAction> = vec![];

    for target in &game_state.bases {
        if target.level != 4 {
            continue;
        }

        if target.player == 0 {
            for oger in &game_state.bases {
                if oger.player != game_state.game.player || oger.population < 20 {
                    continue;
                }
    
                if target.population == 21 {
                    let sub_oger = PlayerAction {
                        src: oger.uid,
                        dest: target.uid,
                        amount: 20,
                    };
        
                    ogermeister.push(sub_oger);
                } else {
                    let sub_oger = PlayerAction {
                        src: oger.uid,
                        dest: target.uid,
                        amount: 1,
                    };

                    ogermeister.push(sub_oger);
                }
    
                break;
            }
        } else if target.population >= 200 {
            for next_target in &game_state.bases {
                if next_target.player != game_state.game.player {
                    let sub_oger = PlayerAction {
                        src: target.uid,
                        dest: next_target.uid,
                        amount: next_target.population * 2  + 4,
                    };

                    ogermeister.push(sub_oger);
                }
            }
        }

        break;
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
