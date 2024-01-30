#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

enum Gamemode {
    Menu,
    Play,
    Pause,
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
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {}
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Practice Game")
        .build()?;

    main_loop(context, State::new())
}
