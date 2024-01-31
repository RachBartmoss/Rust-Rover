#![warn(clippy::all, clippy::pedantic)]

use std::alloc::GlobalAlloc;

use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

enum Gamemode {
    Menu,
    Play,
    Pause,
    Quit,
}

struct State {
    mode: Gamemode,
}

impl State {
    fn new() -> Self {
        State {
            mode: Gamemode::Menu,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        todo!();
    }

    fn play(&mut self, ctx: &mut BTerm) {
        todo!();
    }

    fn pause(&mut self, ctx: &mut BTerm) {
        todo!();
    }

    fn quit(&mut self, ctx: &mut BTerm) {
        todo!();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            Gamemode::Menu => self.main_menu(ctx),
            Gamemode::Play => self.play(ctx),
            Gamemode::Pause => self.pause(ctx),
            Gamemode::Quit => self.quit(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Practice Game")
        .build()?;

    main_loop(context, State::new())
}
