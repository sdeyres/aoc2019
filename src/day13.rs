use std::{cmp::Ordering, collections::HashMap, fmt};

use crate::{coordinates::Coordinates, intcode::Intcode};

pub fn solve_part_one(test: bool) {
    let mut program = load_data(test);
    let mut screen = Screen::new();
    screen.fill(&mut program);
    let block_count = screen
        .tiles
        .values()
        .filter(|tile| matches!(tile, Tile::Block))
        .count();
    println!("Part 1: {}", block_count);
}

pub fn solve_part_two(test: bool) {
    let mut program = load_data(test);
    program[0] = 2;
    let mut screen = Screen::new();
    let score = screen.fill(&mut program);
    println!("Part 2: {}", score);
}

struct Screen {
    tiles: HashMap<Coordinates, Tile>,
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x_max = self.tiles.keys().map(|c| c.get_x()).max().unwrap();
        let y_max = self.tiles.keys().map(|c| c.get_y()).max().unwrap();

        for y in 0..=y_max {
            for x in 0..=x_max {
                if let Some(tile) = self.tiles.get(&Coordinates::new(x, y)) {
                    match tile {
                        Tile::Empty => write!(f, " ")?,
                        Tile::Wall => write!(f, "*")?,
                        Tile::Block => write!(f, "#")?,
                        Tile::Paddle => write!(f, "_")?,
                        Tile::Ball => write!(f, ".")?,
                    }
                } else {
                    write!(f, "?")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Screen {
    fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    fn draw(&mut self, pixel: Coordinates, tile: Tile) {
        self.tiles.insert(pixel, tile);
    }

    fn fill(&mut self, program: &mut Intcode) -> i64 {
        let mut score = 0;
        let mut paddle_x = 0;

        loop {
            program.execute();
            program.execute();
            program.execute();
            if let Some(x) = program.next_output() {
                if x == -1 {
                    _ = program.next_output();
                    score = program
                        .next_output()
                        .expect("No corresponding score output");
                } else {
                    let y = program.next_output().expect("No corresponding y output");
                    let tile = program.next_output().expect("No corresponding tile output");

                    let pixel = Coordinates::new(x as i32, y as i32);
                    let tile = Tile::from(tile);

                    match tile {
                        Tile::Ball => match x.cmp(&paddle_x) {
                            Ordering::Equal => program.add_input(0),
                            Ordering::Greater => program.add_input(1),
                            Ordering::Less => program.add_input(-1),
                        },
                        Tile::Paddle => {
                            paddle_x = x;
                        }
                        _ => (),
                    }

                    self.draw(pixel, tile);
                }
            } else {
                break;
            }
        }

        score
    }
}

enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<i64> for Tile {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::Paddle,
            4 => Self::Ball,
            _ => panic!("Unknown tile"),
        }
    }
}

fn load_data(test: bool) -> Intcode {
    Intcode::from(aoc2019::load_data(13, test).trim())
}
