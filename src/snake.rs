use std::collections::LinkedList;
use piston_window::{Context, G2d};

use crate::constants::{INITIAL_TAIL_LENGHT, SNAKE, SNAKE_HEAD};
use crate::motion::{Position, Direction};
use crate::render::{render_arc};

pub struct Snake {
    direction: Direction,
    head: Position,
    tail: LinkedList<Position>,
    snake_moved: bool,
}

impl Snake {
    pub fn new(head: Position) -> Self {
        let (x, y) = (head.x, head.y);
        let mut tail = LinkedList::new();

        for i in 1..(INITIAL_TAIL_LENGHT + 1) {
            tail.push_back(Position{ x  , y: y - i as i32});
        }

        Self {
            direction: Direction::Down,
            head: Position { x,y },
            tail: tail,
            snake_moved: false,
        }
    }

    pub fn update(&mut self, width: u32, height: u32) {
        if self.tail.len() > 0 {
            self.tail.push_front(self.head.clone());
            self.tail.pop_back();
        }

        match self.direction {
            Direction::Up => self.head.y -= 1,
            Direction::Right => self.head.x += 1,
            Direction::Down => self.head.y += 1,
            Direction::Left => self.head.x -= 1,
        }

        if self.head.x >= width as i32 {
            self.head.x = 0;
        } else if self.head.y >= height as i32 {
            self.head.y = 0;
        } else if self.head.y < 0 {
            self.head.y = height as i32;
        } else if self.head.x < 0 {
            self.head.x = width as i32;
        }

        self.snake_moved = true;
    } 

    pub fn render(&self, cx: &Context, g: &mut G2d) {
        render_arc(&cx, g, SNAKE_HEAD, &self.head);

        for circle in self.tail.iter() { 
            render_arc(&cx, g, SNAKE, circle);
        }
    }

    pub fn get_len(&self) -> usize {
        &self.tail.len() - INITIAL_TAIL_LENGHT
    }

    pub fn set_direction(&mut self, dir: Direction) {
        if dir == self.direction.opposite() || !self.snake_moved {
            return;
        }

        self.direction = dir;
        self.snake_moved = false;
    }

    pub fn next_head_position(&self) -> Position {
        let mut position = self.head.clone();
        
        match self.direction {
            Direction::Up => position.y -= 1,
            Direction::Left => position.x -= 1,
            Direction::Down => position.y += 1,
            Direction::Right => position.x += 1,
        }

        position
    }

    pub fn get_head_position(&self) -> &Position {
        &self.head
    }

    pub fn collided(&self) -> bool {
        for pos in self.tail.iter() {
            if *pos == self.head {
                return true;
            }
        }

        false
    }

    pub fn will_collide(&self) -> bool {
        let next = self.next_head_position();

        for pos in self.tail.iter() {
            if *pos == next {
                return true;
            }
        }

        false
    }

    pub fn grow(&mut self) {
        let last = match self.tail.back() {
            Some(pos) => pos.clone(),
            None => self.head.clone(),
        };

        self.tail.push_back(last);
    }

}
