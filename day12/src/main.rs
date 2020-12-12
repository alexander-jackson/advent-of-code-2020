use std::ops::AddAssign;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    pub fn rotate(&mut self, degrees: i32) {
        let current = match *self {
            Heading::North => 0,
            Heading::East => 1,
            Heading::South => 2,
            Heading::West => 3,
        };

        let updated = (current + degrees / 90) % 4;

        *self = match updated {
            0 => Heading::North,
            1 => Heading::East,
            2 => Heading::South,
            3 => Heading::West,
            _ => unreachable!(),
        }
    }

    pub fn modifier(&self, value: i32) -> Coordinate {
        match *self {
            Heading::North => Coordinate { x: 0, y: value },
            Heading::East => Coordinate { x: value, y: 0 },
            Heading::South => Coordinate { x: 0, y: -value },
            Heading::West => Coordinate { x: -value, y: 0 },
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Orienteer {
    heading: Heading,
    position: Coordinate,
}

impl Orienteer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn simulate(&mut self, moves: Vec<(char, i32)>) -> Coordinate {
        for instruction in moves {
            let (modifier, value) = instruction;

            match modifier {
                'N' => self.position.y += value,
                'E' => self.position.x += value,
                'S' => self.position.y -= value,
                'W' => self.position.x -= value,
                'L' => self.heading.rotate(360 - value),
                'R' => self.heading.rotate(value),
                'F' => self.position += self.heading.modifier(value),
                _ => unreachable!(),
            }
        }

        self.position
    }
}

impl Default for Orienteer {
    fn default() -> Self {
        Self {
            heading: Heading::East,
            position: Default::default(),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let moves: Vec<_> = input
        .trim()
        .lines()
        .map(|x| {
            let modifier = x.chars().next().unwrap();
            let value = i32::from_str(&x[1..]).unwrap();

            (modifier, value)
        })
        .collect();

    let mut orienteer = Orienteer::new();
    let position = orienteer.simulate(moves);

    println!("Part 1 Solution: {}", position.x.abs() + position.y.abs());
}
