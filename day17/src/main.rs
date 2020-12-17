use std::collections::{HashMap, HashSet};
use std::str::Lines;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn get_surroundings(&self) -> Vec<Point> {
        let Point { x, y, z } = self;
        let mut points = Vec::new();

        for i in x - 1..=x + 1 {
            for j in y - 1..=y + 1 {
                for k in z - 1..=z + 1 {
                    let point = Point::new(i, j, k);

                    if point == *self {
                        continue;
                    }

                    points.push(point);
                }
            }
        }

        points
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State {
    On,
    Off,
}

impl Default for &State {
    fn default() -> Self {
        &State::Off
    }
}

impl From<char> for State {
    fn from(c: char) -> Self {
        match c {
            '.' => State::Off,
            '#' => State::On,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Space {
    data: HashMap<Point, State>,
}

impl Space {
    fn active_surroundings(&self, point: Point) -> usize {
        point
            .get_surroundings()
            .iter()
            .filter(|p| *self.data.get(p).unwrap_or_default() == State::On)
            .count()
    }

    /// Checks all active cubes and returns the ones that should be turned off.
    fn check_active_cubes(&self) -> HashSet<Point> {
        self.data
            .iter()
            .filter_map(|(k, v)| {
                let active = self.active_surroundings(*k);

                if *v == State::On && (active < 2 || active > 3) {
                    Some(k)
                } else {
                    None
                }
            })
            .copied()
            .collect()
    }

    /// Checks the surroundings of active cubes and determines the ones to turn on.
    fn check_inactive_cubes(&self) -> HashSet<Point> {
        let mut cubes = HashSet::new();

        // For each point that is active
        for (p, _) in self.data.iter().filter(|(_, v)| **v == State::On) {
            let surroundings = p.get_surroundings();

            for s in surroundings {
                if self.active_surroundings(s) == 3
                    && *self.data.get(&s).unwrap_or_default() == State::Off
                {
                    cubes.insert(s);
                }
            }
        }

        cubes
    }
}

impl Iterator for Space {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let turn_off = self.check_active_cubes();
        let turn_on = self.check_inactive_cubes();

        for p in turn_off {
            self.data.insert(p, State::Off);
        }

        for p in turn_on {
            self.data.insert(p, State::On);
        }

        Some(self.data.iter().filter(|(_, v)| **v == State::On).count())
    }
}

impl From<Lines<'_>> for Space {
    fn from(lines: Lines) -> Self {
        let mut data = HashMap::new();

        for (i, l) in lines.enumerate() {
            for (j, c) in l.chars().enumerate() {
                let point = Point::new(i as i32, j as i32, 0);
                let state = State::from(c);
                data.insert(point, state);
            }
        }

        Space { data }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let space = Space::from(input.lines());

    let solution = space.skip(5).next().unwrap();
    println!("Part 1 Solution: {}", solution);
}
