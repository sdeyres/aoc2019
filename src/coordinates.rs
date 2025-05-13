use std::{f64::consts::PI, ops};

use num::integer;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn normalize(&self) -> Coordinates {
        let gcd = integer::gcd(self.x, self.y);
        self / gcd
    }

    pub fn angle(&self) -> f64 {
        let result = (self.y as f64).atan2(self.x as f64);
        if result < 0.0 {
            result + 2.0 * PI
        } else {
            result
        }
    }

    pub fn magnitude(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }
}

impl ops::Div<i32> for &Coordinates {
    type Output = Coordinates;

    fn div(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::Add<&Coordinates> for &Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: &Coordinates) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Mul<i32> for &Coordinates {
    type Output = Coordinates;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: rhs * self.x,
            y: rhs * self.y,
        }
    }
}

impl ops::Sub<&Coordinates> for &Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: &Coordinates) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Default for Coordinates {
    fn default() -> Self {
        Self::new(0, 0)
    }
}
