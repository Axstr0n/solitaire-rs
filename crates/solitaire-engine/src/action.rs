use serde::{Deserialize, Serialize};
use solitaire_core::pile::PileId;
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Draw,
    Recycle,
    Move {
        num_cards: usize,
        from: PileId,
        to: PileId,
    },
    Undo,
    Reset,
}
impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match &self {
            Action::Draw => "Drew a card from stock".to_string(),
            Action::Recycle => "Recycled waste into stock".to_string(),
            Action::Move {
                num_cards,
                from,
                to,
            } => {
                format!("Moved {num_cards} card(s) from {from} to {to}")
            }
            Action::Undo => "Undid last action".to_string(),
            Action::Reset => "Reset game".to_string(),
        };
        write!(f, "{string}")
    }
}
