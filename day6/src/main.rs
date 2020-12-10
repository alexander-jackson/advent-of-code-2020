use std::collections::{HashMap, HashSet};

fn read_input() -> Vec<String> {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split("\n\n")
        .map(|x| x.into())
        .collect()
}

fn main() {
    let input = read_input();

    let count: usize = input
        .iter()
        .map(|x| {
            let mut map = HashSet::new();

            for c in x.chars() {
                map.insert(c);
            }

            map.len() - 1
        })
        .sum();

    println!("Part 1 Solution: {}", count + 1);

    let count: usize = input
        .iter()
        .map(|x| {
            let mut map = HashMap::new();

            for c in x.chars() {
                let entry = map.entry(c).or_insert(0);
                *entry += 1;
            }

            let people = map.get(&'\n').map(|x| *x).unwrap_or_default() + 1;

            map.values().filter(|v| **v == people).count()
        })
        .sum();

    println!("Part 2 Solution: {}", count);
}
