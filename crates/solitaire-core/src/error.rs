use serde::{Deserialize, Serialize};

use crate::{
    card::{Card, Color, Rank, Suit},
    pile::PileId,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CannotAcceptReason {
    WrongSuit { expected: Suit, found: Suit },
    WrongRank { expected: Rank, found: Rank },
    WrongColor { expected: Color, found: Color },
    Other(String), // fallback for custom rules
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PileError {
    NotEnoughCards {
        pile: PileId,
        requested: usize,
        available: usize,
    },

    CannotAccept {
        pile: PileId,
        cards: Vec<Card>,
        reason: CannotAcceptReason,
    },

    InvalidTakeZero {
        pile: PileId,
    },

    InvalidTakeTooMany {
        pile: PileId,
        requested: usize,
        max: usize,
    },

    InvalidPlaceZero {
        pile: PileId,
    },

    InvalidPlaceTooMany {
        pile: PileId,
        attempted: usize,
        max: usize,
    },
}
