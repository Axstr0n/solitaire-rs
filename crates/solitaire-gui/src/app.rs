use egui::{RichText, StrokeKind};
use serde::{Deserialize, Serialize};
use solitaire_core::{
    card::{Card, Face, Suit},
    pile::{PileId, Side},
};
use solitaire_engine::prelude::*;

use crate::{card_textures::CardTextures, layout::*, logger::GuiLogger, ui_element::UiElement};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    #[serde(skip)]
    card_textures: Option<CardTextures>,
    game: Game,
    #[serde(skip)]
    logger: GuiLogger,
    #[serde(skip)]
    side_panel_width: f32,
    #[serde(skip)]
    ui_elements: Vec<UiElement>,
    #[serde(skip)]
    dragging: Option<Dragging>,

    #[cfg(debug_assertions)]
    debug_mode: bool,
}

#[derive(Debug)]
pub struct Dragging {
    from: PileId,
    cards: Vec<Card>,
    offset: egui::Vec2,
}

impl Default for App {
    fn default() -> Self {
        let mut app = Self {
            card_textures: None,
            game: Game::new(Some(1)),
            logger: GuiLogger::default(),
            side_panel_width: 10.0,
            dragging: None,
            ui_elements: vec![],
            debug_mode: false,
        };
        let ui_elements = app.compute_ui_elements();
        app.ui_elements = ui_elements;
        app
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app: Self = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        // Always load textures fresh (can't be serialized anyway)
        app.card_textures = Some(CardTextures::load(&cc.egui_ctx));

        app
    }
    pub fn reset(&mut self) {
        self.game = Game::new(None);
        self.logger.clear();
    }
    pub fn undo(&mut self) {
        self.execute_action(Action::Undo);
    }
}

impl eframe::App for App {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(debug_assertions)]
        {
            if ctx.input(|i| i.key_pressed(egui::Key::D)) {
                self.debug_mode = !self.debug_mode;
            }
            ctx.set_debug_on_hover(self.debug_mode);
        }
        let panel_response = egui::SidePanel::left("left").show(ctx, |ui| {
            ui.label(RichText::new("Sloitaire").heading());
            if ui.button("Reset").clicked() {
                self.reset();
            }
            if ui.button("Undo").clicked() {
                self.undo();
            }
            egui::ScrollArea::vertical()
                .max_height(200.0)
                .show(ui, |ui| {
                    self.logger.render(ui, &mut self.game);
                });
            ui.label(format!("Dragging: {:?}", self.dragging));
            self.display_piles(ui);
            self.display_interactble_elements(ui);
            self.display_actions_results(ui);
        });

        self.side_panel_width = panel_response.response.rect.width();
        self.ui_elements = self.compute_ui_elements();

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_ui_elements(ui);
            self.handle_interactions(ui);
        });
    }
}

// Interactions
impl App {
    fn execute_action(&mut self, action: Action) {
        match self.game.handle_action(action) {
            Ok(msg) => self.logger.add(format!("Success: {}", msg)),
            Err(e) => self.logger.add(format!("Failed: {:?}", e)),
        }
        self.game.update_actions_results();
    }
    fn handle_interactions(&mut self, ui: &mut egui::Ui) {
        // Handle ongoing drag
        if let Some(dragging) = &self.dragging {
            let pointer_released = ui.input(|i| i.pointer.any_released());
            if pointer_released {
                if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                    // Find which pile (if any) the mouse is over
                    let target_pile = self.ui_elements.iter().find_map(|e| match e {
                        UiElement::Card { pile, rect, .. }
                        | UiElement::EmptyPile { pile, rect, .. } => {
                            if rect.contains(pos) {
                                Some(*pile)
                            } else {
                                None
                            }
                        }
                    });

                    if let Some(to_pile) = target_pile {
                        let action = Action::Move {
                            from: dragging.from,
                            to: to_pile,
                            num_cards: dragging.cards.len(),
                        };

                        self.execute_action(action);
                    }
                }

                // Clear dragging state
                self.dragging = None;
            }

            // Dragging in progress → do not handle normal interactions
            return;
        }

        // Handle normal clicks / drag starts
        let mut actions_to_execute = Vec::new();

        for ui_element in &self.ui_elements {
            if !ui_element.interactible() {
                continue;
            }

            match ui_element {
                UiElement::Card {
                    card,
                    pile,
                    index,
                    rect,
                    ..
                } => {
                    let response = match pile {
                        PileId::Stock => ui.allocate_rect(*rect, egui::Sense::click()),
                        _ => ui.allocate_rect(*rect, egui::Sense::click_and_drag()),
                    };

                    // Click actions
                    if response.clicked() && pile == &PileId::Stock {
                        actions_to_execute.push(Action::Draw);
                    }

                    // Drag start
                    if response.drag_started() {
                        let mut cards = match pile {
                            PileId::Column(_) => {
                                if let Ok(pile_ref) = self.game.pile(*pile) {
                                    let num_cards = pile_ref.len().saturating_sub(*index);
                                    pile_ref.peek_cards(num_cards, Side::Top)
                                } else {
                                    // Column does not exist → return empty vec
                                    vec![]
                                }
                            }
                            _ => vec![*card], // single card for Stock/Waste/Foundation
                        };
                        cards.reverse();

                        // Compute drag offset
                        let mouse_pos = ui.input(|i| i.pointer.interact_pos()).unwrap_or_default();
                        let offset = mouse_pos - rect.min.to_vec2();

                        self.dragging = Some(Dragging {
                            from: *pile,
                            cards,
                            offset: offset.to_vec2(),
                        });
                    }
                }

                UiElement::EmptyPile { pile, rect, .. } => {
                    let response = ui.allocate_rect(*rect, egui::Sense::click());

                    if response.clicked() && pile == &PileId::Stock {
                        actions_to_execute.push(Action::Recycle);
                    }
                }
            }
        }
        for action in actions_to_execute {
            self.execute_action(action);
        }
    }
}

/// Render
impl App {
    fn render_ui_elements(&self, ui: &mut egui::Ui) {
        for ui_element in &self.ui_elements {
            match ui_element {
                UiElement::Card {
                    card, rect, pile, ..
                } => {
                    if let Some(dragging) = &self.dragging {
                        // Skip any card that's part of the dragging stack from this pile
                        if *pile == dragging.from && dragging.cards.contains(card) {
                            continue;
                        }
                    }
                    self.render_card(card, *rect, ui);
                }
                UiElement::EmptyPile { pile, rect, .. } => {
                    Self::render_placeholder(*rect, ui);
                    if let PileId::Foundation(i) = pile
                        && let Some(pile) = self.game.foundations.get(i)
                    {
                        self.render_template(pile.suit(), *rect, ui);
                    }
                }
            }
        }
        // If dragging, render the dragging cards at mouse cursor
        if let Some(dragging) = &self.dragging
            && let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos())
        {
            for (i, card) in dragging.cards.iter().enumerate() {
                // Compute stacking offset for multiple cards
                let stack_offset = egui::vec2(0.0, i as f32 * COLUMN_CARDS_SPACING);

                // Position = mouse position minus initial offset + stacking offset
                let pos = mouse_pos - dragging.offset + stack_offset;

                let rect = egui::Rect::from_min_size(pos, egui::Vec2::new(CARD_W, CARD_H));
                self.render_card(card, rect, ui);
            }
        }
    }
    fn render_card(&self, card: &Card, rect: egui::Rect, ui: &mut egui::Ui) {
        if let Some(card_textures) = &self.card_textures {
            let tex_opt = match card.face() {
                Face::Down => Some(card_textures.get_back()),
                Face::Up => card_textures.get(*card.suit(), *card.rank()),
            };
            if let Some(tex) = tex_opt {
                ui.put(
                    rect,
                    egui::Image::new((tex.id(), rect.size())).fit_to_exact_size(rect.size()),
                );
            }
        }
    }
    fn render_template(&self, suit: &Suit, rect: egui::Rect, ui: &mut egui::Ui) {
        if let Some(card_textures) = &self.card_textures
            && let Some(tex) = card_textures.get_template(*suit)
        {
            ui.put(
                rect,
                egui::Image::new((tex.id(), rect.size())).fit_to_exact_size(rect.size()),
            );
        }
    }
    fn render_placeholder(rect: egui::Rect, ui: &mut egui::Ui) {
        ui.painter().rect_stroke(
            rect,
            5.0,
            egui::Stroke::new(1.0, egui::Color32::LIGHT_GRAY),
            StrokeKind::Inside,
        );
    }
}

enum PileLayout {
    Overlap,
    Vertical { spacing: f32 },
}
// Ui elements
impl App {
    /// Create UiElement entries for a pile
    fn push_pile_elements_generic(
        &self,
        ui_elements: &mut Vec<UiElement>,
        pile_id: PileId,
        cards: &[Card], // ordered bottom → top
        base_pos: (f32, f32),
        layout: PileLayout,
    ) {
        let base_rect = self.card_rect(base_pos);

        // Always push empty pile entry first
        ui_elements.push(UiElement::empty_pile(pile_id, base_rect, cards.is_empty()));

        match layout {
            PileLayout::Overlap => {
                for (i, card) in cards.iter().enumerate() {
                    let is_top = i == cards.len().saturating_sub(1);
                    let interactible = is_top;
                    ui_elements.push(UiElement::card(*card, pile_id, i, base_rect, interactible));
                }
            }

            PileLayout::Vertical { spacing } => {
                for (i, card) in cards.iter().enumerate() {
                    let pos = (base_pos.0, base_pos.1 + spacing * i as f32);
                    let rect = self.card_rect(pos);
                    let interactible = card.face() == &Face::Up;
                    ui_elements.push(UiElement::card(*card, pile_id, i, rect, interactible));
                }
            }
        }
    }

    /// Compute all ui elements (cards and empty piles)
    fn compute_ui_elements(&self) -> Vec<UiElement> {
        let mut ui_elements: Vec<UiElement> = Vec::new();

        // Stock, Waste
        for pile_id in [PileId::Stock, PileId::Waste] {
            let pos = match pile_id {
                PileId::Stock => STOCK_POS,
                PileId::Waste => WASTE_POS,
                _ => unreachable!(),
            };

            if let Ok(pile) = self.game.pile(pile_id) {
                let cards = pile.peek_cards(pile.len(), Side::Bottom);
                self.push_pile_elements_generic(
                    &mut ui_elements,
                    pile_id,
                    &cards,
                    pos,
                    PileLayout::Overlap,
                );
            }
        }

        // Foundations
        for id in 0..*self.game.num_foundations() {
            let pile_id = PileId::Foundation(id);
            let pos = FOUNDATION_POS[id as usize];

            if let Ok(pile_ref) = self.game.pile(pile_id) {
                let cards = pile_ref.peek_cards(pile_ref.len(), Side::Bottom);
                self.push_pile_elements_generic(
                    &mut ui_elements,
                    pile_id,
                    &cards,
                    pos,
                    PileLayout::Overlap,
                );
            }
        }

        // Columns
        for id in 0..*self.game.num_columns() {
            let pile_id = PileId::Column(id);
            let pos = COLUMN_POS[id as usize];

            if let Ok(pile_ref) = self.game.pile(pile_id) {
                let cards = pile_ref.peek_all(Side::Bottom);
                self.push_pile_elements_generic(
                    &mut ui_elements,
                    pile_id,
                    &cards,
                    pos,
                    PileLayout::Vertical {
                        spacing: COLUMN_CARDS_SPACING,
                    },
                );
            }
        }

        ui_elements
    }

    fn panel_relative_pos(&self, x: f32, y: f32) -> egui::Pos2 {
        egui::pos2(self.side_panel_width + x, y)
    }

    fn card_rect(&self, position: (f32, f32)) -> egui::Rect {
        egui::Rect::from_min_size(
            self.panel_relative_pos(position.0, position.1),
            egui::vec2(CARD_W, CARD_H),
        )
    }
}

// Debug display
impl App {
    fn display_pile(&self, pile_id: PileId, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(format!("{pile_id:?}"));
            if let PileId::Foundation(i) = pile_id
                && let Some(pile) = self.game.foundations.get(&i)
            {
                ui.label(format!(" {}", pile.suit()));
            }
        });
        if let Ok(pile) = self.game.pile(pile_id) {
            for card in pile.peek_cards(pile.len(), Side::Bottom) {
                ui.label(format!("{card:?}"));
            }
        }
        ui.separator();
    }
    fn display_piles(&self, ui: &mut egui::Ui) {
        ui.separator();
        ui.collapsing("Piles", |ui| {
            egui::ScrollArea::vertical()
                .id_salt("piles_debug")
                .max_height(500.0)
                .show(ui, |ui| {
                    self.display_pile(PileId::Stock, ui);
                    self.display_pile(PileId::Waste, ui);
                    for id in 0..*self.game.num_foundations() {
                        self.display_pile(PileId::Foundation(id), ui);
                    }
                    for id in 0..*self.game.num_columns() {
                        self.display_pile(PileId::Column(id), ui);
                    }
                });
            ui.separator();
        });
    }
    fn display_interactble_elements(&self, ui: &mut egui::Ui) {
        ui.separator();
        ui.collapsing("Interactible elements", |ui| {
            for ele in &self.ui_elements {
                if !ele.interactible() {
                    continue;
                }
                ui.label(format!("{:?}", ele));
            }
        });
    }
    fn display_actions_results(&self, ui: &mut egui::Ui) {
        ui.separator();
        ui.collapsing("ActionsResults", |ui| {
            let actions = self.game.actions_results().data();

            // Separate actions into Ok and Err
            let (ok_actions, err_actions): (Vec<_>, Vec<_>) = actions
                .iter()
                .cloned()
                .partition(|(_, result)| result.is_ok());

            // ScrollArea for Ok actions
            ui.group(|ui| {
                ui.label(format!("Valid Actions ({}):", ok_actions.len()));
                egui::ScrollArea::vertical()
                    .id_salt("valid_actions")
                    .show(ui, |ui| {
                        for (action, _) in ok_actions {
                            ui.horizontal(|ui| {
                                ui.label(format!("{:?}", action));
                                ui.label("Ok");
                            });
                        }
                    });
            });

            ui.separator();

            // ScrollArea for Err actions
            ui.group(|ui| {
                ui.label("Errored Actions:");
                egui::ScrollArea::vertical()
                    .id_salt("errored_actions")
                    .show(ui, |ui| {
                        for (action, _) in err_actions {
                            ui.horizontal(|ui| {
                                ui.label(format!("{:?}", action));
                                ui.label("Err");
                            });
                        }
                    });
            });
        });
    }
}
