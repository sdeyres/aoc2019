use aoc2019::Intcode;

pub fn solve_part_one(test: bool) {
    let mut program = load_data(test);
    program.add_input(1);
    program.execute();
    while let Some(output) = program.next_output() {
        println!("Output: {}", output);
        program.execute();
    }
}

pub fn solve_part_two(test: bool) {}

fn load_data(test: bool) -> Intcode {
    Intcode::parse(aoc2019::load_data(9, test).trim())
}