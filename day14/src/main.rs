use std::collections::HashMap;
use std::str::FromStr;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    static ref MASK: Regex = Regex::new(r"^mask = ([01X]{36})$").unwrap();
    static ref ASSIGN: Regex = Regex::new(r"^mem\[(\d*)\] = (\d*)$").unwrap();
}

#[derive(Debug, Default)]
struct Initialiser {
    mask: Vec<(usize, u32)>,
    memory: HashMap<u64, u64>,
}

impl Initialiser {
    pub fn handle(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Mask(mask) => self.update_mask(mask),
            Instruction::Assignment { address, value } => self.assign_value(address, value),
        }
    }

    pub fn update_mask(&mut self, mask: Vec<(usize, u32)>) {
        self.mask = mask;
    }

    pub fn assign_value(&mut self, address: u64, value: u64) {
        self.memory.insert(address, apply_mask(&self.mask, value));
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Mask(Vec<(usize, u32)>),
    Assignment { address: u64, value: u64 },
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        if let Some(caps) = MASK.captures(line) {
            let bitmask: Vec<_> = caps
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .enumerate()
                .filter(|x| x.1 != 'X')
                .map(|(i, x)| (35 - i, x.to_digit(10).unwrap()))
                .collect();

            return Instruction::Mask(bitmask);
        }

        let caps = ASSIGN.captures(line).unwrap();

        let address = u64::from_str(caps.get(1).unwrap().as_str()).unwrap();
        let value = u64::from_str(caps.get(2).unwrap().as_str()).unwrap();

        Instruction::Assignment { address, value }
    }
}

fn apply_mask(mask: &[(usize, u32)], value: u64) -> u64 {
    let mut result = value;

    for (addr, binary) in mask {
        match binary {
            0 => result &= !(1 << addr),
            1 => result |= 1 << addr,
            _ => unreachable!(),
        }
    }

    result
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<_> = input.trim().lines().collect();

    let instructions: Vec<_> = lines.iter().map(|x| Instruction::from(*x)).collect();

    let mut initialiser = Initialiser::default();

    for instruction in instructions {
        initialiser.handle(instruction);
    }

    let solution: u64 = initialiser.memory.values().sum();
    println!("Part 1 Solution: {}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masks_are_applied_correctly() {
        let mask = vec![(0, 0), (1, 1)];
        let value = 0b101;

        let expected = 0b110;

        assert_eq!(apply_mask(&mask, value), expected);
    }
}
