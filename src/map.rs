use crate::character::Character;
use crate::SCREEN_HEIGHT;
use bracket_lib::prelude::*;

/// The LandingPad struct is used to create and manage the landing pads that the character will be trying to land on.
/// The landing pads are the goal of the game, and the character must land on them to win.
pub struct LandingPad {
    x: i32,
    y: i32,
    pub heigth: i32,
    pad_width: i32,
}

impl LandingPad {
    /// The new function is used to create a new instance of the LandingPad struct.
    /// It takes in an x coordinate and returns a new instance of the LandingPad struct.
    /// The height and width of the landing pad are randomly generated.
    pub fn new(x: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        let height = random.range(SCREEN_HEIGHT / 4, SCREEN_HEIGHT / 2);
        let mut pad_width = random.range(3, 11);
        if pad_width % 2 == 0 {
            pad_width += 1;
        }
        LandingPad {
            x,
            y: SCREEN_HEIGHT,
            heigth: height,
            pad_width,
        }
    }

    /// The render function is used to draw the landing pad to the screen. Since it uses floating-point fancy-rendering, it must
    /// first shift on the fancy-console (id 1), before drawing the landing pad, and then shift back to the main console (id 0).
    pub fn render(&mut self, ctx: &mut BTerm) {
        for i in 0..self.heigth {
            ctx.set(self.x, self.y - i, GREEN, BLACK, to_cp437('H'));
        }
        for i in 0..self.pad_width {
            ctx.set(
                self.x - i + (self.pad_width / 2),
                self.y - self.heigth,
                GREEN,
                BLACK,
                to_cp437('='),
            );
        }
    }

    /// The check_landing function is used to check if the character has landed on the landing pad.
    /// It takes in a mutable reference to a Character and returns a boolean.
    /// For the moment, landing on the pad does nothing execpt keeping you from falling.
    pub fn check_landing(&self, character: &mut Character) -> bool {
        if character.coordinate.x > (self.x - (self.pad_width / 2 + 1)) as f32
            && character.coordinate.x < (self.x + (self.pad_width / 2 + 1)) as f32
            && character.coordinate.y as i32 == (self.y - self.heigth)
        {
            return true;
        }
        false
    }
}
