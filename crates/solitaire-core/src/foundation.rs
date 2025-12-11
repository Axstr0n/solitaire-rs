use getset::Getters;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use crate::{
    card::{Card, Face, Rank, Suit},
    error::{CannotAcceptReason, PileError},
    pile::{PileBehavior, PileId, Side},
};

/// Foundation of cards where we take from and add to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Getters)]
pub struct Foundation {
    id: PileId,
    #[getset(get = "pub")]
    suit: Suit,
    cards: VecDeque<Card>, // front = bottom, back = top
}

impl Foundation {
    pub fn new(id: u8, suit: Suit, initial_cards: Vec<Card>) -> Self {
        let mut foundation = Self {
            id: PileId::Foundation(id),
            suit,
            cards: VecDeque::new(),
        };
        for card in initial_cards {
            // Use insert_card with Side::Top and Face::Up
            foundation.insert_card(card, Side::Top, Face::Up).unwrap();
        }
        foundation
    }
}

impl PileBehavior for Foundation {
    fn id(&self) -> PileId {
        self.id
    }

    fn len(&self) -> usize {
        self.cards.len()
    }

    fn max_take_count(&self) -> usize {
        1
    }

    fn cards(&self) -> &VecDeque<Card> {
        &self.cards
    }

    fn cards_mut(&mut self) -> &mut VecDeque<Card> {
        &mut self.cards
    }

    fn insert_card(&mut self, mut card: Card, _side: Side, _face: Face) -> Result<(), PileError> {
        // Only one card at a time
        if !self.cards.is_empty() {
            let top_card = self.cards.back().unwrap();

            // Suit must match foundation
            if *card.suit() != self.suit {
                return Err(PileError::CannotAccept {
                    pile: self.id,
                    cards: vec![card],
                    reason: CannotAcceptReason::WrongSuit {
                        expected: self.suit,
                        found: *card.suit(),
                    },
                });
            }

            // Rank must be next in sequence
            if let Some(next_rank) = top_card.rank().higher() {
                if *card.rank() != next_rank {
                    return Err(PileError::CannotAccept {
                        pile: self.id,
                        cards: vec![card],
                        reason: CannotAcceptReason::WrongRank {
                            expected: next_rank,
                            found: *card.rank(),
                        },
                    });
                }
            } else {
                return Err(PileError::CannotAccept {
                    pile: self.id,
                    cards: vec![card],
                    reason: CannotAcceptReason::Other("Top card has no higher rank".to_string()),
                });
            }
        } else {
            // Suit must match foundation
            if *card.suit() != self.suit {
                return Err(PileError::CannotAccept {
                    pile: self.id,
                    cards: vec![card],
                    reason: CannotAcceptReason::WrongSuit {
                        expected: self.suit,
                        found: *card.suit(),
                    },
                });
            }
            // Empty foundation must start with Ace
            if *card.rank() != Rank::Ace {
                return Err(PileError::CannotAccept {
                    pile: self.id,
                    cards: vec![card],
                    reason: CannotAcceptReason::WrongRank {
                        expected: Rank::Ace,
                        found: *card.rank(),
                    },
                });
            }
        }

        // Insert card face up at top
        card.set_face(Face::Up);
        self.cards.push_back(card);

        Ok(())
    }
}
