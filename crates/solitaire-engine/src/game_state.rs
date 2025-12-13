use getset::Getters;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use solitaire_core::prelude::*;

use crate::error::GameError;

#[derive(Clone, Serialize, Deserialize, Getters)]
pub struct GameState {
    pub stock: Stock,
    pub waste: Waste,
    pub columns: HashMap<u8, Column>,
    pub foundations: HashMap<u8, Foundation>,
}

impl GameState {
    pub fn new(
        columns: HashMap<u8, Column>,
        stock: Stock,
        waste: Waste,
        foundations: HashMap<u8, Foundation>,
    ) -> Self {
        Self {
            stock,
            waste,
            columns,
            foundations,
        }
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

impl GameState {
    pub fn draw(&mut self) -> Result<(), GameError> {
        let from_id = PileId::Stock;
        let to_id = PileId::Waste;

        // Take card from stock
        let card = {
            let from = self.pile_mut(from_id)?;
            let mut cards = from
                .take_cards(1, Side::Top)
                .map_err(|_| GameError::NoCardToDraw)?;
            cards.pop().unwrap()
        };

        // Place card into waste
        let to = self.pile_mut(to_id)?;
        to.insert_cards(vec![card], Side::Top, Face::Up)
            .map_err(GameError::PileError)?;

        Ok(())
    }

    pub fn recycle(&mut self) -> Result<(), GameError> {
        let from_id = PileId::Waste;
        let to_id = PileId::Stock;

        if self.pile(from_id)?.is_empty() {
            return Err(GameError::NothingToRecycle);
        }
        if !self.pile(to_id)?.is_empty() {
            return Err(GameError::StockNotEmpty);
        }

        // Take all cards from waste
        let cards = {
            let from = self.pile_mut(from_id)?;
            from.take_all(Side::Top)
        };

        // Place cards into stock (automatically face-down)
        {
            let to = self.pile_mut(to_id)?;
            to.insert_cards(cards.clone(), Side::Bottom, Face::Down)
                .map_err(GameError::PileError)?;
        }

        Ok(())
    }

    pub fn move_cards(
        &mut self,
        num_cards: usize,
        from: PileId,
        to: PileId,
    ) -> Result<(), GameError> {
        if from == to {
            return Err(GameError::InvalidMove);
        }
        match (from, to) {
            (PileId::Stock, _) | (_, PileId::Waste) | (_, PileId::Stock) => {
                return Err(GameError::InvalidMove);
            }
            (_, _) => {}
        }

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

        // Flip top card of 'from' if column rules require it
        if let PileId::Column(_) = from {
            let from_pile = self.pile_mut(from)?;
            if let Some(card) = from_pile.peek_mut(Side::Top)
                && *card.face() == Face::Down
            {
                card.flip();
            }
        }

        Ok(())
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
