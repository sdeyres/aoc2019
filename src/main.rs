mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod intcode;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(about)]
struct Args {
    #[arg(short, long)]
    test: bool,
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let args = Args::parse();
    match args.day {
        1 => {
            day1::solve_part_one(args.test);
            day1::solve_part_two(args.test);
        }
        2 => {
            day2::solve_part_one(args.test);
            day2::solve_part_two(args.test);
        }
        3 => {
            day3::solve_part_one(args.test);
            day3::solve_part_two(args.test);
        }
        4 => {
            day4::solve_part_one(args.test);
            day4::solve_part_two(args.test);
        }
        5 => {
            day5::solve_part_one(args.test);
            day5::solve_part_two(args.test);
        }
        6 => {
            day6::solve_part_one(args.test);
            day6::solve_part_two(args.test);
        }
        7 => {
            day7::solve_part_one(args.test);
            day7::solve_part_two(args.test);
        }
        8 => {
            day8::solve_part_one(args.test);
            day8::solve_part_two(args.test);
        }
        9 => {
            day9::solve_part_one(args.test);
            day9::solve_part_two(args.test);
        }
        _ => {}
    }
}
