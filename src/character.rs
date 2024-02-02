#![warn(clippy::all, clippy::pedantic)]

use std::cmp::Ordering;

use bracket_lib::prelude::*;

pub enum Direction {
    Up,
    Left,
    Right,
}

#[derive(PartialEq, PartialOrd)]
pub struct Momentum {
    pub vertical: f32,
    pub horizontal: f32,
}

impl Momentum {
    fn new() -> Self {
        Momentum {
            vertical: -1.0,
            horizontal: 0.0,
        }
    }
}

pub struct Character {
    pub coordinate: PointF,
    orientation: Degrees,
    scaling: PointF,
    pub momentum: Momentum,
}

impl Character {
    pub fn new(x: f32, y: f32) -> Self {
        Character {
            coordinate: PointF::new(x, y),
            orientation: Degrees::new(0.0),
            scaling: PointF::new(1.0, 1.0),
            momentum: Momentum::new(),
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();

        ctx.set_fancy(
            self.coordinate,
            1,
            self.orientation,
            self.scaling,
            BLUE,
            BLACK,
            to_cp437('^'),
        );
        ctx.set_active_console(0);
    }

    pub fn thrust(&mut self, direction: Direction, acceleration: f32, max_acceleration: f32) {
        let negative_max_acceleration: f32 = max_acceleration * -1.0;

        match direction {
            Direction::Up => {
                if self.momentum.vertical > negative_max_acceleration {
                    self.momentum.vertical -= acceleration
                }
            }
            Direction::Left => {
                if self.momentum.horizontal > negative_max_acceleration {
                    self.momentum.horizontal -= acceleration
                }
            }
            Direction::Right => {
                if self.momentum.horizontal < max_acceleration {
                    self.momentum.horizontal += acceleration
                }
            }
        }
    }

    pub fn apply_momentum(&mut self) {
        self.coordinate.x += self.momentum.horizontal;
        self.coordinate.y += self.momentum.vertical;
    }

    pub fn apply_gravity_and_drag(&mut self, gravity: f32) {
        match self.momentum.vertical.partial_cmp(&gravity) {
            Some(Ordering::Greater) => self.momentum.vertical = gravity,
            Some(Ordering::Less) => self.momentum.vertical += 0.02,
            Some(Ordering::Equal) => {}
            _ => {}
        }
        match self.momentum.horizontal.partial_cmp(&0.0) {
            Some(Ordering::Greater) => self.momentum.horizontal -= 0.1,
            Some(Ordering::Less) => self.momentum.horizontal += 0.1,
            Some(Ordering::Equal) => {}
            _ => {}
        }
        if self.momentum.horizontal > -0.1 && self.momentum.horizontal < 0.1 {
            self.momentum.horizontal = 0.0;
        }
    }
}
