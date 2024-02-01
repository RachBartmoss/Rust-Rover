#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

pub enum Direction {
    Up,
    Left,
    Right,
}

struct Momentum {
    upward: f32,
    downward: f32,
    leftward: f32,
    rightward: f32,
}

impl Momentum {
    fn new() -> Self {
        Momentum {
            upward: 0.0,
            downward: 0.5,
            leftward: 0.0,
            rightward: 0.0,
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
            scaling: PointF::new(2.0, 2.0),
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

    pub fn thrust(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.momentum.upward < 2.0 {
                    self.momentum.upward += 0.3
                } else {
                    self.momentum.upward = 2.0
                }
            }

            Direction::Left => self.momentum.leftward += 0.3,
            Direction::Right => self.momentum.rightward += 0.3,
        }
    }

    pub fn apply_momentum(&mut self) {
        self.coordinate.x += self.momentum.rightward;
        self.coordinate.x -= self.momentum.leftward;

        self.coordinate.y -= self.momentum.upward;
    }

    pub fn apply_gravity_and_drag(&mut self) {
        self.coordinate.y += self.momentum.downward;
        if self.momentum.upward > 0.0 {
            self.momentum.upward -= 0.1;
        }
        if self.momentum.leftward > 0.0 {
            self.momentum.leftward -= 0.1;
        }
        if self.momentum.rightward > 0.0 {
            self.momentum.rightward -= 0.1;
        }
    }
}
