use std::collections::HashMap;

use solitaire_core::pile::PileId;
use solitaire_engine::game::Game;

#[derive(Clone)]
pub struct Layout {
    pub pile_positions: HashMap<PileId, (f32, f32)>,
    pub card_width: f32,
    pub card_height: f32,
    pub column_card_spacing: f32,
}

impl Layout {
    pub fn new(game: &Game) -> Self {
        let card_width = 90.0;
        let card_height = 128.0;
        let spacing_x = 20.0;
        let x_start = 20.0;
        let top_y = 20.0;
        let bot_y = 180.0;
        let column_card_spacing = 40.0;

        let mut pile_positions = HashMap::new();

        // Stock + Waste
        pile_positions.insert(PileId::Stock, (x_start, top_y));
        pile_positions.insert(PileId::Waste, (x_start + card_width + spacing_x, top_y));

        // Foundations
        for &id in game.foundations.keys() {
            let x = x_start + (card_width + spacing_x) * (3 + id) as f32; // same spacing logic
            pile_positions.insert(PileId::Foundation(id), (x, top_y));
        }

        // Columns
        for &id in game.columns.keys() {
            let x = x_start + (card_width + spacing_x) * id as f32;
            pile_positions.insert(PileId::Column(id), (x, bot_y));
        }

        Layout {
            pile_positions,
            card_width,
            card_height,
            column_card_spacing,
        }
    }
    pub fn get_position(&self, pile: &PileId) -> Option<(f32, f32)> {
        self.pile_positions.get(pile).copied()
    }
}
