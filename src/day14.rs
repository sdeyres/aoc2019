use std::{collections::HashMap, ops};

use regex::Regex;

pub fn solve_part_one(test: bool) {
    let reactions = load_data(test);
    let ore = fuel_cost(&reactions, 1);
    println!("Part 1: {}", ore);
}

pub fn solve_part_two(test: bool) {
    let reactions = load_data(test);
    let max_fuel = max_fuel(&reactions, 1_000_000_000_000);
    println!("Part 2: {}", max_fuel);
}

#[derive(Debug)]
struct Reactions {
    reactions: HashMap<String, Reaction>,
}

impl From<String> for Reactions {
    fn from(value: String) -> Self {
        let reactions = value
            .lines()
            .map(|line| {
                let reaction = Reaction::from(line);
                (reaction.produced.element.clone(), reaction)
            })
            .collect();

        Self { reactions }
    }
}

fn fuel_cost(reactions: &Reactions, fuel_amount: u64) -> u64 {
    let fuel = Ingredient::new(String::from("FUEL"), fuel_amount);
    let mut ore = 0;
    let mut requirement_list = vec![fuel];
    let mut leftovers: HashMap<String, u64> = HashMap::new();

    while let Some(required) = requirement_list.pop() {
        if required.element == "ORE" {
            ore += required.quantity;
        } else {
            let remaining = try_leftovers(&required, &mut leftovers);

            if remaining != 0 {
                let reaction = reactions
                    .reactions
                    .get(&required.element)
                    .expect("Reaction to produce ingredient does not exist");
                let reaction_multiplier =
                    (remaining as f64 / reaction.produced.quantity as f64).ceil() as u64;

                if let Some(result) =
                    (reaction.produced.quantity * reaction_multiplier).checked_sub(remaining)
                {
                    if result > 0 {
                        *leftovers.entry(required.element).or_insert(0) += result;
                    }
                }

                for requirement in &reaction.required {
                    requirement_list.push(requirement * reaction_multiplier);
                }
            }
        }
    }

    ore
}

// Check if ingredient can be taken from leftovers.
// Possible scenarios:
//     1. There is enough of the ingredient in the leftovers, quantity is substracted from leftovers.
//     2. There isn't any of the ingredient in the leftovers, quantity needs to be produced.
//     3. There is some of the ingredient in the leftovers, but not enough to cover the needs. Existing quantity is consumed, and remaining quantity to prduce is returned.
// Returns the quantity of ingredient to produce after consuming leftovers.
fn try_leftovers(product: &Ingredient, leftovers: &mut HashMap<String, u64>) -> u64 {
    if let Some(quantity) = leftovers.get_mut(&product.element) {
        if product.quantity <= *quantity {
            // Case 1
            *quantity -= product.quantity;
            0
        } else {
            // Case 3
            let remaining = product.quantity - *quantity;
            leftovers.remove(&product.element);
            remaining
        }
    } else {
        // Case 2
        product.quantity
    }
}

fn max_fuel(reactions: &Reactions, available_ore: u64) -> u64 {
    let cost_one = fuel_cost(reactions, 1);
    let mut fuel_left = available_ore / cost_one;
    let mut fuel_right = fuel_left * 2;

    while fuel_right - fuel_left > 1 {
        let fuel = (fuel_left + fuel_right) / 2;
        let cost = fuel_cost(reactions, fuel);

        if cost < available_ore {
            fuel_left = fuel;
        } else {
            fuel_right = fuel;
        }
    }

    fuel_left
}

#[derive(Debug)]
struct Reaction {
    produced: Ingredient,
    required: Vec<Ingredient>,
}

impl From<&str> for Reaction {
    fn from(value: &str) -> Self {
        let regex = Regex::new(r"(?<required>.+) => (?<produced>.+)").unwrap();
        if let Some(matches) = regex.captures(value) {
            let produced = Ingredient::from(&matches["produced"]);
            let required = matches["required"]
                .split(", ")
                .map(Ingredient::from)
                .collect();

            Self { produced, required }
        } else {
            panic!()
        }
    }
}

#[derive(Debug)]
struct Ingredient {
    element: String,
    quantity: u64,
}

impl From<&str> for Ingredient {
    fn from(value: &str) -> Self {
        let regex = Regex::new(r"(?<qty>\d+) (?<elem>\w+)").unwrap();
        if let Some(matches) = regex.captures(value) {
            let quantity = matches["qty"].parse().unwrap();
            let element = matches["elem"].to_owned();

            Self { element, quantity }
        } else {
            panic!("Unknown ingredient format")
        }
    }
}

impl ops::Mul<u64> for &Ingredient {
    type Output = Ingredient;

    fn mul(self, rhs: u64) -> Self::Output {
        Ingredient {
            element: self.element.clone(),
            quantity: self.quantity * rhs,
        }
    }
}

impl Ingredient {
    fn new(element: String, quantity: u64) -> Self {
        Ingredient { element, quantity }
    }
}

fn load_data(test: bool) -> Reactions {
    Reactions::from(aoc2019::load_data(14, test))
}
