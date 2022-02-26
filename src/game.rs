use rand::{thread_rng, Rng};
use piston_window::{Context, G2d};
use crate::render::*;
use crate::motion::{Direction, Position};
use crate::snake::Snake;
use crate::constants::{FRUIT, OVERLAY, FPS};
use crate::keyboard::Key;

pub fn get_random_pos(width: u32, height: u32 ) -> Position {
    let mut generator = thread_rng();

    Position {
        x: generator.gen_range(0..width as i32), 
        y: generator.gen_range(0..height as i32),
    }
}

pub struct Game {
    snake: Snake,
    fruit: Position, 
    size: (u32, u32),
    pause_time: f64,
    score: u32, 
    pause: bool,
    finish: bool,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            snake: Snake::new(get_random_pos(width, height)),
            fruit: get_random_pos(width, height), 
            size: (width, height),
            pause_time: 0.0,
            score: 0, 
            pause: true,
            finish: false, 
        }
    }

    pub fn start(&mut self) {
        self.pause = false;
    }

    pub fn render(&self, cx: Context, g: &mut G2d) {
        render_fruit(&cx, g, FRUIT, &self.fruit);
        self.snake.render(&cx, g); 

        if self.finish {
            render_overlay(&cx, g, OVERLAY, self.size);
        }
    }

    pub fn press_key(&mut self, key: Key) {

        match key {
            Key::A | Key::Left => self.snake.set_direction(Direction::Left),
            Key::W | Key::Up => self.snake.set_direction(Direction::Up),
            Key::D | Key::Right => self.snake.set_direction(Direction::Right),
            Key::S | Key::Down => self.snake.set_direction(Direction::Down),
            Key::R => self.restart(),
            _ => {}
        }
    }

    pub fn update(&mut self, time: f64) {
        self.pause_time += time;

        if self.pause_time > 1.0/FPS && !self.finish && !self.pause {
            self.pause_time = 0.0;

            if !self.snake.will_collide() && !self.snake.collided() {
                self.snake.update(self.size.0, self.size.1);

                if *self.snake.get_head_position() == self.fruit {
                    self.snake.grow();
                    self.snake.update(self.size.0, self.size.1);
                    self.fruit = get_random_pos(self.size.0, self.size.1);
                    self.calculate_score();
                }
            } else {
                self.finish = true;
            }
        }
    }

    fn calculate_score(&mut self) {
        self.score = (self.snake.get_len() * 10) as u32
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    fn restart(&mut self) {
        *self = Game::new(self.size.0, self.size.1);
        self.start();
    }

}