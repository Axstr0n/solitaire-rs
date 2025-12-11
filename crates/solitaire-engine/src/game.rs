use getset::Getters;
use rand::{Rng, SeedableRng, rngs::StdRng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use solitaire_core::prelude::*;
use std::collections::HashMap;

use crate::{
    action::{Action, ActionResult, GameChange},
    error::GameError,
    prelude::ActionsResults,
};

#[derive(Clone, Serialize, Deserialize, Getters)]
pub struct Game {
    pub stock: Stock,
    pub waste: Waste,
    pub columns: HashMap<u8, Column>,
    pub foundations: HashMap<u8, Foundation>,
    action_history: Vec<ActionResult>,
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

        let mut game = Self {
            seed,
            stock,
            waste,
            foundations,
            columns,
            action_history: vec![],
            actions_results: ActionsResults::empty(),
        };
        let ar = ActionsResults::from_game(&game);
        game.actions_results = ar;
        game
    }

    pub fn foundation_ids(&self) -> Vec<PileId> {
        let mut ids: Vec<PileId> = self
            .foundations
            .keys()
            .map(|id| PileId::Foundation(*id))
            .collect();
        ids.sort();
        ids
    }

    pub fn column_ids(&self) -> Vec<PileId> {
        let mut ids: Vec<PileId> = self.columns.keys().map(|id| PileId::Column(*id)).collect();
        ids.sort();
        ids
    }

    pub fn pile(&self, id: PileId) -> Result<&dyn PileBehavior, GameError> {
        match id {
            PileId::Stock => Ok(&self.stock),
            PileId::Waste => Ok(&self.waste),
            PileId::Column(n) => self
                .columns
                .get(&n)
                .map(|c| c as &dyn PileBehavior)
                .ok_or(GameError::ColumnNotExist(n)),
            PileId::Foundation(n) => self
                .foundations
                .get(&n)
                .map(|f| f as &dyn PileBehavior)
                .ok_or(GameError::FoundationNotExist(n)),
        }
    }

    pub fn pile_mut(&mut self, id: PileId) -> Result<&mut dyn PileBehavior, GameError> {
        match id {
            PileId::Stock => Ok(&mut self.stock),
            PileId::Waste => Ok(&mut self.waste),
            PileId::Column(n) => self
                .columns
                .get_mut(&n)
                .map(|c| c as &mut dyn PileBehavior)
                .ok_or(GameError::ColumnNotExist(n)),
            PileId::Foundation(n) => self
                .foundations
                .get_mut(&n)
                .map(|f| f as &mut dyn PileBehavior)
                .ok_or(GameError::FoundationNotExist(n)),
        }
    }
}

impl Game {
    pub fn handle_action(&mut self, action: Action) -> Result<String, GameError> {
        let result = match action {
            Action::Draw => self.draw()?,
            Action::Recycle => self.recycle()?,
            Action::Undo => {
                self.undo()?;
                return Ok("Undid last action".to_string());
            }
            Action::Move {
                num_cards,
                from,
                to,
            } => self.move_cards(num_cards, from, to)?,
            // other actions...
            _ => return Err(GameError::InvalidMove),
        };
        self.action_history.push(result);
        Ok(action.to_string())
    }
    pub fn update_actions_results(&mut self) {
        self.actions_results = ActionsResults::from_game(self);
    }

    pub fn draw(&mut self) -> Result<ActionResult, GameError> {
        let from_id = PileId::Stock;
        let to_id = PileId::Waste;
        let mut changes = Vec::new();

        // Take card from stock
        let card = {
            let from = self.pile_mut(from_id)?;
            let mut cards = from
                .take_cards(1, Side::Top)
                .map_err(|_| GameError::NoCardToDraw)?;
            cards.pop().unwrap()
        };

        changes.push(GameChange::CardsMoved {
            from: from_id,
            to: to_id,
            cards: vec![card],
        });

        // Place card into waste
        let to = self.pile_mut(to_id)?;
        to.insert_cards(vec![card], Side::Top, Face::Up)
            .map_err(GameError::PileError)?;

        Ok(ActionResult {
            action: Action::Draw,
            changes,
        })
    }

    pub fn recycle(&mut self) -> Result<ActionResult, GameError> {
        let from_id = PileId::Waste;
        let to_id = PileId::Stock;

        if self.pile(from_id)?.is_empty() {
            return Err(GameError::NothingToRecycle);
        }
        if !self.pile(to_id)?.is_empty() {
            return Err(GameError::StockNotEmpty);
        }

        let mut changes = Vec::new();

        // Take all cards from waste
        let cards = {
            let from = self.pile_mut(from_id)?;
            from.take_all(Side::Top)
        };

        changes.push(GameChange::CardsMoved {
            from: from_id,
            to: to_id,
            cards: cards.clone(),
        });

        // Place cards into stock (automatically face-down)
        {
            let to = self.pile_mut(to_id)?;
            to.insert_cards(cards.clone(), Side::Bottom, Face::Down)
                .map_err(GameError::PileError)?;
        }

        // Record face changes if needed (Stock sets face-down automatically)
        for (i, card) in cards.iter().enumerate() {
            if *card.face() != Face::Down {
                changes.push(GameChange::CardFlipped {
                    pile: to_id,
                    index: i,
                    old_face: *card.face(),
                    new_face: Face::Down,
                });
            }
        }

        Ok(ActionResult {
            action: Action::Recycle,
            changes,
        })
    }

    pub fn undo(&mut self) -> Result<(), GameError> {
        // Get the last action
        let last_action = self
            .action_history
            .pop()
            .ok_or(GameError::UndoUnavailable)?;

        // Reverse each change in reverse order
        for change in last_action.changes.into_iter().rev() {
            match change {
                GameChange::CardsMoved { from, to, cards } => {
                    // Remove cards from 'to' pile
                    let to_pile = self.pile_mut(to)?;
                    let _removed = to_pile
                        .take_cards(cards.len(), Side::Top)
                        .map_err(GameError::PileError)?;

                    // Place them back into 'from' pile
                    let from_pile = self.pile_mut(from)?;
                    for card in cards {
                        from_pile.raw_insert_card(card, Side::Top, *card.face());
                    }
                }
                GameChange::CardFlipped {
                    pile,
                    index,
                    old_face,
                    new_face: _,
                } => {
                    let pile = self.pile_mut(pile)?;
                    pile.flip_card_at(index, Side::Bottom, old_face);
                }
            }
        }

        Ok(())
    }

    pub fn move_cards(
        &mut self,
        num_cards: usize,
        from: PileId,
        to: PileId,
    ) -> Result<ActionResult, GameError> {
        if from == to {
            return Err(GameError::InvalidMove);
        }
        match (from, to) {
            (PileId::Stock, _) | (_, PileId::Waste) | (_, PileId::Stock) => {
                return Err(GameError::InvalidMove);
            }
            (_, _) => {}
        }

        let mut changes = Vec::new();

        // --- Save top card face of 'from' pile ---
        let old_top_face = {
            let from_pile = self.pile_mut(from)?;
            from_pile.peek(Side::Top).map(|c| *c.face())
        };

        // --- Take cards ---
        let cards = {
            let from_pile = self.pile_mut(from)?;
            let mut crds = from_pile
                .take_cards(num_cards, Side::Top)
                .map_err(GameError::PileError)?;
            crds.reverse();
            crds
        };

        // --- Try inserting into 'to' pile ---
        {
            let dest_pile = self.pile_mut(to)?;
            if let Err(e) = dest_pile.insert_cards(cards.clone(), Side::Top, Face::Up) {
                // Rollback: put cards back
                let from_pile = self.pile_mut(from)?;
                for card in cards {
                    from_pile.raw_insert_card(card, Side::Top, Face::Up);
                }

                // Restore top card face if it was flipped
                if let Some(face) = old_top_face {
                    let from_pile = self.pile_mut(from)?;
                    if let Some(top_card) = from_pile.peek_mut(Side::Top) {
                        top_card.set_face(face);
                    }
                }

                return Err(GameError::PileError(e));
            }
        }

        // Record moved cards
        changes.push(GameChange::CardsMoved {
            from,
            to,
            cards: cards.clone(),
        });

        // Flip top card of 'from' if column rules require it
        if let PileId::Column(_) = from {
            let from_pile = self.pile_mut(from)?;
            let len = from_pile.len();
            if let Some(card) = from_pile.peek_mut(Side::Top)
                && *card.face() == Face::Down
            {
                let old_face = *card.face();
                card.flip();
                changes.push(GameChange::CardFlipped {
                    pile: from,
                    index: len - 1,
                    old_face,
                    new_face: *card.face(),
                });
            }
        }

        // --- Build ActionResult ---
        let action_result = ActionResult {
            action: Action::Move {
                num_cards,
                from,
                to,
            },
            changes,
        };

        Ok(action_result)
    }

    pub fn is_won(&self) -> bool {
        if !self.stock.is_empty() || !self.waste.is_empty() {
            return false;
        }
        for column in self.columns.values() {
            if !column.is_empty() {
                return false;
            }
        }
        true
    }
}
