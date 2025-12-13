use getset::Getters;
use serde::{Deserialize, Serialize};
use solitaire_core::pile::PileId;

use crate::{action::Action, error::GameError, game::Game, prelude::GameState};

#[derive(Clone, Serialize, Deserialize, Getters)]
pub struct ActionsResults {
    #[getset(get = "pub")]
    data: Vec<(Action, Result<GameState, GameError>)>,
}
impl ActionsResults {
    pub fn empty() -> Self {
        Self { data: vec![] }
    }
    pub fn from_game(game: &Game) -> Self {
        let mut res = vec![];
        // Draw
        {
            let game_new = game.clone();
            let action = Action::Draw;
            let result = game_new.test_action(action.clone());
            res.push((action, result));
        }
        // Recycle
        {
            let game_new = game.clone();
            let action = Action::Recycle;
            let result = game_new.test_action(action.clone());
            res.push((action, result));
        }

        // Waste to foundations
        {
            for pile_id in game.state.foundation_ids() {
                let game_new = game.clone();
                let action = Action::Move {
                    num_cards: 1,
                    from: PileId::Waste,
                    to: pile_id,
                };
                let result = game_new.test_action(action.clone());
                res.push((action, result));
            }
        }

        // Waste to columns
        {
            for pile_id in game.state.column_ids() {
                let game_new = game.clone();
                let action = Action::Move {
                    num_cards: 1,
                    from: PileId::Waste,
                    to: pile_id,
                };
                let result = game_new.test_action(action.clone());
                res.push((action, result));
            }
        }

        // Foundation to columns
        {
            for f_pile_id in game.state.foundation_ids() {
                for c_pile_id in game.state.column_ids() {
                    let game_new = game.clone();
                    let action = Action::Move {
                        num_cards: 1,
                        from: f_pile_id,
                        to: c_pile_id,
                    };
                    let result = game_new.test_action(action.clone());
                    res.push((action, result));
                }
            }
        }

        // Columns to (foundations, columns)
        {
            for from_pile_id in game.state.column_ids() {
                if let Ok(from_column) = game.state.pile(from_pile_id) {
                    let column_len = from_column.len();

                    // Try all possible stacks
                    for start_index in 0..column_len {
                        let n = column_len - start_index; // number of cards from start_index to top

                        // Skip zero cards
                        if n == 0 {
                            continue;
                        }

                        // --- To Foundations ---
                        for to_pile_id in game.state.foundation_ids() {
                            let game_new = game.clone();
                            let action = Action::Move {
                                num_cards: n,
                                from: from_pile_id,
                                to: to_pile_id,
                            };
                            let result = game_new.test_action(action.clone());
                            res.push((action, result));
                        }

                        // --- To other Columns ---
                        for to_pile_id in game.state.column_ids() {
                            if to_pile_id == from_pile_id {
                                continue; // skip same column
                            }
                            let game_new = game.clone();
                            let action = Action::Move {
                                num_cards: n,
                                from: from_pile_id,
                                to: to_pile_id,
                            };
                            let result = game_new.test_action(action.clone());
                            res.push((action, result));
                        }
                    }
                }
            }
        }

        Self { data: res }
    }
    /// Return only the actions that succeeded
    pub fn all_valid(&self) -> Vec<Action> {
        self.data
            .iter()
            .filter_map(|(action, result)| result.as_ref().ok().map(|_| action.clone()))
            .collect()
    }
}
