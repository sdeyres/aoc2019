use std::{fmt, ops};

use regex::Regex;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Vec3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl From<&str> for Vec3D {
    fn from(value: &str) -> Self {
        let regex = Regex::new(r"<x=(?<x>-?\d+), y=(?<y>-?\d+), z=(?<z>-?\d+)>").unwrap();
        if let Some(captures) = regex.captures(value) {
            Self::new(
                captures["x"].parse().unwrap(),
                captures["y"].parse().unwrap(),
                captures["z"].parse().unwrap(),
            )
        } else {
            Self::default()
        }
    }
}

impl fmt::Display for Vec3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<x={}, y={}, z={}>", self.x, self.y, self.z)
    }
}

impl ops::Add for &Vec3D {
    type Output = Vec3D;

    fn add(self, rhs: &Vec3D) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<Vec3D> for &Vec3D {
    type Output = Vec3D;

    fn add(self, rhs: Vec3D) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add for Vec3D {
    type Output = Vec3D;
    
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<&Vec3D> for Vec3D {
    type Output =  Vec3D;

    fn add(self, rhs: &Vec3D) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Vec3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::AddAssign<&Vec3D> for Vec3D {
    fn add_assign(&mut self, rhs: &Vec3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<&Vec3D> for &Vec3D {
    type Output = Vec3D;

    fn sub(self, rhs: &Vec3D) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Neg for &Vec3D {
    type Output = Vec3D;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vec3D {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn signum(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
        }
    }

    pub fn energy(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        let vec = Vec3D::new(-1, 0, 2);
        assert_eq!(String::from("<x=-1, y=0, z=2>"), format!("{}", vec));
    }

    #[test]
    fn test_parse() {
        let vec = Vec3D::from("<x=-1, y=0, z=2>");
        assert_eq!(Vec3D::new(-1, 0, 2), vec);
    }
}
