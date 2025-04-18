use std::collections::HashMap;

use array_tool::vec::Intersect;
use itertools::Itertools;

pub fn solve_part_one(test: bool) {
    let orbits = load_data(test);

    let mut orbit_count = 0;
    for orbit in &orbits {
        orbit_count += 1;
        let mut parent = orbit.1;
        while orbits.contains_key(parent) {
            orbit_count += 1;
            parent = orbits.get(parent).unwrap();
        }
    }
    println!("Part 1: {}", orbit_count);
}

pub fn solve_part_two(test: bool) {
    let orbits = load_data(test);

    let you_path = get_path_to_com(&"YOU".to_string(), &orbits);
    let santa_path = get_path_to_com(&"SAN".to_string(), &orbits);
    let intersection = you_path.intersect(santa_path.clone());

    let distance_from_you_to_intersection =
        you_path.iter().position(|x| *x == intersection[0]).unwrap();
    let distance_from_intersection_to_santa = santa_path
        .iter()
        .position(|x| *x == intersection[0])
        .unwrap();
    println!(
        "Part 2: {}",
        distance_from_you_to_intersection + distance_from_intersection_to_santa
    );
}

fn get_path_to_com<'a>(object: &String, orbits: &'a HashMap<String, String>) -> Vec<&'a String> {
    let mut path = Vec::new();
    let mut orbiter = object;

    while let Some(parent) = orbits.get(orbiter) {
        path.push(parent);
        orbiter = parent;
    }

    path
}

fn load_data(test: bool) -> HashMap<String, String> {
    aoc2019::load_data(6, test)
        .trim()
        .lines()
        .flat_map(|s| s.rsplit(")"))
        .map(String::from)
        .tuples()
        .collect()
}
