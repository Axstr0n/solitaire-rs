use serde::{Deserialize, Serialize};

/// Trait representing a generic tool with update and rendering capabilities.
pub trait Mode: Default + Serialize + for<'de> Deserialize<'de> {
    /// Update the tool's internal state.
    fn update(&mut self);

    /// Render UI content of the tool.
    fn render(&mut self, ctx: &egui::Context);
}
