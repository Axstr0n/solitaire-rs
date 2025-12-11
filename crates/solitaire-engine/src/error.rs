use std::fmt;

use serde::{Deserialize, Serialize};
use solitaire_core::{error::PileError, pile::PileId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameError {
    NoCardToDraw,     // Tried to draw from empty stock
    NothingToRecycle, // Tried to recycle when waste is empty
    StockNotEmpty,    // Tried to recycle when stock is not empty
    InvalidMove,      // Move not allowed by game rules
    UndoUnavailable,  // Tried to undo but no history
    FoundationFull,   // Foundation pile cannot accept more cards
    ColumnNotExist(u8),
    FoundationNotExist(u8),
    PileError(PileError),
    InvalidPile(PileId),
}

impl std::error::Error for GameError {}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::NoCardToDraw => write!(f, "Cannot draw: stock is empty"),
            GameError::NothingToRecycle => write!(f, "Cannot recycle: waste is empty"),
            GameError::StockNotEmpty => write!(f, "Cannot recycle: stock is not empty"),
            GameError::InvalidMove => write!(f, "Invalid move according to the rules"),
            GameError::UndoUnavailable => write!(f, "Nothing to undo"),
            GameError::FoundationFull => write!(f, "Foundation pile is full"),
            GameError::ColumnNotExist(i) => write!(f, "Column {} doesn not exist", i),
            GameError::FoundationNotExist(i) => write!(f, "Foundation {} doesn not exist", i),
            GameError::PileError(e) => write!(f, "{e:?}"),
            GameError::InvalidPile(id) => write!(f, "Pile {id} is not valid"),
        }
    }
}
