use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::{
    card::{Card, Face},
    error::PileError,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize)]
pub enum PileId {
    Stock,
    Waste,
    Column(u8),     // 0..=6
    Foundation(u8), // 0..=3
}
impl PileId {
    pub fn is_valid(&self) -> bool {
        match self {
            PileId::Stock | PileId::Waste => true,
            PileId::Column(i) => *i <= 6,
            PileId::Foundation(i) => *i <= 3,
        }
    }
}
impl std::fmt::Display for PileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PileId::Stock => write!(f, "stock"),
            PileId::Waste => write!(f, "waste"),
            PileId::Column(i) => write!(f, "column {}", i),
            PileId::Foundation(i) => write!(f, "foundation {}", i),
        }
    }
}

pub trait PileBehavior {
    fn id(&self) -> PileId;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn max_take_count(&self) -> usize;

    fn cards(&self) -> &VecDeque<Card>;
    fn cards_mut(&mut self) -> &mut VecDeque<Card>;

    // Raw operations (don't follow rules)
    fn raw_take_card(&mut self, side: Side) -> Option<Card> {
        match side {
            Side::Top => self.cards_mut().pop_back(),
            Side::Bottom => self.cards_mut().pop_front(),
        }
    }
    fn raw_insert_card(&mut self, mut card: Card, side: Side, face: Face) {
        card.set_face(face);
        match side {
            Side::Top => self.cards_mut().push_back(card),
            Side::Bottom => self.cards_mut().push_front(card),
        }
    }

    fn take_card(&mut self, side: Side) -> Result<Card, PileError> {
        let card = match side {
            Side::Top => self.cards_mut().pop_back(),
            Side::Bottom => self.cards_mut().pop_front(),
        };

        card.ok_or(PileError::NotEnoughCards {
            pile: self.id(),
            requested: 1,
            available: 0,
        })
    }
    fn take_cards(&mut self, n: usize, side: Side) -> Result<Vec<Card>, PileError> {
        if n == 0 {
            return Err(PileError::InvalidTakeZero { pile: self.id() });
        }
        let max = self.max_take_count();
        if n > max {
            return Err(PileError::InvalidTakeTooMany {
                pile: self.id(),
                requested: n,
                max,
            });
        }

        let mut cards = Vec::with_capacity(n);
        for _ in 0..n {
            cards.push(self.take_card(side)?);
        }
        Ok(cards)
    }
    fn take_all(&mut self, side: Side) -> Vec<Card> {
        let mut result = Vec::with_capacity(self.len());
        while let Ok(card) = self.take_card(side) {
            result.push(card);
        }
        result
    }

    fn insert_card(&mut self, card: Card, side: Side, face: Face) -> Result<(), PileError>;
    fn insert_cards(&mut self, cards: Vec<Card>, side: Side, face: Face) -> Result<(), PileError> {
        for card in cards {
            self.insert_card(card, side, face)?;
        }
        Ok(())
    }

    // --- Peek ---
    fn peek_index(&self, index: usize) -> Option<Card> {
        self.cards().get(index).cloned()
    }
    fn peek_index_mut(&mut self, index: usize) -> Option<&mut Card> {
        self.cards_mut().get_mut(index)
    }
    fn peek(&self, side: Side) -> Option<Card> {
        match side {
            Side::Top => self.peek_index(self.len().checked_sub(1)?),
            Side::Bottom => self.peek_index(0),
        }
    }
    fn peek_mut(&mut self, side: Side) -> Option<&mut Card> {
        match side {
            Side::Top => self
                .len()
                .checked_sub(1)
                .and_then(|i| self.peek_index_mut(i)),
            Side::Bottom => self.peek_index_mut(0),
        }
    }
    fn peek_cards(&self, n: usize, side: Side) -> Vec<Card> {
        let count = self.len().min(n);
        if count == 0 {
            return Vec::new();
        }

        let iter: Box<dyn Iterator<Item = &Card>> = match side {
            Side::Bottom => Box::new(self.cards().iter().take(count)),
            Side::Top => Box::new(self.cards().iter().rev().take(count)),
        };

        iter.cloned().collect()
    }
    fn peek_all(&self, side: Side) -> Vec<Card> {
        self.peek_cards(self.len(), side)
    }

    fn flip_card_at(&mut self, index: usize, side: Side, face: Face) {
        let idx = match side {
            Side::Bottom => index,
            Side::Top => {
                if let Some(idx) = self.len().checked_sub(index + 1) {
                    idx
                } else {
                    return; // index too large → silently ignore
                }
            }
        };

        if let Some(card) = self.peek_index_mut(idx) {
            card.set_face(face);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    Top,
    Bottom,
}
