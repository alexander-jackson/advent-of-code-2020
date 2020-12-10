use std::collections::HashSet;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum Instruction {
    NoOperation(i32),
    Accumulate(i32),
    Jump(i32),
}

impl Instruction {
    pub fn flip(self) -> Self {
        use crate::Instruction::*;

        match self {
            NoOperation(v) => Jump(v),
            Accumulate(v) => Accumulate(v),
            Jump(v) => NoOperation(v),
        }
    }
}

impl From<&str> for Instruction {
    fn from(v: &str) -> Self {
        let instruction = &v[..3];
        let operand = &v[4..];
        let value = i32::from_str(operand).unwrap();

        match instruction {
            "nop" => Instruction::NoOperation(value),
            "acc" => Instruction::Accumulate(value),
            "jmp" => Instruction::Jump(value),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Machine {
    instructions: Vec<Instruction>,
    accumulator: i32,
    program_counter: usize,
    visited: HashSet<usize>,
}

impl Machine {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            ..Default::default()
        }
    }

    pub fn simulate_until_loop(&mut self) -> i32 {
        loop {
            if !self.visited.insert(self.program_counter) {
                break self.accumulator;
            }

            let current = self
                .instructions
                .get(self.program_counter)
                .ok_or(self.accumulator)
                .unwrap();

            // Perform instruction operation
            match *current {
                Instruction::Accumulate(v) => self.accumulator += v as i32,
                _ => (),
            }

            // Perform program counter manipulation
            match *current {
                Instruction::Jump(v) => {
                    if v < 0 {
                        self.program_counter -= (-v) as usize;
                    } else {
                        self.program_counter += v as usize;
                    }
                }
                _ => self.program_counter += 1,
            }
        }
    }

    pub fn reset(&mut self) {
        self.program_counter = Default::default();
        self.accumulator = Default::default();
        self.visited = Default::default();
    }

    pub fn flip(&mut self, index: usize) {
        // If this isn't the first run, restore the previous index
        if index != 0 {
            self.instructions[index - 1] = self.instructions[index - 1].flip();
        }

        self.instructions[index] = self.instructions[index].flip();
    }
}

fn parse_input(lines: &[&str]) -> Vec<Instruction> {
    lines.iter().map(|x| Instruction::from(*x)).collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<_> = input.trim().lines().collect();

    let instructions = parse_input(&lines);
    let mut machine = Machine::new(instructions);

    let value = machine.simulate_until_loop();
    println!("Part 1 Solution: {}", value);

    for i in 0..lines.len() {
        machine.reset();
        machine.flip(i);
        let value = machine.simulate_until_loop();
    }
}
