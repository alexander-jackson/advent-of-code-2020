use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::str::FromStr;

use regex::Regex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RULE: Regex = Regex::new(r"^([a-z ]*): (\d*)-(\d*) or (\d*)-(\d*)$").unwrap();
}

#[derive(Debug)]
struct Rules<'a> {
    rules: HashMap<&'a str, (RangeInclusive<usize>, RangeInclusive<usize>)>,
}

impl Rules<'_> {
    pub fn check(&self, ticket: &Ticket) -> Vec<usize> {
        let mut invalid = Vec::new();

        for value in &ticket.values {
            if !self
                .rules
                .values()
                .any(|x| x.0.contains(&value) || x.1.contains(&value))
            {
                invalid.push(*value);
            }
        }

        invalid
    }
}

impl<'a> From<&'a str> for Rules<'a> {
    fn from(input: &'a str) -> Self {
        let mut rules = HashMap::new();

        for line in input.lines() {
            let captures = RULE.captures(line).unwrap();
            let key = captures.get(1).unwrap().as_str();

            let left_lower = usize::from_str(captures.get(2).unwrap().as_str()).unwrap();
            let left_upper = usize::from_str(captures.get(3).unwrap().as_str()).unwrap();

            let right_lower = usize::from_str(captures.get(4).unwrap().as_str()).unwrap();
            let right_upper = usize::from_str(captures.get(5).unwrap().as_str()).unwrap();

            let left = RangeInclusive::new(left_lower, left_upper);
            let right = RangeInclusive::new(right_lower, right_upper);

            rules.insert(key, (left, right));
        }

        Rules { rules }
    }
}

#[derive(Debug)]
struct Ticket {
    values: Vec<usize>,
}

impl From<&str> for Ticket {
    fn from(input: &str) -> Self {
        Self {
            values: input
                .split(',')
                .map(|x| usize::from_str(x).unwrap())
                .collect(),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut blocks = input.trim().split("\n\n");

    let rules = Rules::from(blocks.next().unwrap());

    let ours = Ticket::from(blocks.next().unwrap().split('\n').skip(1).next().unwrap());

    let others = blocks
        .next()
        .unwrap()
        .split('\n')
        .skip(1)
        .map(|x| Ticket::from(x));

    let solution: usize = others.map(|t| rules.check(&t).iter().sum::<usize>()).sum();

    println!("Part 1 Solution: {}", solution);
}
