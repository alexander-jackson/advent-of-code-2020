use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut formatted: Vec<_> = input
        .trim()
        .lines()
        .map(|l| u32::from_str(l).unwrap())
        .collect();

    formatted.sort();

    let adapter = formatted.last().unwrap() + 3;

    let one_gaps = formatted
        .windows(2)
        .filter(|window| match window {
            &[a, b] => b - a == 1,
            _ => unreachable!(),
        })
        .count()
        + 1;

    dbg!(&one_gaps);

    let three_gaps = formatted
        .windows(2)
        .filter(|window| match window {
            &[a, b] => b - a == 3,
            _ => unreachable!(),
        })
        .count()
        + 1;

    dbg!(&three_gaps);

    println!("Solution: {}", one_gaps * three_gaps);
}
