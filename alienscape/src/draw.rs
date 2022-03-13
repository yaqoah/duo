use ab_glyph::PxScale;
use mint::Point2;
use ggez::graphics::{Font, Text, Image, draw, DrawParam};
use ggez::*;
use crate::constants::{
    SCOPE_PATH, INTRO_PATH, WIN_PATH,
    TEXT_FONT_PATH, OVER_PATH,
    ALIEN_PATH, CLOWN_PATH, SHIP_PATH};
use crate::character::{Alien, Clown, Ship};


fn get_font(ctx: &mut Context, font_path: &str) -> Font{
     Font::new(ctx, font_path).unwrap()
}

pub fn draw_alien(ctx: &mut Context, alien: &mut Alien) {
    let bg = Image::new(ctx, ALIEN_PATH).unwrap();
    draw(ctx, &bg,  DrawParam::default().dest(Point2{x:alien.get_x(), y:alien.get_y()})).unwrap();
}

pub fn draw_clown(ctx: &mut Context, clown: &mut Clown) {
    let bg = Image::new(ctx, CLOWN_PATH).unwrap();
    draw(ctx, &bg,  DrawParam::default().dest(Point2{x:clown.get_x(), y:clown.get_y()})).unwrap();
}

pub fn draw_ship(ctx: &mut Context, ship: &mut Ship) {
    let bg = Image::new(ctx, SHIP_PATH).unwrap();
    draw(ctx, &bg,  DrawParam::default().dest(Point2{x:ship.get_x(), y:ship.get_y()})).unwrap();
}

pub fn draw_scope(ctx: &mut Context, alien: &mut Alien) {
    let scope_img = Image::new(ctx, SCOPE_PATH).unwrap();
    draw(ctx, &scope_img, DrawParam::default().dest(Point2{x:alien.get_x()-120.0, y:alien.get_y()-110.0})).unwrap();
}

pub fn draw_start_screen(ctx: &mut Context) {
    let bg = Image::new(ctx, INTRO_PATH).unwrap();
    draw(ctx, &bg,  DrawParam::default().dest(Point2{x:0.0, y:0.0})).unwrap();

    let mut title: Text =  Text::new("ALIENSCAPE");
    title.set_font(get_font(ctx, TEXT_FONT_PATH), PxScale::from(65.0));
    draw(ctx, &title, DrawParam::default().dest(Point2{x:125.0, y:55.0})).unwrap();

    let mut start: Text =  Text::new("hit SPACE to start");
    start.set_font(get_font(ctx, TEXT_FONT_PATH), PxScale::from(25.0));
    draw(ctx, &start, DrawParam::default().dest(Point2{x:230.0, y:420.0})).unwrap();

    let mut hook: Text = Text::new("ESCAPE THE CLOWNS");
    hook.set_font(get_font(ctx, TEXT_FONT_PATH), PxScale::from(38.0));
    draw(ctx, &hook, DrawParam::default().dest(Point2{x:123.0, y:455.0})).unwrap();
}

pub fn draw_win_screen(ctx: &mut Context) {
    let bg = Image::new(ctx, WIN_PATH).unwrap();
    draw(ctx, &bg, DrawParam::default().dest(Point2{x:0.0, y:0.0})).unwrap();

    let mut win_message: Text =  Text::new("That was close, good job!");
    win_message.set_font(get_font(ctx, TEXT_FONT_PATH), PxScale::from(38.0));
    draw(ctx, &win_message, DrawParam::default().dest(Point2{x:95.0, y:25.0})).unwrap();

    let mut hook: Text = Text::new("hit SPACE to play again");
    hook.set_font(get_font(ctx, TEXT_FONT_PATH), PxScale::from(38.0));
    draw(ctx, &hook, DrawParam::default().dest(Point2{x:115.0, y:550.0})).unwrap();
}

pub fn draw_lose_screen(ctx: &mut Context) {
    let bg = Image::new(ctx, OVER_PATH).unwrap();
    draw(ctx, &bg, DrawParam::default().dest(Point2{x:0.0, y:0.0})).unwrap();

    let mut win_message: Text =  Text::new("DEM CLOWNS ARE SAVAGE!");
    win_message.set_font(get_font(ctx, TEXT_FONT_PATH), PxScale::from(25.0));
    draw(ctx, &win_message, DrawParam::default().dest(Point2{x:230.0, y:510.0})).unwrap();

    let mut hook: Text = Text::new("hit SPACE to try again");
    hook.set_font(get_font(ctx, TEXT_FONT_PATH), PxScale::from(38.0));
    draw(ctx, &hook, DrawParam::default().dest(Point2{x:85.0, y:525.0})).unwrap();
}
