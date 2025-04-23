use crate::intcode::Intcode;

pub fn solve_part_one(test: bool) {
    let mut program = load_data(test);
    println!("{:#?}", program);

    program.execute();

    println!("Part 1: {}", program.get(0));
}

pub fn solve_part_two(test: bool) {
    let initial_program = load_data(test);

    let mut noun = 0;
    let mut verb = 0;

    while noun <= 99 {
        while verb <= 99 {
            let mut program = initial_program.clone();
            program.set(1, noun);
            program.set(2, verb);
            program.execute();

            if program.get(0) == 19690720 {
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
        program.set(1, 12);
        program.set(2, 2);    
    }

    program
}
