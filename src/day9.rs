use crate::intcode::Intcode;

pub fn solve_part_one(test: bool) {
    let mut program = load_data(test);
    program.add_input(1);
    program.execute();
    while let Some(output) = program.next_output() {
        println!("Part 1: {}", output);
        program.execute();
    }
}

pub fn solve_part_two(test: bool) {
    let mut program = load_data(test);
    program.add_input(2);
    program.execute();
    println!("Part 2: {}", program.next_output().unwrap());
}

fn load_data(test: bool) -> Intcode {
    Intcode::from(aoc2019::load_data(9, test).trim())
}
