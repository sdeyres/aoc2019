use std::{
    collections::{HashMap, VecDeque},
    io::Write,
};

#[derive(Clone, Debug)]
pub struct Intcode {
    program: Vec<i64>,
    position: usize,
    relative_base: i64,
    running: bool,
    memory: HashMap<usize, i64>,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
}

impl From<&str> for Intcode {
    fn from(value: &str) -> Self {
        let program = value
            .split(",")
            .map(|i| {
                i.parse::<i64>()
                    .expect(&format!("Could not parse intcode instruction: {}", i))
            })
            .collect();

        Self {
            program,
            position: 0,
            relative_base: 0,
            running: false,
            memory: HashMap::new(),
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }
}

impl Intcode {
    pub fn add_input(&mut self, input: i64) {
        self.input.push_back(input);
    }

    pub fn next_output(&mut self) -> Option<i64> {
        self.output.pop_front()
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

    pub fn execute(&mut self) {
        self.running = true;

        while self.position < self.program.len() {
            let start = self.position;
            let end = usize::min(self.position + 5, self.program.len());
            let instruction = Instruction::from(&self.program[start..end]);
            println!("{:#?}", instruction);
            instruction.execute(self);
            if matches!(instruction.opcode, Opcode::Out | Opcode::Hcf) {
                break;
            }
        }
    }

    pub fn halt_and_catch_fire(&mut self) {
        self.running = false;
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
                .expect("Could not read from stdin");
            input.trim().parse().expect("Input is not valid")
        }
    }

    fn output(&mut self, output: i64) {
        self.output.push_back(output);
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    parameters: Vec<Parameter>,
}

impl From<&[i64]> for Instruction {
    fn from(value: &[i64]) -> Self {
        let digits: Vec<u32> = format!("{instruction:0>5}", instruction = value[0])
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect();

        let opcode = Opcode::from(&digits[3..5]);

        
        let parameters: Vec<Parameter> = if value.len() > 1 {
            (0..value.len() - 2)
            .into_iter()
            .rev()
            .map(|i| Parameter::from((digits[i], value[value.len() - 2 - i])))
            .collect()
        } else {
            vec![]
        };

        Self { opcode, parameters }
    }
}

impl Instruction {
    fn execute(&self, intcode: &mut Intcode) {
        self.opcode.execute(&self.parameters, intcode);
    }
}

#[derive(Debug)]
enum Opcode {
    Add, // 01
    Mul, // 02
    In,  // 03
    Out, // 04
    Jnz, // 05
    Jz,  // 06
    Lt,  // 07
    Eq,  // 08
    Rel, // 09
    Hcf, // *
}

impl From<&[u32]> for Opcode {
    fn from(value: &[u32]) -> Self {
        match value {
            [0, 1] => Self::Add,
            [0, 2] => Self::Mul,
            [0, 3] => Self::In,
            [0, 4] => Self::Out,
            [0, 5] => Self::Jnz,
            [0, 6] => Self::Jz,
            [0, 7] => Self::Lt,
            [0, 8] => Self::Eq,
            [0, 9] => Self::Rel,
            _ => Self::Hcf,
        }
    }
}

impl Opcode {
    fn execute(&self, parameters: &Vec<Parameter>, intcode: &mut Intcode) {
        match self {
            Self::Add => {
                let first = parameters[0].value(intcode);
                let second = parameters[1].value(intcode);
                let target = parameters[2].value(intcode) as usize;
                intcode.set(target, first + second);
                intcode.position += 4;
            }
            Self::Mul => {
                let first = parameters[0].value(intcode);
                let second = parameters[1].value(intcode);
                let target = parameters[2].value(intcode) as usize;
                intcode.set(target, first * second);
                intcode.position += 4;
            }
            Self::In => {
                let first = intcode.next_input();
                let target = parameters[0].value(intcode) as usize;
                intcode.set(target, first);
                intcode.position += 2;
            }
            Self::Out => {
                let first = parameters[0].value(intcode);
                intcode.output(first);
                intcode.position += 2;
                intcode.halt_and_catch_fire();
            }
            Self::Jnz => {
                let first = parameters[0].value(intcode);
                let target = parameters[1].value(intcode) as usize;
                if first != 0 {
                    intcode.position = target;
                } else {
                    intcode.position += 3;
                }
            }
            Self::Jz => {
                let first = parameters[0].value(intcode);
                let target = parameters[1].value(intcode) as usize;
                if first == 0 {
                    intcode.position = target;
                } else {
                    intcode.position += 3;
                }
            }
            Self::Lt => {
                let first = parameters[0].value(intcode);
                let second = parameters[1].value(intcode);
                let target = parameters[2].value(intcode) as usize;
                intcode.set(target, if first < second { 1 } else { 0 });
                intcode.position += 4;
            }
            Self::Eq => {
                let first = parameters[0].value(intcode);
                let second = parameters[1].value(intcode);
                let target = parameters[2].value(intcode) as usize;
                intcode.set(target, if first == second { 1 } else { 0 });
                intcode.position += 4;
            }
            Self::Rel => {
                let first = parameters[0].value(intcode);
                intcode.relative_base += first;
                intcode.position += 2;
            }
            Self::Hcf => {
                intcode.halt_and_catch_fire();
            }
        }
    }
}

#[derive(Debug)]
enum Parameter {
    Position(usize),
    Immediate(i64),
    Relative(i64),
}

impl From<(u32, i64)> for Parameter {
    fn from(value: (u32, i64)) -> Self {
        match value.0 {
            1 => Self::Immediate(value.1),
            2 => Self::Relative(value.1),
            _ => Self::Position(value.1 as usize),
        }
    }
}

impl Parameter {
    fn value(&self, intcode: &Intcode) -> i64 {
        match self {
            Self::Position(address) => intcode.get(*address),
            Self::Immediate(value) => *value,
            Self::Relative(offset) => intcode.get((intcode.relative_base + offset) as usize),
        }
    }
}
