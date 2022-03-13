use crate::constants::{
    INITIAL_SPEED, MAX_SPEED, 
    OPTIMUM_SPEED,ACCELERATION, POWER_UP, 
    INITIAL_ALIEN_X, INITIAL_ALIEN_Y, ENEMIES, 
    SCOPE_RADIUS, OBJ_HEIGHT, OBJ_WIDTH, 
    WINDOW_HEIGHT, WINDOW_WIDTH,
    BOOST, BOOSTABLE, BOOST_TIME, COOLDOWN_TIME
};
use crossbeam_channel::{select, tick};
use std::thread;
use std::time::{Duration};
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Alien {
    x: f32,
    y: f32,
    speed: f32,
    up: bool,
    down: bool,
    right: bool, 
    left: bool, 
}

impl Alien {
    pub fn new(x: f32, y: f32) -> Alien {
        Alien {
            x: x,
            y: y,
            speed: INITIAL_SPEED,
            up: false,
            down: false,
            right: false,
            left: false,
        } 
    }
    
    pub fn set_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up  => self.up = true,
            Direction::Down  => self.down = true,
            Direction::Right  => self.right = true,
            Direction::Left  => self.left = true,
        }
    }

    pub fn reset_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up  => self.up = false,
            Direction::Down  => self.down = false,
            Direction::Right  => self.right = false,
            Direction::Left  => self.left = false,
        }
    }

    pub fn motion(&mut self, time_per_frames: f32) {
        unsafe{
            let mut momentum: f32 = OPTIMUM_SPEED * time_per_frames;
            self.speed = momentum;
            if POWER_UP {
                momentum += BOOST;
            }
            self.escape(momentum);
        }
    }

    pub fn get_x(&self) -> f32{
        self.x
    }

    pub fn get_y(&self) -> f32{
        self.y
    }

    pub fn _get_speed(&self) -> f32 {
        self.speed
    }

    pub fn escape(&mut self, momentum: f32) {
            if self.up && self.y - (OBJ_HEIGHT/2.0) > 0.0 {
                self.y -= momentum;
            }
            if self.down && self.y + (OBJ_HEIGHT/2.0) < WINDOW_HEIGHT {
                self.y += momentum;
            }
            if self.right && self.x + (OBJ_HEIGHT/2.0) < WINDOW_WIDTH {
                self.x += momentum;
            }
            if self.left && self.x - (OBJ_WIDTH/2.0) > 0.0 {
                self.x -= momentum;
            }
    }

    pub fn boost(&mut self) {
        unsafe {
            if  BOOSTABLE {
                POWER_UP = true;
                BOOSTABLE = false;
                let boost_period = tick(Duration::from_millis(BOOST_TIME));
                let cooldown_period = tick(Duration::from_millis(COOLDOWN_TIME));
                thread::spawn(move || loop {
                    select! {
                        recv(boost_period) -> _ => {
                            POWER_UP = false;
                        },
                        recv(cooldown_period) -> _  => {
                            BOOSTABLE = true;
                            break;
                        },
                    }
                });
            }
        }
    }

}

pub struct Ship {
    x: f32,
    y: f32,
} 

impl Ship {
    pub fn new(x: f32, y: f32) -> Ship {
        Ship {
            x: x, 
            y: y,
        }
    }

    pub fn get_x(&self) -> f32{
        self.x
    }

    pub fn get_y(&self) -> f32{
        self.y
    }
}

pub struct Clown {
    x: f32,
    y: f32,
    speed: f32,
}

impl Clown {
    pub fn new(x: f32, y: f32) -> Clown {
        Clown {
            x: x,
            y: y,
            speed: INITIAL_SPEED,
        }
    }

    pub fn get_x(&self) -> f32{
        self.x
    }

    pub fn get_y(&self) -> f32{
        self.y
    }

    pub fn activate(&mut self, enemy: &Alien, time_per_frames: f32) {
        let left = self.x < enemy.x;
        let right = self.x > enemy.x;
        let top = self.y < enemy.y;
        let bottom = self.y > enemy.y;
        let mut movement: f32 = self.speed * time_per_frames;
        if left && top || left && bottom || right && top || right && bottom {
            movement /= 2.0;
        }
        if left {
            self.x += movement
        }
        if right {
            self.x -= movement
        }
        if top {
            self.y += movement
        }
        if bottom {
            self.y -= movement
        }
        if self.speed < MAX_SPEED {
            self.speed += ACCELERATION
        }
    }

    pub fn deactive(&mut self) {
        self.speed = INITIAL_SPEED
    }
    
}

pub fn in_scope(r: f32, xc: f32, yc: f32, x: f32, y: f32) -> bool {
    return ((xc - x).abs().powf(2.0) + (yc - y).abs().powf(2.0)).sqrt() < r;
}

pub fn get_random_position() -> (f32, f32) {
    let mut generator = rand::thread_rng();
    let x = generator.gen_range(0.0..WINDOW_WIDTH) as f32;
    let y = generator.gen_range(0.0..WINDOW_HEIGHT) as f32;

    if in_scope(SCOPE_RADIUS, x, y, INITIAL_ALIEN_X, INITIAL_ALIEN_Y) {
        return get_random_position();
    }
    (x, y)
}

pub fn new_alien() -> Alien {
    Alien::new(INITIAL_ALIEN_X, INITIAL_ALIEN_Y)
}

pub fn new_ship() -> Ship {
    let (new_x, new_y) = get_random_position();
    return Ship::new(new_x, new_y);
}

pub fn new_clowns() -> Vec<Clown> {
    let mut index: usize = 0;
    let mut clowns: Vec<Clown> = vec![];
    while index < ENEMIES {
        let (x, y) = get_random_position();
        clowns.push(Clown::new(x, y));
        index += 1;
    }
    clowns
}

pub fn clash(x: f32, y: f32, x2: f32, y2: f32) -> bool {
    let left = x + OBJ_WIDTH > x2 && x + OBJ_WIDTH < x2 + OBJ_WIDTH;
    let right = x - OBJ_WIDTH < x2 && x - OBJ_HEIGHT > x2 - OBJ_WIDTH;
    let top = y < y2 + OBJ_HEIGHT && y - OBJ_HEIGHT > y2 - OBJ_HEIGHT;
    let bottom = y + OBJ_HEIGHT > y2 && y + OBJ_HEIGHT < y2 + OBJ_HEIGHT;
    return (left && right) || (left && bottom)|| (right && top) || (right && bottom);
}