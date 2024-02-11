use crate::character::Character;
use bracket_lib::prelude::*;

/// The Asteroid struct is used to create and manage the asteroids that will be flying around the screen.
pub struct Asteroid {
    /// The coordinate field is used to store the position of the asteroid in floating point coordinates to allow for fancy rendering.
    coordinate: PointF,
    /// The trajectory field is used to store the angle at which the asteroid is traveling.
    trajectory: f32,
    /// The orientation field is used to store the rotation of the asteroid, to allow for a spinning flight pattern.
    orientation: Degrees,
    /// The scaling field is used to store the size of the asteroid.
    scaling: PointF,
    /// The speed field is used to store the speed at which the asteroid is traveling.
    speed: f32,
}

impl Asteroid {
    /// The new function is used to create a new instance of the Asteroid struct.
    /// It takes in an x and y coordinate and returns a new instance of the Asteroid struct.
    /// All asteroids start at the top of the screen ( y = 0 ) and have a random x coordinate.
    /// The size, speed, and trajectory are all randomly generated.
    pub fn new(x: f32, y: f32) -> Self {
        let mut random = RandomNumberGenerator::new();
        let size: f32 = random.range(2.0, 4.0);
        Asteroid {
            coordinate: PointF::new(x, y),
            trajectory: random.range(89.7, 90.3),
            orientation: Degrees::new(0.0),
            scaling: PointF::new(size, size),
            speed: random.range(0.2, 0.6),
        }
    }

    /// The render function is used to draw the asteroid to the screen. Since it uses floating-point fancy-rendering, it must
    /// first shift on the fancy-console (id 1), before drawing the asteroid, and then shift back to the main console (id 0).
    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);

        ctx.set_fancy(
            self.coordinate,
            1,
            self.orientation,
            self.scaling,
            RED,
            BLACK,
            to_cp437('@'),
        );
        ctx.set_active_console(0);
    }

    /// The update function is used to move the asteroid in the direction of its trajectory by using the cosine and sine of the trajectory
    /// to calculate the x and y components of the movement. It also updates the orientation of the asteroid to make it spin.
    pub fn update(&mut self) {
        self.coordinate.x += self.trajectory.cos() * self.speed;
        self.coordinate.y += self.trajectory.sin() * self.speed;
        self.orientation.0 += 0.2;
    }

    /// The check_finished_course function is used to check if the asteroid has left the screen. It takes in the x and y bounds of the screen
    /// and returns a boolean value. If the asteroid has left the screen, it returns true, otherwise it returns false.
    pub fn check_finished_course(&mut self, x_bound: i32, y_bound: i32) -> bool {
        if self.coordinate.x > x_bound as f32 {
            return true;
        }
        if self.coordinate.x < 0.0 {
            return true;
        }
        if self.coordinate.y > y_bound as f32 {
            return true;
        }
        false
    }

    /// The check_collision function is used to check if the asteroid has collided with the character. It takes in a mutable reference to the character
    /// and returns a boolean value. If the asteroid has collided with the character, it returns true, otherwise it returns false.
    pub fn check_collision(&mut self, character: &mut Character) -> bool {
        let x = self.coordinate.x - character.coordinate.x;
        let y = self.coordinate.y - character.coordinate.y;
        let distance = (x * x + y * y).sqrt();
        if distance < self.scaling.x / 2.0 {
            return true;
        }
        false
    }
}
