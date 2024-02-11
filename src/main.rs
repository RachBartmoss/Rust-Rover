use bracket_lib::prelude::*;

/// The character module handles everything related to the game's character, such as the character's ship position and momentum
mod character;

/// The hazards module handles everything related to the game's hazards, such as the asteroids
mod hazards;

/// The map module handles everything related to the game's map, such as the landing pad
mod map;

/// The width of the window used in the game, the project uses bracket-lib's
/// BTermBuilder to set the window size to 80x50
const SCREEN_WIDTH: i32 = 80;

/// The heigth of the window used in the game, the project uses bracket-lib's
/// BTermBuilder to set the window size to 80x50
const SCREEN_HEIGHT: i32 = 50;

/// The duration of the frame in milliseconds, certain game actions are only
/// performed after a certain amount of time has passed to avoid the game
/// running too fast
const FRAME_DURATION: f32 = 10.0;

/// Variant of gamemodes, This is used in the GameState::tick() implementation
///  of the project, this is bracket-lib's way of handling the main game loop.
/// This enumeration's role is to tell the main loop which game mode to run
/// and display
///
/// # Variants
/// - Menu: The game's main menu with options to start playing or quit the game
/// - Play: The game's main mode, where the player can control the character
/// - Pause: The game's pause mode, where the game is paused and the player can
///         resume playing or quit the game
/// - Quit: The game's quit mode, where the game is about to close
enum Gamemode {
    Menu,
    Play,
    Pause,
    Quit,
    Lost,
}

/// The main struct of the game, this struct is used to store the game's state
/// and is used in the main loop to run the game, it keeps track of the main
/// game's components, suche as score, the position of the player character or
/// of the asteroids
struct State {
    /// The current game mode, this is used to tell the main loop which game
    /// mode to run and display
    mode: Gamemode,

    /// The character struct, this is used to store the player's character's
    /// position and momentum
    character: character::Character,

    /// The asteroids vector, this is used to store the asteroids' position and
    /// trajectory
    asteroids: Vec<hazards::Asteroid>,

    /// The frame time, this is used to keep track of the time passed since the
    /// last frame, this is used to avoid the game running too fast
    frame_time: f32,

    /// The score, this is used to keep track of the player's score
    score: usize,

    /// The avoided, this is used to keep track of the number of asteroids
    /// avoided by the player, which in turn is used to update score once the
    /// player avoided enough asteroids
    avoided: usize,

    /// The landing pad, this is used to store the landing pad's position and
    /// heigth
    landing_pad: map::LandingPad,
}

impl State {
    /// The constructor of the State struct, this is used to create a new
    /// instance of the State struct, this is used to initialize the game's
    /// state. One of these struct is created at the start of the game and is
    /// set to the menu.
    /// Nevertheless this initial state is also populated with starting positions
    /// for the character's ship and the asteroids, as well as the landing pad
    ///
    /// # Returns
    /// A new instance of the State struct
    ///
    /// # Examples
    /// ```
    /// let state = State::new();
    /// ```
    fn new() -> Self {
        let mut random = RandomNumberGenerator::new();
        let starting_point = random.range(0.0, SCREEN_WIDTH as f32);
        let asteroids = vec![hazards::Asteroid::new(starting_point, 0.0)];

        State {
            mode: Gamemode::Menu,
            character: character::Character::new((SCREEN_WIDTH / 2) as f32, SCREEN_HEIGHT as f32),
            frame_time: 0.0,
            asteroids,
            score: 0,
            avoided: 0,
            landing_pad: map::LandingPad::new(random.range(0, SCREEN_WIDTH)),
        }
    }

    /// The main menu function, this is used to display the game's main menu
    /// while the state's mode is set to Menu, from there the player can start
    /// playing or quit the game
    /// The function is only called by the GameState::tick() function.
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.printer(
            SCREEN_WIDTH / 2,
            20,
            "#[orange]RUST ROVER#[]",
            TextAlign::Center,
            Some(RGBA::from_u8(200, 0, 0, 255)),
        );
        ctx.print_centered(23, "Press (P) to start playing !");
        ctx.print_centered(24, "Press (Q) to quit the game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => self.quit(ctx),
                _ => {}
            }
        }
    }

    /// The play function, this is used to display the game's main mode, while
    /// the state's mode is set to Play, from there the player can control the
    /// character and avoid the asteroids while trying to land on the landing
    /// pad
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // Clear the screen
        ctx.print(0, 1, format!("Score: {}", self.score)); // Print the score

        let input = INPUT.lock(); // Get the input state to check for key presses
        self.frame_time += ctx.frame_time_ms; // Add the time passed in ms between the main loop iterations to the frame_time

        // Check for menu key presses (Not gameplay-related) to pause or quit the game
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.mode = Gamemode::Pause,
                VirtualKeyCode::Q => self.mode = Gamemode::Quit,
                _ => {}
            }
        }

        // All the following actions only occurs if the frame_time is greater than the FRAME_DURATION, so these events speed are kept in check
        // by the FRAME_DURATION constant.
        if self.frame_time > FRAME_DURATION {
            // Check for key presses to control the character's ship, thrusting in the corresponding direction, with the use of event_queue
            // it is possible to move in multiple directions at once
            input.key_pressed_set().iter().for_each(|&key| match key {
                VirtualKeyCode::Up => self.character.thrust(character::Direction::Up, 0.42, 1.0),
                VirtualKeyCode::Left => self.character.thrust(character::Direction::Left, 0.3, 1.0),

                VirtualKeyCode::Right => {
                    self.character.thrust(character::Direction::Right, 0.3, 1.0);
                }
                _ => {}
            });

            // Apply gravity and drag to the character's ship, then apply momentum to the ship
            self.character.apply_gravity_and_drag(0.4, 0.2);
            self.character.apply_momentum();

            // Update the asteroids' position and check for collisions
            self.asteroids
                .iter_mut()
                .for_each(|asteroid| asteroid.update());

            // Check if the number of asteroid is less than the score, if so, add a new asteroid to the game, this is the project's
            // way of increasing the difficulty as the player's score increases
            let asteroid_number = self.asteroids.len();
            if asteroid_number < self.score {
                let mut random = RandomNumberGenerator::new();
                let starting_point = random.range(1.0, SCREEN_WIDTH as f32 - 1.0);
                self.asteroids
                    .push(hazards::Asteroid::new(starting_point, 0.0));
            }

            self.frame_time = 0.0;
        }

        // Keep the character's ship inside the window
        self.character.coordinate.x =
            State::keep_in_bounds(self.character.coordinate.x, 0, SCREEN_WIDTH - 1);
        self.character.coordinate.y =
            State::keep_in_bounds(self.character.coordinate.y, 1, SCREEN_HEIGHT);

        // Check if the character's ship has landed on the landing pad
        if self.landing_pad.check_landing(&mut self.character) {
            self.character.coordinate.y = (SCREEN_HEIGHT - self.landing_pad.heigth) as f32;
        }

        // Check for asteroid which left the screen and increase the avoided counter, if the avoided counter is greater than the number of
        // asteroid, increase the score and reset the avoided counter
        self.asteroids.iter_mut().for_each(|asteroid| {
            if asteroid.check_finished_course(SCREEN_WIDTH, SCREEN_HEIGHT) {
                let mut random = RandomNumberGenerator::new();
                let starting_point = random.range(1.0, SCREEN_WIDTH as f32 - 1.0);
                *asteroid = hazards::Asteroid::new(starting_point, 0.0);
                self.avoided += 1;
            }
        });
        if self.avoided > self.asteroids.len() {
            self.score += 1;
            self.avoided = 0;
        }

        // Check for collision between the character's ship and the asteroids, if so, set the game's mode to Menu
        self.asteroids.iter_mut().for_each(|asteroid| {
            if asteroid.check_collision(&mut self.character) {
                self.mode = Gamemode::Lost;
            }
        });

        // Clears the fancy console (The console which contains fancy-rendred element such as the character's ship and the asteroids) to
        // prepare for the next frame.
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(0);

        // Render the character's ship, the asteroids and the landing pad
        self.character.render(ctx);

        self.asteroids
            .iter_mut()
            .for_each(|asteroid| asteroid.render(ctx));

        self.landing_pad.render(ctx);
    }

    /// The pause function, this is used to display the game's pause mode, while
    /// in pause mode the game clears the screen and waits for input from the
    /// player to resume playing or quit the game
    fn pause(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(0);
        ctx.cls();
        ctx.print_centered(20, "Game Paused");
        ctx.print_centered(23, "Press (P) to resume playing !");
        ctx.print_centered(24, "Press (Q) to quit the game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.mode = Gamemode::Play,
                VirtualKeyCode::Q => self.mode = Gamemode::Quit,
                _ => {}
            }
        }
    }

    /// The lost function, this is used to display the game's lost mode, while
    /// This occurs when a collistion is made, the game clears the screen and
    /// displays the player's score and waits for input from the player to
    /// restart the game or quit the game
    fn lost(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(20, "You lost !");
        ctx.print_centered(22, format!("Your score is : {}", self.score));
        ctx.print_centered(24, "Press (P) to restart the game");
        ctx.print_centered(25, "Press (Q) to quit the game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => self.mode = Gamemode::Quit,
                _ => {}
            }
        }
    }

    /// The quit function, this is used to quit the game using the main loop's
    /// quitting flag
    fn quit(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.quitting = true;
    }

    /// The restart function, this is used to restart the game, ounce the player looses
    /// It resets the state struct to its initial state except for the game_state that is kept
    /// to Play
    fn restart(&mut self) {
        self.character = character::Character::new((SCREEN_WIDTH / 2) as f32, SCREEN_HEIGHT as f32);
        let mut random = RandomNumberGenerator::new();
        self.asteroids = Vec::new();
        let starting_point = random.range(0.0, SCREEN_WIDTH as f32);
        self.asteroids
            .push(hazards::Asteroid::new(starting_point, 0.0));
        self.frame_time = 0.0;
        self.avoided = 0;
        self.landing_pad = map::LandingPad::new(random.range(0, SCREEN_WIDTH));
        self.score = 0;
        self.mode = Gamemode::Play;
    }

    /// The keep_in_bounds function, this is used to keep the character's ship
    /// inside the window generated by Btermbuilder.
    fn keep_in_bounds(coordinate: f32, min: i32, max: i32) -> f32 {
        if coordinate > max as f32 {
            return max as f32;
        }

        if coordinate < min as f32 {
            return min as f32;
        }
        coordinate
    }
}

/// The implementation of the GameState trait for the State struct. It is imperative for
/// the state struct to implement the GameState trait in order to be used in the main loop.
/// This specific implementation checks for a particular variant of the gamemode enum and launches
/// the appropriate state's method.
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            Gamemode::Menu => self.main_menu(ctx),
            Gamemode::Play => self.play(ctx),
            Gamemode::Pause => self.pause(ctx),
            Gamemode::Quit => self.quit(ctx),
            Gamemode::Lost => self.lost(ctx),
        }
    }
}

/// The main function of the game, this only builds the game's window with these specific
/// settings :
///
/// - .with_fancy_console : to allow for smoother movement, without it the movement used in-game would be with integer
///     coordinates
/// - .with_title : to set the window's title to "Game Project"
/// - .with_vsync : to disable vertical synchronization
/// - .with_advanced_input : to enable event_queue, this is important so that the gamestate can handle multiple inputs at once.
/// - .with_fps_cap : to set the game's frame rate to 60.0
/// - .with_fullscreen : to set the game to fullscreen
///
/// It then launches the main loop with the State::new() as the game's state
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_fancy_console(80, 50, "terminal8x8.png")
        .with_title("Rust Rover")
        .with_vsync(false)
        .with_advanced_input(true)
        .with_fps_cap(60.0)
        .with_fullscreen(true)
        .build()?;

    main_loop(context, State::new())
}
