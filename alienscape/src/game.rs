use crate::character::{Alien, Clown, Ship, new_alien, new_clowns, new_ship, clash, in_scope, Direction};
use crate::constants::{SCOPE_RADIUS};
use crate::draw::{draw_alien, draw_clown, draw_ship, draw_scope, draw_start_screen, draw_win_screen, draw_lose_screen};
use ggez::graphics::{clear, present};
use ggez::error::GameResult;
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::{Context, timer};

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Start,
    InGame,
    GameOver,
    Victory,
}

pub struct Game {
    alien: Alien,
    clowns: Vec<Clown>,
    ship: Ship,
    boarded_plane: bool,
    clash_clown: bool,
    state: State,
}

impl Game {
    pub fn new() -> GameResult<Game>{
        let alien = new_alien();
        let clowns = new_clowns();
        let ship = new_ship();

        let game = Game {
            alien: alien,
            clowns: clowns,
            ship: ship,
            boarded_plane: false,
            clash_clown: false,
            state: State::Start,
        };
        Ok(game)
    }

    fn start(&mut self) {
        self.state = State::InGame;
    }

    fn end(&mut self) {
        self.reset();
    }

    fn reset(&mut self) {
        self.alien = new_alien();
        self.clowns = new_clowns();
        self.ship = new_ship();
        self.boarded_plane =  false;
        self.clash_clown = false;
    }

    fn set_state(&mut self) {
        match (self.boarded_plane, self.clash_clown) {
            (true, false) => {self.state = State::Victory; self.end()},
            (false, true)  => {self.state = State::GameOver; self.end()},
            (true, true) => {self.state = State::GameOver; self.end()},
            (false, false) => {}
        }
    }

    fn is_running(&mut self) -> bool {
        self.state == State::InGame
    }

}

impl EventHandler for Game {
    fn update (&mut self, ctx: &mut Context) -> GameResult<()> {
        let t = timer::duration_to_f64(timer::average_delta(ctx)) as f32;
        self.alien.motion(t);
        self.boarded_plane = clash(
            self.alien.get_x(), 
            self.alien.get_y(), 
            self.ship.get_x(), 
            self.ship.get_y()
        );
        for clown in &mut self.clowns {
            if clash(self.alien.get_x(), 
                self.alien.get_y(), 
                clown.get_x(), 
                clown.get_y()) {
                self.clash_clown = true;
            } 
            if in_scope(SCOPE_RADIUS, 
                self.alien.get_x(), 
                self.alien.get_y(), 
                clown.get_x(), 
                clown.get_y()) {
                clown.activate(&self.alien, t);
            } else {
                clown.deactive();
            };
        }
        self.set_state();
        Ok(())
    }

    fn draw (&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        if self.state == State::Start {
            draw_start_screen(ctx);
        } else if self.state == State::GameOver {
            draw_lose_screen(ctx);
        } else if self.state == State::Victory {
            draw_win_screen(ctx);
        } else {
            draw_scope(ctx, &mut self.alien);
            draw_alien(ctx, &mut self.alien);
            for enemy in &mut self.clowns {
                if in_scope(SCOPE_RADIUS, self.alien.get_x(), self.alien.get_y(), enemy.get_x(), enemy.get_y()) {
                    draw_clown(ctx, enemy);
                }
            }
            if in_scope(SCOPE_RADIUS, self.alien.get_x(), self.alien.get_y(), self.ship.get_x(), self.ship.get_y()) {
                draw_ship(ctx, &mut self.ship);
            }
        }
        present(ctx).unwrap();
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _: KeyMods, _:bool) {
        match (keycode, self.is_running()) {
            (KeyCode::Up, true) => self.alien.set_direction(Direction::Up),
            (KeyCode::Down, true) => self.alien.set_direction(Direction::Down),
            (KeyCode::Left, true) => self.alien.set_direction(Direction::Left),
            (KeyCode::Right, true) => self.alien.set_direction(Direction::Right),
            (KeyCode::Space, true) => self.alien.boost(),
            (KeyCode::Space, false) => self.start(),
            _ => {}
       
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _: KeyMods) {
        match (keycode, self.is_running()) {
            (KeyCode::Up, true) => self.alien.reset_direction(Direction::Up),
            (KeyCode::Down, true) => self.alien.reset_direction(Direction::Down),
            (KeyCode::Left, true) => self.alien.reset_direction(Direction::Left),
            (KeyCode::Right, true) => self.alien.reset_direction(Direction::Right),
            _ => {}
       
        }  
    }
}