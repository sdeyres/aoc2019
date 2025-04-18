use std::collections::HashSet;

use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag, character::complete::digit1,
    combinator::map, multi::separated_list1,
};

pub fn solve_part_one(test: bool) {
    let wires = load_data(test);
    let closest_intersection_distance: i32 = wires[0]
        .intersections(&wires[1])
        .iter()
        .map(manhattan_distance)
        .min()
        .unwrap();
    println!("Part 1: {}", closest_intersection_distance);
}

pub fn solve_part_two(test: bool) {
    let wires = load_data(test);
    let intersections = wires[0].intersections(&wires[1]);
    let closest_intersection_distance: usize = intersections
        .iter()
        .map(|intersection| -> usize {
            let dist1 = wires[0]
                .wire
                .iter()
                .position(|position| position == *intersection)
                .expect("Oops...");
            let dist2 = wires[1]
                .wire
                .iter()
                .position(|position| position == *intersection)
                .expect("Oops...");
            dist1 + dist2
        })
        .min()
        .unwrap();
    println!("Part 2: {}", closest_intersection_distance);
}

struct Wire {
    wire: Vec<(i32, i32)>,
}

impl Wire {
    pub fn new(directions: Vec<Direction>) -> Self {
        let mut wire = Vec::new();

        let mut position = (0, 0);
        wire.push(position);

        for direction in directions {
            match direction {
                Direction::Down(distance) => {
                    for _ in 0..distance {
                        position = (position.0, position.1 - 1);
                        wire.push(position);
                    }
                }
                Direction::Left(distance) => {
                    for _ in 0..distance {
                        position = (position.0 - 1, position.1);
                        wire.push(position);
                    }
                }
                Direction::Right(distance) => {
                    for _ in 0..distance {
                        position = (position.0 + 1, position.1);
                        wire.push(position);
                    }
                }
                Direction::Up(distance) => {
                    for _ in 0..distance {
                        position = (position.0, position.1 + 1);
                        wire.push(position);
                    }
                }
            }
        }

        Self { wire }
    }

    pub fn intersections<'a>(&'a self, other: &'a Wire) -> HashSet<&'a (i32, i32)> {
        let mut positions: HashSet<&(i32, i32)> = HashSet::new();
        self.wire
            .iter()
            .filter(|position| position != &&(0, 0))
            .for_each(|position| _ = positions.insert(position));
        other
            .wire
            .iter()
            .filter(|position| positions.contains(position))
            .collect()
    }
}

#[derive(Debug)]
enum Direction {
    Down(i32),
    Left(i32),
    Right(i32),
    Up(i32),
}

fn down(input: &str) -> IResult<&str, Direction> {
    map((tag("D"), digit1), |(_tag, distance): (&str, &str)| {
        let distance = distance.parse::<i32>().expect("Oops...");
        Direction::Down(distance)
    })
    .parse(input)
}

fn left(input: &str) -> IResult<&str, Direction> {
    map((tag("L"), digit1), |(_tag, distance): (&str, &str)| {
        let distance = distance.parse::<i32>().expect("Oops...");
        Direction::Left(distance)
    })
    .parse(input)
}

fn right(input: &str) -> IResult<&str, Direction> {
    map((tag("R"), digit1), |(_tag, distance): (&str, &str)| {
        let distance = distance.parse::<i32>().expect("Oops...");
        Direction::Right(distance)
    })
    .parse(input)
}

fn up(input: &str) -> IResult<&str, Direction> {
    map((tag("U"), digit1), |(_tag, distance): (&str, &str)| {
        let distance = distance.parse::<i32>().expect("Oops...");
        Direction::Up(distance)
    })
    .parse(input)
}

fn wire(input: &str) -> IResult<&str, Wire> {
    map(
        separated_list1(tag(","), alt((down, left, right, up))),
        Wire::new,
    )
    .parse(input)
}

fn manhattan_distance(position: &&(i32, i32)) -> i32 {
    position.0.abs() + position.1.abs()
}

fn load_data(test: bool) -> Vec<Wire> {
    aoc2019::load_data(3, test)
        .lines()
        .map(|l| wire(l).expect("Oops...").1)
        .collect()
}
