use aoc2019::Intcode;

pub fn solve_part_one(test: bool) {
    let mut program = load_data(test);
    program.add_input(1);
    let mut output = 0;
    loop {
        program.execute();
        if let Some(o) = program.next_output() {
            output = o;
        } else {
            break;
        }
    }
    println!("Part 1: {}", output);
}

pub fn solve_part_two(test: bool) {
    let mut program = load_data(test);
    program.add_input(5);
    program.execute();
    println!("Part 2: {}", program.next_output().unwrap());
}

fn load_data(test: bool) -> Intcode {
    Intcode::parse(aoc2019::load_data(5, test).trim())
}
