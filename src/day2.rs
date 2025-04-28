use crate::intcode::Intcode;

pub fn solve_part_one(test: bool) {
    let mut program = load_data(test);

    program.execute();

    println!("Part 1: {}", program[0]);
}

pub fn solve_part_two(test: bool) {
    let initial_program = load_data(test);

    let mut noun = 0;
    let mut verb = 0;

    while noun <= 99 {
        while verb <= 99 {
            let mut program = initial_program.clone();
            program[1] = noun;
            program[2] = verb;
            program.execute();

            if program[0] == 19690720 {
                println!("Part 2: {}", 100 * noun + verb);
                return;
            }
            verb += 1;
        }
        noun += 1;
        verb = 0;
    }
}

fn load_data(test: bool) -> Intcode {
    let mut program = Intcode::from(aoc2019::load_data(2, test).trim_end());

    if !test {
        program[1] = 12;
        program[2] = 2;
    }

    program
}
