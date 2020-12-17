use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::str::Lines;

mod point3d;
mod point4d;

use point3d::Point3D;
use point4d::Point4D;

trait Surround {
    fn get_surroundings(&self) -> Vec<Self>
    where
        Self: Sized;
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
struct Space<P> {
    data: HashMap<P, State>,
    on: usize,
}

impl<P: Eq + Hash + Copy + Surround> Space<P> {
    /// Counts the number of active cubes around a given point.
    fn active_surroundings(&self, point: P) -> usize {
        point
            .get_surroundings()
            .iter()
            .filter(|p| *self.data.get(p).unwrap_or_default() == State::On)
            .count()
    }

    /// Returns an iterator over the active points in the current state.
    fn iter_active_points(&self) -> impl Iterator<Item = P> + '_ {
        self.data
            .iter()
            .filter_map(|(point, state)| match *state {
                State::On => Some(point),
                State::Off => None,
            })
            .copied()
    }

    /// Checks all active cubes and returns the ones that should be turned off.
    fn check_active_cubes(&self) -> HashSet<P> {
        self.iter_active_points()
            .filter(|point| !(2..=3).contains(&self.active_surroundings(*point)))
            .collect()
    }

    /// Checks the surroundings of active cubes and determines the ones to turn on.
    fn check_inactive_cubes(&self) -> HashSet<P> {
        let mut cubes = HashSet::new();

        // For each point that is active
        for p in self.iter_active_points() {
            // Get the surrounding points in this coordinate space
            let surroundings = p.get_surroundings();

            for s in surroundings {
                // Check we haven't already added it
                if cubes.contains(&s) {
                    continue;
                }

                // Check their active surroundings
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

impl<P: Eq + Hash + Copy + Surround> Iterator for Space<P> {
    type Item = usize;

    /// Returns the number of cubes that are on after each iteration.
    fn next(&mut self) -> Option<Self::Item> {
        let turn_off = self.check_active_cubes();
        let turn_on = self.check_inactive_cubes();

        for p in &turn_off {
            self.data.insert(*p, State::Off);
        }

        for p in &turn_on {
            self.data.insert(*p, State::On);
        }

        // Update the internal state
        self.on += turn_on.len();
        self.on -= turn_off.len();

        Some(self.on)
    }
}

impl<P: Eq + Hash + From<(usize, usize)>> From<Lines<'_>> for Space<P> {
    /// Creates a [`Space`] from [`Lines`], usually from a file.
    fn from(lines: Lines) -> Self {
        let mut data = HashMap::new();

        for (i, l) in lines.enumerate() {
            for (j, c) in l.chars().enumerate() {
                let point = P::from((i, j));
                let state = State::from(c);
                data.insert(point, state);
            }
        }

        let on = data.values().filter(|v| **v == State::On).count();

        Space { data, on }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let space: Space<Point3D> = Space::from(input.lines());

    let solution = space.skip(5).next().unwrap();
    println!("Part 1 Solution: {}", solution);

    let input = std::fs::read_to_string("input.txt").unwrap();
    let space: Space<Point4D> = Space::from(input.lines());

    let solution = space.skip(5).next().unwrap();
    println!("Part 2 Solution: {}", solution);
}
