use itertools::Itertools;

use crate::intcode::Intcode;

pub fn solve_part_one(test: bool) {
    let master_program = load_data(test);
    let result = (0..=4)
        .permutations(5)
        .map(|settings| {
            let mut output = 0;

            for phase in settings {
                let mut program = master_program.clone();
                program.add_input(phase);
                program.add_input(output);
                program.execute();
                output = program.next_output().unwrap();
            }

            output
        })
        .max()
        .unwrap();

    println!("Part 1: {}", result);
}

pub fn solve_part_two(test: bool) {
    let master_program = load_data(test);
    let result = (5..10)
        .permutations(5)
        .map(|settings| {
            // Initialize amplifiers
            let mut amplifiers: Vec<Intcode> = (0..5)
                .map(|i| {
                    let mut amplifier = master_program.clone();
                    amplifier.add_input(settings[i]);
                    amplifier
                })
                .collect();

            let mut output = 0;
            let mut next = 0;
            let mut amplifier = &mut amplifiers[next % 5];
            loop {
                amplifier.add_input(output);
                amplifier.execute();
                if let Some(o) = amplifier.next_output() {
                    output = o;
                    next += 1;
                    amplifier = &mut amplifiers[next % 5];
                } else {
                    break;
                }
            }

            output
        })
        .max()
        .unwrap();

    println!("Part 2: {}", result);
}

fn load_data(test: bool) -> Intcode {
    Intcode::from(aoc2019::load_data(7, test).trim())
}
