#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}


struct Momentum{
    upward : f32,
    downward : f32,
    leftward : f32,
    rightward : f32,
}

impl Momentum{
    fn new() -> Self {
        Momentum{
            upward:0.0,
            downward:1.0,
            leftward:0.0,
            rightward:0.0,
        }

    }
}

pub struct Character {
    pub coordinate: PointF,
    orientation: Degrees,
    scaling: PointF,
    momentum : Momentum,
}

impl Character {
    pub fn new(x: f32, y: f32) -> Self {
        Character {
            coordinate: PointF::new(x, y),
            orientation: Degrees::new(0.0),
            scaling: PointF::new(2.0, 2.0),
            momentum : Momentum::new(),
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

    fn thrust(&mut self,direction : Direction){
        match direction {
            Direction::Up => self.momentum.upward += 0.2
            Direction::Down =>
            Direction::Left =>
            Direction::Right =>
        }
    }

    fn apply_momentum(){
        todo!();
    }
}
