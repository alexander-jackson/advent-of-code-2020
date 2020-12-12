use std::ops::{AddAssign, Mul};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Mode {
    Normal,
    Waypoint,
}

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

impl Coordinate {
    pub fn update(&mut self, modifier: char, value: i32) {
        match modifier {
            'N' => self.y += value,
            'E' => self.x += value,
            'S' => self.y -= value,
            'W' => self.x -= value,
            _ => unreachable!(),
        }
    }

    pub fn rotate(&mut self, degrees: i32) {
        let s = (-degrees as f32).to_radians().sin() as i32;
        let c = (-degrees as f32).to_radians().cos() as i32;

        *self = Self {
            x: self.x * c - self.y * s,
            y: self.x * s + self.y * c,
        }
    }
}

impl Mul<i32> for Coordinate {
    type Output = Self;

    fn mul(self, value: i32) -> Self {
        Self {
            x: self.x * value,
            y: self.y * value,
        }
    }
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
    waypoint: Coordinate,
}

impl Orienteer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn simulate(&mut self, moves: &[(char, i32)], mode: Mode) -> Coordinate {
        match mode {
            Mode::Normal => self.simulate_normal(moves),
            Mode::Waypoint => self.simulate_waypoint(moves),
        }
    }

    fn simulate_normal(&mut self, moves: &[(char, i32)]) -> Coordinate {
        for instruction in moves {
            let (modifier, value) = *instruction;

            match modifier {
                'N' | 'E' | 'S' | 'W' => self.position.update(modifier, value),
                'L' => self.heading.rotate(360 - value),
                'R' => self.heading.rotate(value),
                'F' => self.position += self.heading.modifier(value),
                _ => unreachable!(),
            }
        }

        self.position
    }

    fn simulate_waypoint(&mut self, moves: &[(char, i32)]) -> Coordinate {
        for instruction in moves {
            let (modifier, value) = *instruction;

            match modifier {
                'N' | 'E' | 'S' | 'W' => self.waypoint.update(modifier, value),
                'L' => self.waypoint.rotate(360 - value),
                'R' => self.waypoint.rotate(value),
                'F' => self.position += self.waypoint * value,
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
            waypoint: Coordinate { x: 10, y: 1 },
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
    let position = orienteer.simulate(&moves, Mode::Normal);

    println!("Part 1 Solution: {}", position.x.abs() + position.y.abs());

    let mut orienteer = Orienteer::new();
    let position = orienteer.simulate(&moves, Mode::Waypoint);

    println!("Part 2 Solution: {}", position.x.abs() + position.y.abs());
}
