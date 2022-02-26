use piston_window::types::Color;    
use piston_window::{circle_arc, rectangle, Context, G2d};

use crate::motion::{Position};
use crate::constants::{DIAMETER_IN_PIXELS, ARC_START, ARC_END};

pub fn render_arc(cx: &Context, g: &mut G2d, c: Color, pos: &Position) {
    circle_arc (
        c,
        to_radius(DIAMETER_IN_PIXELS) - 5.0,
        ARC_START,
        ARC_END,
        [
            pos.x as f64 * DIAMETER_IN_PIXELS,
            pos.y as f64 * DIAMETER_IN_PIXELS,
            DIAMETER_IN_PIXELS,
            DIAMETER_IN_PIXELS,                         
        ],
        cx.transform,
        g,
    )
}           

pub fn render_fruit(cx: &Context, g: &mut G2d, rgba: Color, pos: &Position) {
    rectangle(
        rgba,
        [
            pos.x as f64 * DIAMETER_IN_PIXELS,
            pos.y as f64 * DIAMETER_IN_PIXELS,
            DIAMETER_IN_PIXELS,
            DIAMETER_IN_PIXELS,
        ],
        cx.transform,
        g,
    );
}

pub fn render_overlay(cx: &Context, g: &mut G2d, rgba: Color, size: (u32, u32)) { 
    rectangle(
        rgba,
        [
            0.0,
            0.0,
            blocks_to_pixels(size.0) as f64,
            blocks_to_pixels(size.1) as f64,
        ],
        cx.transform,
        g,
    );
}

pub fn to_radius(d: f64) -> f64{
    d / 2.0
}

pub fn blocks_to_pixels(n: u32) -> u32 {
    n * DIAMETER_IN_PIXELS as u32
}
