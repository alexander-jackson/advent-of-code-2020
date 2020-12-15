use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut numbers: Vec<_> = input
        .trim()
        .split(',')
        .map(|x| usize::from_str(x).unwrap())
        .collect();

    for i in numbers.len()..2020 {
        let last_spoken = numbers.last().unwrap();

        if let Some(j) = numbers.iter().rev().skip(1).position(|i| i == last_spoken) {
            numbers.push(i - (numbers.len() - 1 - j));
        } else {
            numbers.push(0);
        }
    }

    let solution = numbers.last().unwrap();
    println!("Part 1 Solution: {}", solution);
}
