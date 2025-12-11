use std::collections::VecDeque;

use crate::{
    card::{Card, Face},
    error::PileError,
    pile::{PileBehavior, PileId, Side},
};
use serde::{Deserialize, Serialize};

/// Waste of cards where we take from.
#[derive(Clone, Deserialize, Serialize)]
pub struct Waste {
    id: PileId,
    cards: VecDeque<Card>, // front = bottom, back = top
}

impl Waste {
    pub fn new(cards: Vec<Card>) -> Self {
        let mut waste = Self {
            id: PileId::Waste,
            cards: VecDeque::new(),
        };
        for card in cards {
            waste.insert_card(card, Side::Top, Face::Up).unwrap();
        }
        waste
    }
}

impl PileBehavior for Waste {
    fn id(&self) -> PileId {
        self.id
    }

    fn len(&self) -> usize {
        self.cards.len()
    }

    fn max_take_count(&self) -> usize {
        1 // only one card can be taken at a time
    }

    fn cards(&self) -> &VecDeque<Card> {
        &self.cards
    }

    fn cards_mut(&mut self) -> &mut VecDeque<Card> {
        &mut self.cards
    }

    fn insert_card(&mut self, mut card: Card, _side: Side, _face: Face) -> Result<(), PileError> {
        card.set_face(Face::Up); // Waste cards are always face up
        self.cards.push_back(card); // insert on top/back
        Ok(())
    }
}
