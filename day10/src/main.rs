use std::collections::HashMap;
use std::str::FromStr;

fn count_combinations(adapters: &[i64]) -> i64 {
    let mut counts: HashMap<i64, i64> = HashMap::new();

    // Start with our initial value
    counts.insert(0, 1);

    let end = *adapters.last().unwrap();

    // Iterate dynamically
    for i in 1..=end {
        if !adapters.contains(&i) {
            continue;
        }

        // Find the routes that we could have come from
        let routes: i64 = (1..=3)
            .filter(|j| counts.contains_key(&(i - j)))
            .map(|j| counts[&(i - j)])
            .sum();

        counts.insert(i, routes);
    }

    counts[&end]
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut formatted: Vec<_> = input
        .trim()
        .lines()
        .map(|l| i64::from_str(l).unwrap())
        .collect();

    formatted.sort();

    let one_gaps = formatted
        .windows(2)
        .filter(|window| match window {
            &[a, b] => b - a == 1,
            _ => unreachable!(),
        })
        .count()
        + 1;

    let three_gaps = formatted
        .windows(2)
        .filter(|window| match window {
            &[a, b] => b - a == 3,
            _ => unreachable!(),
        })
        .count()
        + 1;

    println!("Part 1 Solution: {}", one_gaps * three_gaps);

    let combinations = count_combinations(&formatted);

    println!("Part 2 Solution: {}", combinations);
}
