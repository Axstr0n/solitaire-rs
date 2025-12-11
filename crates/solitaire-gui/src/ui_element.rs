use solitaire_core::prelude::*;

#[derive(Debug)]
pub enum UiElement {
    Card {
        card: Card,
        pile: PileId,
        index: usize, // position within pile
        rect: egui::Rect,
        interactible: bool,
    },
    EmptyPile {
        pile: PileId,
        rect: egui::Rect,
        interactible: bool,
    },
}

impl UiElement {
    pub fn card(
        card: Card,
        pile: PileId,
        index: usize,
        rect: egui::Rect,
        interactible: bool,
    ) -> Self {
        UiElement::Card {
            card,
            pile,
            index,
            rect,
            interactible,
        }
    }

    pub fn empty_pile(pile: PileId, rect: egui::Rect, interactible: bool) -> Self {
        UiElement::EmptyPile {
            pile,
            rect,
            interactible,
        }
    }

    pub fn pile(&self) -> PileId {
        match self {
            UiElement::Card { pile, .. } => *pile,
            UiElement::EmptyPile { pile, .. } => *pile,
        }
    }

    pub fn rect(&self) -> egui::Rect {
        match self {
            UiElement::Card { rect, .. } => *rect,
            UiElement::EmptyPile { rect, .. } => *rect,
        }
    }
    pub fn interactible(&self) -> bool {
        match self {
            UiElement::Card { interactible, .. } => *interactible,
            UiElement::EmptyPile { interactible, .. } => *interactible,
        }
    }
}
