pub fn solve_part_one(test: bool) {
    let weights = load_data(test);
    let fuel = weights.into_iter().map(get_required_fuel).sum::<u32>();
    println!("Part 1 - Fuel needed: {}", fuel);
}

pub fn solve_part_two(test: bool) {
    let weights = load_data(test);
    let fuel = weights.into_iter().map(|w| {
        let mut total_fuel = get_required_fuel(w);
        let mut fuel = get_required_fuel(total_fuel);
        while fuel != 0 {
            total_fuel += fuel;
            fuel = get_required_fuel(fuel);
        }
        total_fuel
    }).sum::<u32>();
    println!("Part 2 - Fuel needed: {}", fuel);
}

fn get_required_fuel(weight: u32) -> u32 {
    if weight < 9 {
        0
    } else {
        weight / 3 - 2
    }
}

fn load_data(test: bool) -> Vec<u32> {
    aoc2019::load_data(1, test)
        .lines()
        .map(|line| line.parse::<u32>().expect("Oops..."))
        .collect()
}
