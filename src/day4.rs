use regex::Regex;

pub fn solve_part_one(test: bool) {
    let (start, end) = load_data(test);
    let count = (start..=end).filter(|i| meets_criteria(*i)).count();
    println!("Part 1: {}", count);
}
pub fn solve_part_two(test: bool) {
    let (start, end) = load_data(test);
    let count = (start..=end).filter(|i| meets_criteria_part_2(*i)).count();
    println!("Part 2: {}", count);
}

fn meets_criteria(value: u32) -> bool {
    let digits: Vec<u32> = value
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    (1..digits.len()).all(|i| digits[i] >= digits[i - 1]) // only increasing digits
        && (1..digits.len()).any(|i| digits[i] == digits[i - 1]) // two consecutive identical digits
}

fn meets_criteria_part_2(value: u32) -> bool {
    let digits: Vec<u32> = value
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    (1..digits.len()).all(|i| digits[i] >= digits[i - 1]) // only increasing digits
        && (1..digits.len()).any(|i| {
            let mut result = digits[i] == digits[i - 1];
            if i > 1 {
                result = result && digits[i] != digits[i - 2];
            }
            if i < digits.len() - 1 {
                result = result && digits[i] != digits[i + 1];
            }
            result
        }) // at least one group of two consecutive identical digits, but not more
}

fn load_data(test: bool) -> (u32, u32) {
    let input = aoc2019::load_data(4, test);
    let range = input.trim_end();
    let regex = Regex::new("^(\\d+)-(\\d+)$").unwrap();
    let captures = regex.captures(range).unwrap();
    (
        captures[1].parse::<u32>().unwrap(),
        captures[2].parse::<u32>().unwrap(),
    )
}
