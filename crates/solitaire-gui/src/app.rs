use serde::{Deserialize, Serialize};

use crate::{
    app_stats::AppStats,
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
    stats: AppStats,

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

        self.stats.update_frame();

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::global_theme_preference_switch(ui);
                ui.separator();
                ui.heading("Solitaire");
                ui.separator();
                if ui.button("UserPlay").clicked() {
                    self.mode = AppMode::UserPlay;
                }
                ui.separator();
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    self.stats_ui(ui);
                });
            });
        });
        match self.mode {
            AppMode::UserPlay => self.user_play_mode.update(),
        }
        egui::CentralPanel::default().show(ctx, |_ui| match self.mode {
            AppMode::UserPlay => self.user_play_mode.render(ctx),
        });

        ctx.request_repaint();
    }
}
// Stats
impl App {
    fn stats_ui(&self, ui: &mut egui::Ui) {
        ui.menu_button("Stats", |ui| {
            ui.heading("FPS Monitor");
            ui.separator();

            ui.add_space(10.0);

            // Current FPS
            ui.horizontal(|ui| {
                ui.label("Current FPS:");
                ui.colored_label(
                    if self.stats.fps > 55.0 {
                        egui::Color32::GREEN
                    } else if self.stats.fps > 30.0 {
                        egui::Color32::YELLOW
                    } else {
                        egui::Color32::RED
                    },
                    format!("{:.1}", self.stats.fps),
                );
            });

            ui.add_space(5.0);

            // Frame time
            ui.horizontal(|ui| {
                ui.label("Frame Time:");
                ui.label(format!("{:.2} ms", self.stats.avg_frame_time * 1000.0));
            });

            ui.add_space(5.0);

            // Min/Max FPS
            ui.horizontal(|ui| {
                ui.label("Min/Max:");
                ui.label(format!(
                    "{:.1} / {:.1} FPS",
                    self.stats.min_fps, self.stats.max_fps
                ));
            });
            ui.separator();
        });
    }
}
