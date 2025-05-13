use crate::coordinates::Coordinates;

#[derive(Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn next(&self, coordinates: &Coordinates) -> Coordinates {
        let velocity = match self {
            Direction::Up => Coordinates::new(0, 1),
            Direction::Right => Coordinates::new(1, 0),
            Direction::Down => Coordinates::new(0, -1),
            Direction::Left => Coordinates::new(-1, 0),
        };

        coordinates + &velocity
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Up
    }
}