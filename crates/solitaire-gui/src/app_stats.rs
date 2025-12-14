#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use web_time::Instant;

#[derive(Deserialize, Serialize)]
pub struct AppStats {
    #[serde(skip, default = "Instant::now")]
    last_frame: Instant,
    frame_times: Vec<f32>,
    pub fps: f32,
    pub min_fps: f32,
    pub max_fps: f32,
    pub avg_frame_time: f32,
}
impl Default for AppStats {
    fn default() -> Self {
        Self {
            last_frame: Instant::now(),
            frame_times: Vec::with_capacity(120),

            fps: 0.0,
            min_fps: 0.0,
            max_fps: 0.0,
            avg_frame_time: 0.0,
        }
    }
}
impl AppStats {
    pub fn reset(&mut self) {
        self.frame_times.clear();
        self.last_frame = Instant::now();

        self.fps = 0.0;
        self.min_fps = 0.0;
        self.max_fps = 0.0;
        self.avg_frame_time = 0.0;
    }
    pub fn update_frame(&mut self) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;

        self.frame_times.push(frame_time);
        if self.frame_times.len() > 100 {
            self.frame_times.remove(0);
        }

        self.avg_frame_time = self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32;

        self.fps = 1.0 / self.avg_frame_time;

        self.min_fps = 1.0
            / self
                .frame_times
                .iter()
                .copied()
                .fold(f32::NEG_INFINITY, f32::max);

        self.max_fps = 1.0
            / self
                .frame_times
                .iter()
                .copied()
                .fold(f32::INFINITY, f32::min);
    }
}
