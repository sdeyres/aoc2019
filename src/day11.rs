use crate::intcode::Intcode;

pub fn solve_part_one(test: bool) {
    let mut program = load_data(test);
    program.execute();
    while let Some(color) = program.next_output() {
        println!("Color: {}", color);
        program.execute();
        println!("Direction: {}", program.next_output().unwrap());
        program.execute();
    }
}
pub fn solve_part_two(test: bool) {}

fn load_data(test: bool) -> Intcode {
    Intcode::from(aoc2019::load_data(11, test).trim())
}