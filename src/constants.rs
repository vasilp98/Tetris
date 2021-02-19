pub const SQUARE_SIZE: f32 = 35.0;
pub const BORDER_SIZE: f32 = 1.0;
pub const BOARD_WIDTH: f32 = SQUARE_SIZE * 10.0;
pub const BOARD_HEIGHT: f32 = SQUARE_SIZE * 18.0;
pub const WINDOW_WIDTH: f32 = BOARD_WIDTH + (2.0 * SQUARE_SIZE) + (7.0 * SQUARE_SIZE);
pub const WINDOW_HEIGHT: f32 = BOARD_HEIGHT + 2.0 * SQUARE_SIZE;
pub const ROTATION_INTERVAL: i32 = 4;
pub const MOVE_INTERVAL: i32 = 1;
pub const ENTRY_POINT: (f32, f32) = (SQUARE_SIZE, SQUARE_SIZE);
pub const DEFAULT_SPEED: f32 = 0.01;
pub const SINGLE_LINE_POINTS: i32 = 40;
pub const DOUBLE_LINE_POINTS: i32 = 100;
pub const TRIPLE_LINE_POINTS: i32 = 300;
pub const TETRIS_POINTS: i32 = 1200;
pub const LINES_TO_LEVEL_UP: i32 = 10;
pub const VIEWING_AREA_ROWS_COUNT: i32 = 10;