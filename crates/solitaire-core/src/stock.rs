use std::collections::VecDeque;

use crate::{
    card::{Card, Face},
    error::PileError,
    pile::{PileBehavior, PileId, Side},
};
use serde::{Deserialize, Serialize};

/// Stock of cards where we draw from.
#[derive(Clone, Deserialize, Serialize)]
pub struct Stock {
    id: PileId,
    cards: VecDeque<Card>, // front = bottom, back = top
}

impl Stock {
    pub fn new(cards: Vec<Card>) -> Self {
        let mut stock = Self {
            id: PileId::Stock,
            cards: VecDeque::new(),
        };
        for card in cards {
            // insert at top face down
            stock.insert_card(card, Side::Top, Face::Down).unwrap();
        }
        stock
    }
}

impl PileBehavior for Stock {
    fn id(&self) -> PileId {
        self.id
    }

    fn len(&self) -> usize {
        self.cards.len()
    }

    fn max_take_count(&self) -> usize {
        1 // can only take one card at a time
    }

    fn cards(&self) -> &VecDeque<Card> {
        &self.cards
    }

    fn cards_mut(&mut self) -> &mut VecDeque<Card> {
        &mut self.cards
    }

    fn insert_card(&mut self, mut card: Card, _side: Side, _face: Face) -> Result<(), PileError> {
        // Always face down in stock
        card.set_face(Face::Down);
        self.cards.push_back(card); // push to top/back
        Ok(())
    }
}
