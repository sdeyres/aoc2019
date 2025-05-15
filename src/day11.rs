use std::{collections::HashMap, fmt::Display};

use crate::{coordinates::Coordinates, direction::Direction, intcode::Intcode};

pub fn solve_part_one(test: bool) {
    let program = load_data(test);
    let mut robot = Robot::new(program);
    let mut hull = Hull::new();

    robot.paint(&mut hull);
    println!("Part 1: {}", hull.surface());
}

pub fn solve_part_two(test: bool) {
    let program = load_data(test);
    let mut robot = Robot::new(program);
    let mut hull = Hull::new();
    hull.set(&Coordinates::default(), 1);

    robot.paint(&mut hull);
    println!("Part 2:");
    println!("{}", hull);
}

struct Robot {
    program: Intcode,
    position: Coordinates,
    direction: Direction,
}

impl Robot {
    fn new(program: Intcode) -> Self {
        Self {
            program,
            position: Coordinates::default(),
            direction: Direction::default(),
        }
    }

    fn paint(&mut self, hull: &mut Hull) {
        loop {
            let old_color = hull.get(&self.position);
            self.program.add_input(old_color);
            self.program.execute();
            if let Some(new_color) = self.program.next_output() {
                hull.set(&self.position, new_color);
                self.program.execute();
                match self.program.next_output().unwrap() {
                    0 => self.direction = self.direction.turn_left(),
                    1 => self.direction = self.direction.turn_right(),
                    _ => panic!("Unknown direction"),
                }
            } else {
                break;
            }
            self.position = self.direction.next(&self.position);
        }
    }
}

struct Hull {
    hull: HashMap<Coordinates, i64>,
}

impl Hull {
    fn new() -> Self {
        Hull {
            hull: HashMap::new(),
        }
    }

    fn get(&self, coord: &Coordinates) -> i64 {
        *self.hull.get(coord).unwrap_or(&0)
    }

    fn set(&mut self, coord: &Coordinates, color: i64) {
        self.hull.insert(*coord, color);
    }

    fn surface(&self) -> usize {
        self.hull.len()
    }
}

impl Display for Hull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_min = self.hull.keys().map(|c| c.get_x()).min().unwrap();
        let x_max = self.hull.keys().map(|c| c.get_x()).max().unwrap();
        let y_min = self.hull.keys().map(|c| c.get_y()).min().unwrap();
        let y_max = self.hull.keys().map(|c| c.get_y()).max().unwrap();

        for y in (y_min..=y_max).rev() {
            for x in x_min..=x_max {
                let color = self.get(&Coordinates::new(x, y));
                write!(f, "{}", if color == 1 { '#' } else { ' ' })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn load_data(test: bool) -> Intcode {
    Intcode::from(aoc2019::load_data(11, test).trim())
}