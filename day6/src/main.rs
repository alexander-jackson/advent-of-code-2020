use std::collections::HashSet;

fn read_input() -> Vec<String> {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split("\n\n")
        .map(|x| x.replace("\n", ""))
        .collect()
}

fn main() {
    let input = read_input();

    let count: usize = input
        .iter()
        .map(|x| x.chars().collect::<HashSet<_>>().len())
        .sum();

    dbg!(&input);
    dbg!(&count);
}
