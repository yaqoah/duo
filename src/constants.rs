use piston_window::types::Color;

// Snake length, body segment diameter & shape 
pub const INITIAL_TAIL_LENGHT: usize = 2;
pub const DIAMETER_IN_PIXELS: f64 = 20.0;
pub const ARC_START: f64 = 0.0;
pub const ARC_END: f64 = 360.0;

// Colors
pub const BACKGROUND: Color = [1.0, 0.839, 0.619, 1.0];
// pub const SCORE: Color = [1.0, 0.0, 0.0, 1.0]; 
pub const SNAKE: Color = [1.0, 0.309, 0.058, 1.0];
pub const SNAKE_HEAD: Color = [0.541, 0.094, 0.0, 1.0];
pub const FRUIT: Color = [0.223, 0.411, 0.086, 1.0];
pub const OVERLAY: Color = [0.152, 0.286, 0.329, 1.0];

// Piston Window & Gameplay properties
pub const TITLE: &'static str = "SNAKE - AI TO BE ";
pub const WIDTH: u32 = 24;
pub const HEIGHT: u32 = 24;
pub const FPS: f64 = 15.0;
