#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;
mod character;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 60.0;

enum Gamemode {
    Menu,
    Play,
    Pause,
    Quit,
}

struct State {
    mode: Gamemode,
    character: character::Character,
    frame_time: f32,
}

impl State {
    fn new() -> Self {
        State {
            mode: Gamemode::Menu,
            character: character::Character::new((SCREEN_WIDTH / 2) as f32, 25.0),
            frame_time: 0.0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Game Project");
        ctx.print_centered(7, "Press (P) to start playing !");
        ctx.print_centered(8, "Press (Q) to quit the game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(ctx),
                VirtualKeyCode::Q => self.quit(ctx),
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.character.apply_momentum();
            self.character.apply_gravity_and_drag(1.0);
            self.frame_time = 0.0;
        }

        self.character.render(ctx);

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Up => self.character.thrust(character::Direction::Up, 0.5, 3.0),
                VirtualKeyCode::Left => self.character.thrust(character::Direction::Left, 0.4, 3.0),
                VirtualKeyCode::Right => {
                    self.character.thrust(character::Direction::Right, 0.4, 3.0)
                }
                _ => {}
            }
        }


    }

    fn pause(&mut self, ctx: &mut BTerm) {
        todo!();
    }

    fn quit(&mut self, ctx: &mut BTerm) {
        todo!();
    }

    fn restart(&mut self, ctx: &mut BTerm) {
        self.character = character::Character::new((SCREEN_WIDTH / 2) as f32, 0.0 as f32);
        self.mode = Gamemode::Play;
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
        .with_fancy_console(80, 50, "terminal8x8.png")
        .with_title("Game Project")
        .with_vsync(false)
        .with_fps_cap(60.0)
        .build()?;

    main_loop(context, State::new())
}
