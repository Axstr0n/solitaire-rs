use getset::Getters;
use rand::{Rng, SeedableRng, rngs::StdRng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use solitaire_core::prelude::*;
use std::collections::HashMap;

use crate::{
    action::Action,
    error::GameError,
    prelude::{ActionsResults, GameState},
};

#[derive(Clone, Serialize, Deserialize, Getters)]
pub struct Game {
    pub state: GameState,
    state_history: Vec<GameState>,
    seed: u64,
    #[getset(get = "pub")]
    actions_results: ActionsResults,
}

impl Game {
    pub fn new(seed: Option<u64>) -> Self {
        // Generate a seed if none provided
        let seed = seed.unwrap_or_else(|| {
            let mut rng = rand::rng();
            rng.random::<u64>()
        });

        let mut cards = all_cards();

        // Shuffle using the seed
        let mut rng = StdRng::seed_from_u64(seed);
        cards.shuffle(&mut rng);

        // Columns
        let mut columns = HashMap::new();
        let column_ids = [0, 1, 2, 3, 4, 5, 6];
        for i in column_ids {
            let num_cards = i + 1; // column 0 gets 1 card, column 1 gets 2, etc.

            // Remove the first num_cards from deck
            let mut cards_for_col = Vec::with_capacity(num_cards);
            for _ in 0..num_cards {
                cards_for_col.push(cards.remove(0));
            }

            // Create column and add cards
            let mut column = Column::new(i as u8, vec![]);
            for (j, card) in cards_for_col.iter().enumerate() {
                let face = if j == 0 { Face::Up } else { Face::Down };
                column.raw_insert_card(*card, Side::Bottom, face);
            }

            columns.insert(i as u8, column);
        }

        // Stock
        let cards_for_stock = cards.to_vec();
        let stock = Stock::new(cards_for_stock);

        // Waste
        let waste = Waste::new(vec![]);

        // Foundations
        let mut foundations = HashMap::new();
        let foundation_ids = [0, 1, 2, 3];
        let foundation_suits = [Suit::Heart, Suit::Spade, Suit::Club, Suit::Diamond];
        for (id, suit) in foundation_ids.into_iter().zip(foundation_suits.into_iter()) {
            foundations.insert(id, Foundation::new(id, suit, vec![]));
        }

        let initial_state = GameState::new(columns, stock, waste, foundations);

        let mut game = Self {
            seed,
            state: initial_state,
            state_history: vec![],
            actions_results: ActionsResults::empty(),
        };
        let ar = ActionsResults::from_game(&game);
        game.actions_results = ar;
        game
    }
    pub fn reset(&mut self) {
        *self = Game::new(Some(self.seed));
    }
}

impl Game {
    /// Simulate an action on a clone of the current state, returns the result
    pub fn test_action(&self, action: Action) -> Result<GameState, GameError> {
        let mut state_clone = self.state.clone();

        match action {
            Action::Draw => state_clone.draw()?,
            Action::Recycle => state_clone.recycle()?,
            Action::Move {
                num_cards,
                from,
                to,
            } => state_clone.move_cards(num_cards, from, to)?,
            Action::Undo => return Err(GameError::UndoUnavailable), // cannot simulate undo
            _ => return Err(GameError::InvalidMove),
        }

        Ok(state_clone)
    }
    pub fn handle_action(&mut self, action: Action) -> Result<String, GameError> {
        if let Action::Undo = action {
            if let Some(prev) = self.state_history.pop() {
                self.state = prev;
                return Ok("Undid last action".to_string());
            } else {
                return Err(GameError::UndoUnavailable);
            }
        }

        // TEMP store current state before attempting action
        let temp_state = self.state.clone();

        // Try to apply action
        let result = match action {
            Action::Draw => self.state.draw(),
            Action::Recycle => self.state.recycle(),
            Action::Move {
                num_cards,
                from,
                to,
            } => self.state.move_cards(num_cards, from, to),
            _ => return Err(GameError::InvalidMove),
        };

        match result {
            Ok(_) => {
                // Action succeeded → push previous state
                self.state_history.push(temp_state);

                // Update cached results
                self.actions_results = ActionsResults::from_game(self);

                Ok(action.to_string())
            }
            Err(e) => {
                // Action failed → do not modify history
                Err(e)
            }
        }
    }
}
