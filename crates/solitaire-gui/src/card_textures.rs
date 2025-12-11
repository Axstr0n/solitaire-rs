use egui::{ColorImage, Context, TextureHandle};
use enum_iterator::all;
use solitaire_core::card::{Rank, Suit};
use std::collections::HashMap;

use crate::card_assets;

#[derive(Clone)]
pub struct CardTextures {
    pub cards: HashMap<(Suit, Rank), TextureHandle>,
    pub templates: HashMap<Suit, TextureHandle>,
    pub back: TextureHandle,
}

impl CardTextures {
    /// Load all card textures from embedded bytes
    pub fn load(ctx: &Context) -> Self {
        // Load all 52 cards
        let mut cards = HashMap::new();
        for suit in all::<Suit>() {
            for rank in all::<Rank>() {
                let bytes = card_assets::get_card_bytes(suit, rank);
                let name = format!("{:?}_{:?}", suit, rank);

                match Self::load_from_bytes(ctx, bytes, &name) {
                    Some(tex) => {
                        cards.insert((suit, rank), tex);
                    }
                    None => {
                        eprintln!("Failed to load texture for {:?} of {:?}", rank, suit);
                    }
                }
            }
        }

        // Load suit templates
        let mut templates = HashMap::new();
        for suit in [Suit::Heart, Suit::Spade, Suit::Diamond, Suit::Club] {
            let bytes = card_assets::get_card_template_bytes(suit);
            let name = format!("{:?}", suit);
            match Self::load_from_bytes(ctx, bytes, &name) {
                Some(tex) => {
                    templates.insert(suit, tex);
                }
                None => {
                    eprintln!("Failed to load template texture for {:?}", suit);
                }
            }
        }

        // Load card back
        let back_bytes = card_assets::get_card_back_bytes();
        let back = Self::load_from_bytes(ctx, back_bytes, "card_back")
            .expect("Failed to load card back texture");

        Self {
            cards,
            templates,
            back,
        }
    }

    /// Load a texture from embedded PNG bytes
    fn load_from_bytes(ctx: &Context, bytes: &[u8], name: &str) -> Option<TextureHandle> {
        let img = image::load_from_memory(bytes).ok()?.to_rgba8();
        let size = [img.width() as usize, img.height() as usize];
        let pixels = img.as_flat_samples();

        Some(ctx.load_texture(
            name,
            ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()),
            Default::default(),
        ))
    }

    /// Get texture for a specific card
    pub fn get(&self, suit: Suit, rank: Rank) -> Option<&TextureHandle> {
        self.cards.get(&(suit, rank))
    }

    /// Get texture for a specific card
    pub fn get_template(&self, suit: Suit) -> Option<&TextureHandle> {
        self.templates.get(&suit)
    }

    /// Get texture for a specific card, or return the back if not found
    pub fn get_or_back(&self, suit: Suit, rank: Rank) -> &TextureHandle {
        self.cards.get(&(suit, rank)).unwrap_or(&self.back)
    }

    /// Get the card back texture
    pub fn get_back(&self) -> &TextureHandle {
        &self.back
    }
}
