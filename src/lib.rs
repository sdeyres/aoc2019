use std::fs;

pub fn load_data(day: u8, test: bool) -> String {
    let path = if test {
        format!("rsc/day{}_test.txt", day)
    } else {
        format!("rsc/day{}.txt", day)
    };
    fs::read_to_string(path).unwrap()
}
/*
#[derive(Clone, Debug)]
pub struct Intcode {
    program: Vec<i64>,
    position: usize,
    relative_base: i64,
    memory: HashMap<usize, i64>,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
}

enum Parameter {
    Position(usize),
    Immediate(i64),
    Relative(i64),
}

impl Parameter {
    pub fn from_instruction(mode: u32, content: i64) -> Self {
        match mode {
            0 => Self::Position(content as usize),
            1 => Self::Immediate(content),
            2 => Self::Relative(content),
            _ => panic!("Unknown mode: {}", mode),
        }
    }

    pub fn value(&self, program: &Intcode) -> i64 {
        match self {
            Self::Position(address) => program.get(*address),
            Self::Immediate(value) => *value,
            Self::Relative(offset) => {
                println!("relative_base: {}, offset: {}", program.relative_base, offset);
                program.get((program.relative_base + offset) as usize)
            },
        }
    }
}

impl Intcode {
    pub fn parse(input: &str) -> Self {
        let program = input
            .split(",")
            .map(|i| i.parse::<i64>().expect("Couldn't parse intcode memory"))
            .collect();

        Self {
            program,
            position: 0,
            relative_base: 0,
            memory: HashMap::new(),
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    pub fn execute(&mut self) {
        while self.position < self.program.len() && self.get(self.position) != 99 {
            let instruction = self.get(self.position);
            let digits: Vec<u32> = format!("{instruction:0>5}")
                .chars()
                .map(|d| d.to_digit(10).unwrap())
                .collect();

            match digits[3..=4] {
                [0, 1] => {
                    let first = Parameter::from_instruction(digits[2], self.get(self.position + 1))
                        .value(self);
                    let second =
                        Parameter::from_instruction(digits[1], self.get(self.position + 2))
                            .value(self);
                    let target = self.get(self.position + 3) as usize;
                    self.set(target, first + second);
                    self.position += 4;
                }
                [0, 2] => {
                    let first = Parameter::from_instruction(digits[2], self.get(self.position + 1))
                        .value(self);
                    let second =
                        Parameter::from_instruction(digits[1], self.get(self.position + 2))
                            .value(self);
                    let target = self.get(self.position + 3) as usize;
                    self.set(target, first * second);
                    self.position += 4;
                }
                [0, 3] => {
                    let input = self.next_input();
                    let target = Parameter::from_instruction(digits[2], self.get(self.position + 1))
                        .value(self) as usize;
                    println!("instruction: {}, parameter: {}", self.program[self.position], self.program[self.position + 1]);
                    println!("target: {}", target);
                    self.set(target, input);
                    println!("content: {}", self.get(target));
                    self.position += 2;
                }
                [0, 4] => {
                    let first = Parameter::from_instruction(digits[2], self.get(self.position + 1))
                        .value(self);
                    self.output(first);
                    self.position += 2;
                    break;
                }
                [0, 5] => {
                    let first = Parameter::from_instruction(digits[2], self.get(self.position + 1))
                        .value(self);
                    let target = Parameter::from_instruction(digits[1], self.get(self.position + 2))
                        .value(self) as usize;
                    if first != 0 {
                        self.position = target;
                    } else {
                        self.position += 3;
                    }
                }
                [0, 6] => {
                    let first = Parameter::from_instruction(digits[2], self.get(self.position + 1))
                        .value(self);
                    let target = Parameter::from_instruction(digits[1], self.get(self.position + 2))
                        .value(self) as usize;
                    if first == 0 {
                        self.position = target;
                    } else {
                        self.position += 3;
                    }
                }
                [0, 7] => {
                    let first = Parameter::from_instruction(digits[2], self.get(self.position + 1))
                        .value(self);
                    let second =
                        Parameter::from_instruction(digits[1], self.get(self.position + 2))
                            .value(self);
                    let target = self.get(self.position + 3) as usize;
                    if first < second {
                        self.set(target, 1);
                    } else {
                        self.set(target, 0);
                    }
                    self.position += 4;
                }
                [0, 8] => {
                    let first = Parameter::from_instruction(digits[2], self.get(self.position + 1))
                        .value(self);
                    let second =
                        Parameter::from_instruction(digits[1], self.get(self.position + 2))
                            .value(self);
                    let target = self.get(self.position + 3) as usize;
                    if first == second {
                        self.set(target, 1);
                    } else {
                        self.set(target, 0);
                    }
                    self.position += 4;
                }
                [0, 9] => {
                    let first = Parameter::from_instruction(digits[2], self.get(self.position + 1))
                        .value(self);
                    self.relative_base += first;
                    self.position += 2;
                }
                _ => {
                    println!(
                        "Error: unknown instruction '{}' at position '{}'.",
                        self.get(self.position),
                        self.position
                    );
                }
            }
        }
    }

    pub fn add_input(&mut self, input: i64) {
        self.input.push_back(input);
    }

    pub fn next_output(&mut self) -> Option<i64> {
        self.output.pop_front()
    }

    pub fn last_output(&mut self) -> Option<i64> {
        self.output.pop_back()
    }

    pub fn set(&mut self, address: usize, value: i64) {
        if address < self.program.len() {
            self.program[address] = value;
        } else {
            self.memory.insert(address, value);
        }
    }

    pub fn get(&self, address: usize) -> i64 {
        if address < self.program.len() {
            self.program[address]
        } else {
            *self.memory.get(&address).unwrap_or(&0)
        }
    }

    fn next_input(&mut self) -> i64 {
        if let Some(value) = self.input.pop_front() {
            value
        } else {
            let mut input = String::new();
            print!("> ");
            std::io::stdout().flush().unwrap();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Could not read stdin");
            input.trim().parse::<i64>().expect("Couldn't parse input")
        }
    }

    fn output(&mut self, value: i64) {
        self.output.push_back(value);
    }
}
*/