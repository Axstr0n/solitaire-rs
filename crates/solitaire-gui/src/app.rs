use serde::{Deserialize, Serialize};

use crate::{
    card_textures::CardTextures,
    modes::{mode::Mode, user_play::UserPlayMode},
};

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum AppMode {
    #[default]
    UserPlay,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    mode: AppMode,
    user_play_mode: UserPlayMode,

    // Shared resources
    #[serde(skip)]
    card_textures: Option<CardTextures>,
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
}

impl eframe::App for App {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Lazy initialize textures
        if self.user_play_mode.card_textures.is_none()
            && let Some(app_textures) = &self.card_textures
        {
            self.user_play_mode.card_textures = Some(app_textures.clone());
        }
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::global_theme_preference_switch(ui);
                ui.separator();
                ui.heading("Solitaire");
                ui.separator();
                if ui.button("UserPlay").clicked() {
                    self.mode = AppMode::UserPlay;
                }
            });
        });
        match self.mode {
            AppMode::UserPlay => self.user_play_mode.update(),
        }
        egui::CentralPanel::default().show(ctx, |_ui| match self.mode {
            AppMode::UserPlay => self.user_play_mode.render(ctx),
        });
    }
}
