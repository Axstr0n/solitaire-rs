use eframe::egui;
use solitaire_core::pile::PileId;
use solitaire_engine::prelude::*;

#[derive(Default)]
pub struct GuiLogger {
    pub input: String,
    pub log: Vec<String>,
}

impl GuiLogger {
    pub fn add(&mut self, entry: impl Into<String>) {
        self.log.push(entry.into());
    }
}

impl GuiLogger {
    pub fn clear(&mut self) {
        self.log.clear();
    }
    pub fn render(&mut self, ui: &mut egui::Ui, game: &mut Game) {
        ui.vertical(|ui| {
            ui.label("Command Logger");

            // Log output
            egui::ScrollArea::vertical()
                .max_height(150.0)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for line in &self.log {
                        ui.label(line);
                    }
                });

            ui.horizontal(|ui| {
                // Input field
                let input_resp = ui.text_edit_singleline(&mut self.input);
                let btn_pressed = ui.button("Send").clicked();

                // When user presses Enter
                if (input_resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                    || btn_pressed
                {
                    let cmd = self.input.trim().to_string();
                    if !cmd.is_empty() {
                        self.execute_command(cmd.clone(), game);
                        self.input.clear();
                    }
                }
            });
        });
    }

    fn execute_command(&mut self, cmd: String, game: &mut Game) {
        let parts: Vec<&str> = cmd.split_whitespace().collect();

        if parts.is_empty() {
            return;
        }

        match parts[0] {
            "h" | "help" => {
                self.add(Command::Help.instructions());
            }
            "draw" => {
                let msg = game
                    .handle_action(Action::Draw)
                    .unwrap_or_else(|e| e.to_string());
                self.add(&msg);
            }

            "recycle" => {
                let msg = game
                    .handle_action(Action::Recycle)
                    .unwrap_or_else(|e| e.to_string());
                self.add(&msg);
            }

            "move" => {
                if parts.len() != 4 {
                    self.add(Command::Move.instructions());
                    return;
                }

                let n: usize = match parts[1].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        self.add("Invalid number of cards");
                        return;
                    }
                };

                let from = match parse_pile(parts[2]) {
                    Some(p) => p,
                    None => {
                        self.add("Invalid 'from' pile");
                        return;
                    }
                };

                let to = match parse_pile(parts[3]) {
                    Some(p) => p,
                    None => {
                        self.add("Invalid 'to' pile");
                        return;
                    }
                };

                let action = Action::Move {
                    num_cards: n,
                    from,
                    to,
                };

                let msg = game.handle_action(action).unwrap_or_else(|e| e.to_string());
                self.add(&msg);
            }

            _ => {
                self.add("Unknown command");
            }
        }
    }
}

pub fn parse_pile(token: &str) -> Option<PileId> {
    if token == "s" {
        return Some(PileId::Stock);
    }
    if token == "w" {
        return Some(PileId::Waste);
    }

    if let Some(rest) = token.strip_prefix("c")
        && let Ok(n) = rest.parse::<u8>()
    {
        return Some(PileId::Column(n));
    }

    if let Some(rest) = token.strip_prefix("f")
        && let Ok(n) = rest.parse::<u8>()
    {
        return Some(PileId::Foundation(n));
    }

    None
}

enum Command {
    Help,
    Draw,
    Recycle,
    Move,
}
impl Command {
    fn instructions(&self) -> String {
        match self {
            Self::Help => format!(
                "Commands\n{}\n{}\n{}",
                Command::Draw.instructions(),
                Command::Recycle.instructions(),
                Command::Move.instructions()
            ),
            Self::Draw => "draw - draws card from stock to waste".to_string(),
            Self::Recycle => "recycle - recycles cards from waste into stock".to_string(),
            Self::Move => {
                "move <n> <from> <to> - moves <n> cards from <from> to <to>\ns-stock,w-waste,fx-foundation(x=0..=3),cx-column(x=0..=6)".to_string()
            }
        }
    }
}
