use std::cmp::Ordering;
use bracket_lib::prelude::*;

/// The Direction enum is used to represent the possible directions the character can move in.
/// It is mainly used as a parameter to the thrust function of the Character struct.
pub enum Direction {
    Up,
    Left,
    Right,
}

/// The Momentum struct is used to calculate the ship's movement.
#[derive(PartialEq, PartialOrd)]
pub struct Momentum {
    /// Vertical is applied to the y coordinate of the ship.
    pub vertical: f32,

    /// Horizontal is applied to the x coordinate of the ship.
    pub horizontal: f32,
}

impl Momentum {
    /// The new function is used to create a new instance of the Momentum struct. set by default at 0.
    fn new() -> Self {
        Momentum {
            vertical: 0.0,
            horizontal: 0.0,
        }
    }
}

/// The Character struct is used to create and manage the ship that the player will be controlling.
pub struct Character {
    /// The coordinate field is used to store the position of the ship in floating point coordinates to allow for fancy rendering.
    pub coordinate: PointF,
    /// The orientation field is used to store the rotation of the ship, it is necessary for fancy-rendering, but it is currently unused.
    orientation: Degrees,
    /// The scaling field is used to store the size of the ship, it is necessary for fancy-rendering, but it is currently unused.
    scaling: PointF,
    /// The momentum field is used to store a momentum struct that is used to calculate the ship's movement.
    pub momentum: Momentum,
}

impl Character {
    /// The new function is used to create a new instance of the Character struct.
    pub fn new(x: f32, y: f32) -> Self {
        Character {
            coordinate: PointF::new(x, y),
            orientation: Degrees::new(0.0),
            scaling: PointF::new(1.0, 1.0),
            momentum: Momentum::new(),
        }
    }

    /// The render function is used to draw the ship to the screen. Since it uses floating-point fancy-rendering, it must
    /// first shift on the fancy-console (id 1), before drawing the ship, and then shift back to the main console (id 0).
    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);

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

    /// The thrust function is used to apply a force to the ship in the direction specified by the direction parameter.
    /// depending on the direction parameter, the function will add, or substract to the ship's vertical or horizontal momentum.
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

    /// This function updates the ship's position using the momentum struct.
    pub fn apply_momentum(&mut self) {
        self.coordinate.x += self.momentum.horizontal;
        self.coordinate.y += self.momentum.vertical;
    }

    /// This function applies gravity and drag to the ship's momentum. It takes in a gravity and a drag parameter.
    /// The gravity parameter represent's the force pulling the ship down, and the drag parameter allows for a sort of
    /// inertia in horizontal movement.
    pub fn apply_gravity_and_drag(&mut self, gravity: f32, drag: f32) {
        match self.momentum.vertical.partial_cmp(&gravity) {
            Some(Ordering::Greater) => self.momentum.vertical = gravity,
            Some(Ordering::Less) => self.momentum.vertical += gravity,
            _ => {}
        }
        match self.momentum.horizontal.partial_cmp(&0.0) {
            Some(Ordering::Greater) => self.momentum.horizontal -= drag,
            Some(Ordering::Less) => self.momentum.horizontal += drag,
            _ => {}
        }
        if self.momentum.horizontal > -0.05 && self.momentum.horizontal < 0.05 {
            self.momentum.horizontal = 0.0;
        }
    }
}
