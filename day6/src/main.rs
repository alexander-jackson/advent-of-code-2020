use std::collections::HashMap;

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
            let mut map = HashMap::new();

            for c in x.chars() {
                let entry = map.entry(c).or_insert(0);
                *entry += 1;
            }

            let people = map.get(&'\n').map(|x| *x).unwrap_or_default() + 1;

            map.iter().filter(|(_, v)| **v == people).count()
        })
        .sum();

    dbg!(&input);
    dbg!(&count);
}
