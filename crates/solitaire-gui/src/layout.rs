pub const CARD_W: f32 = 90.0;
pub const CARD_H: f32 = 128.0;

pub const TOP_Y: f32 = 20.0;
pub const BOT_Y: f32 = 180.0;

pub const SPACING_X: f32 = 20.0;
pub const X_START: f32 = 20.0;

pub const TOP_ROW: [(f32, f32); 8] = horizontal_row(7, X_START, CARD_W + SPACING_X, TOP_Y);

pub const STOCK_POS: (f32, f32) = TOP_ROW[0];
pub const WASTE_POS: (f32, f32) = TOP_ROW[1];
pub const FOUNDATION_POS: [(f32, f32); 4] = [TOP_ROW[3], TOP_ROW[4], TOP_ROW[5], TOP_ROW[6]];

pub const BOT_ROW: [(f32, f32); 8] = horizontal_row(7, X_START, CARD_W + SPACING_X, BOT_Y);

pub const COLUMN_POS: [(f32, f32); 7] = [
    BOT_ROW[0], BOT_ROW[1], BOT_ROW[2], BOT_ROW[3], BOT_ROW[4], BOT_ROW[5], BOT_ROW[6],
];
pub const COLUMN_CARDS_SPACING: f32 = 40.0;

pub const fn horizontal_row(count: usize, x_start: f32, spacing_x: f32, y: f32) -> [(f32, f32); 8] {
    // max size must be known: we use 8 to cover up to tableau columns
    let mut arr = [(0.0, 0.0); 8];
    let mut i = 0;
    while i < count {
        arr[i] = (x_start + spacing_x * i as f32, y);
        i += 1;
    }
    arr
}
