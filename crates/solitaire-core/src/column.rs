use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::{
    card::{Card, Face, Rank},
    error::{CannotAcceptReason, PileError},
    pile::{PileBehavior, PileId, Side},
};

/// Column of cards.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Column {
    id: PileId,
    cards: VecDeque<Card>, // front = bottom, back = top
}

impl Column {
    pub fn new(id: u8, cards: Vec<Card>) -> Self {
        let mut col = Self {
            id: PileId::Column(id),
            cards: VecDeque::new(),
        };
        for (i, card) in cards.iter().enumerate() {
            // last card face up, rest face down
            let face = if i == cards.len() - 1 {
                Face::Up
            } else {
                Face::Down
            };
            col.insert_card(*card, Side::Top, face).unwrap();
        }
        col
    }
}

impl PileBehavior for Column {
    fn id(&self) -> PileId {
        self.id
    }

    fn len(&self) -> usize {
        self.cards.len()
    }

    fn cards(&self) -> &VecDeque<Card> {
        &self.cards
    }

    fn cards_mut(&mut self) -> &mut VecDeque<Card> {
        &mut self.cards
    }

    fn max_take_count(&self) -> usize {
        let mut n = 0;
        for c in self.peek_all(Side::Top) {
            if *c.face() == Face::Up {
                n += 1;
            } else {
                break;
            }
        }
        n
    }

    fn insert_card(&mut self, card: Card, _side: Side, face: Face) -> Result<(), PileError> {
        // Rules enforcement
        if let Some(top) = self.cards.back() {
            // must be lower rank
            if let Some(expected_rank) = top.rank().lower() {
                if *card.rank() != expected_rank {
                    return Err(PileError::CannotAccept {
                        pile: self.id,
                        cards: vec![card],
                        reason: CannotAcceptReason::WrongRank {
                            expected: expected_rank,
                            found: *card.rank(),
                        },
                    });
                }
            } else {
                return Err(PileError::CannotAccept {
                    pile: self.id,
                    cards: vec![card],
                    reason: CannotAcceptReason::Other("Top card has no lower rank".into()),
                });
            }

            // must be alternating color
            if card.color() == top.color() {
                return Err(PileError::CannotAccept {
                    pile: self.id,
                    cards: vec![card],
                    reason: CannotAcceptReason::WrongColor {
                        expected: top.color().opposite(),
                        found: card.color(),
                    },
                });
            }
        } else {
            // empty column: must be King
            if *card.rank() != Rank::King {
                return Err(PileError::CannotAccept {
                    pile: self.id,
                    cards: vec![card],
                    reason: CannotAcceptReason::WrongRank {
                        expected: Rank::King,
                        found: *card.rank(),
                    },
                });
            }
        }

        // Insert on top with given face
        let mut c = card;
        c.set_face(face);
        self.cards.push_back(c);
        Ok(())
    }
}
