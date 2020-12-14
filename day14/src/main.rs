use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    static ref MASK: Regex = Regex::new(r"^mask = ([01X]{36})$").unwrap();
    static ref ASSIGN: Regex = Regex::new(r"^mem\[(\d*)\] = (\d*)$").unwrap();
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Version {
    One,
    Two,
}

#[derive(Debug, Default)]
struct Initialiser {
    mask: Vec<(usize, char)>,
    memory: HashMap<u64, u64>,
}

impl Initialiser {
    pub fn handle(&mut self, instruction: Instruction, version: Version) {
        match instruction {
            Instruction::Mask(mask) => self.update_mask(mask),
            Instruction::Assignment { address, value } => {
                self.assign_value(address, value, version)
            }
        }
    }

    pub fn update_mask(&mut self, mask: Vec<(usize, char)>) {
        self.mask = mask;
    }

    pub fn assign_value(&mut self, address: u64, value: u64, version: Version) {
        if version == Version::One {
            self.memory.insert(address, apply_mask(&self.mask, value));
        } else {
            let locations = apply_v2_mask(&self.mask, address);

            for addr in locations {
                self.memory.insert(addr, value);
            }
        }
    }

    pub fn reset(&mut self) {
        self.mask = Default::default();
        self.memory = Default::default();
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Mask(Vec<(usize, char)>),
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
                .map(|(i, x)| (35 - i, x))
                .collect();

            return Instruction::Mask(bitmask);
        }

        let caps = ASSIGN.captures(line).unwrap();

        let address = u64::from_str(caps.get(1).unwrap().as_str()).unwrap();
        let value = u64::from_str(caps.get(2).unwrap().as_str()).unwrap();

        Instruction::Assignment { address, value }
    }
}

fn apply_mask(mask: &[(usize, char)], value: u64) -> u64 {
    let mut result = value;

    for (addr, binary) in mask {
        match binary {
            '0' => result &= !(1 << addr),
            '1' => result |= 1 << addr,
            _ => (),
        }
    }

    result
}

fn apply_v2_mask(mask: &[(usize, char)], value: u64) -> HashSet<u64> {
    let mut results = vec![value];

    for (addr, binary) in mask {
        match binary {
            '1' => {
                results = results.iter().map(|x| x | 1 << addr).collect();
            }
            'X' => {
                let with_zero: Vec<_> = results.iter().map(|x| x & !(1 << addr)).collect();
                let with_one: Vec<_> = results.iter().map(|x| x | 1 << addr).collect();
                results.extend(with_zero);
                results.extend(with_one);
            }
            _ => (),
        }
    }

    results.into_iter().collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<_> = input.trim().lines().collect();

    let instructions: Vec<_> = lines.iter().map(|x| Instruction::from(*x)).collect();

    let mut initialiser = Initialiser::default();

    for instruction in instructions.clone() {
        initialiser.handle(instruction, Version::One);
    }

    let solution: u64 = initialiser.memory.values().sum();
    println!("Part 1 Solution: {}", solution);

    initialiser.reset();

    for instruction in instructions.clone() {
        initialiser.handle(instruction, Version::Two);
    }

    let solution: u64 = initialiser.memory.values().sum();
    println!("Part 2 Solution: {}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masks_are_applied_correctly() {
        let mask = vec![(0, '0'), (1, '1')];
        let value = 0b101;

        let expected = 0b110;

        assert_eq!(apply_mask(&mask, value), expected);
    }

    #[test]
    fn v2_masks_are_applied_correctly() {
        let mask = vec![(0, '0'), (1, '1'), (2, 'X')];
        let value = 0b001;

        let mut expected = HashSet::new();
        expected.insert(0b011);
        expected.insert(0b111);

        assert_eq!(apply_v2_mask(&mask, value), expected);
    }

    #[test]
    fn v2_masks_are_applied_correctly_for_longer_inputs() {
        let mask = vec![(0, '0'), (1, '1'), (2, 'X'), (3, '1')];
        let value = 0b001;

        let mut expected = HashSet::new();
        expected.insert(0b1011);
        expected.insert(0b1111);

        assert_eq!(apply_v2_mask(&mask, value), expected);
    }

    #[test]
    fn v2_masks_are_applied_correctly_for_even_longer_inputs() {
        let mask = vec![(0, '0'), (1, '1'), (2, 'X'), (3, '1'), (4, 'X')];
        let value = 0b001;

        let mut expected = HashSet::new();
        expected.insert(0b1011);
        expected.insert(0b1111);
        expected.insert(0b11011);
        expected.insert(0b11111);

        assert_eq!(apply_v2_mask(&mask, value), expected);
    }
}
