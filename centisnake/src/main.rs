mod constants;
mod render;
mod snake;
mod game;
mod motion; 

use render::blocks_to_pixels;
use game::Game;
use piston_window::*;
use constants::{WIDTH, HEIGHT, TITLE, BACKGROUND};

fn main() {
    let size = [blocks_to_pixels(WIDTH), blocks_to_pixels(HEIGHT)];

    let mut window: PistonWindow = WindowSettings::new(TITLE, size)
        .resizable(false)
        .build()
        .unwrap();

    let mut snake_game: Game = Game::new(WIDTH, HEIGHT);
    snake_game.start();
    
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            snake_game.press_key(key);
        }

        window.draw_2d(&event, |cx, g, _| {
            clear(BACKGROUND, g);
            // score caching to fix - Piston wrapper issue 
            // text::Text::new_color(SCORE, 30)
            //     .draw(
            //         snake_game.get_score().to_string().as_ref(),
            //         &mut glyphs,
            //         &cx.draw_state,
            //         cx.transform.trans(30.0, 100.0),
            //         g,
            //     )
            //     .unwrap();
            snake_game.render(cx, g);
        });

        event.update(|arg| {
            window.set_title(format!("SnakeAI Game [SCORE: {} ]", snake_game.get_score().to_string()));
            snake_game.update(arg.dt);
        });

    }


}