use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<_> = input.trim().lines().collect();

    let timestamp = u32::from_str(lines[0]).unwrap();
    let bus_identifiers: Vec<_> = lines[1]
        .split(',')
        .filter_map(|t| u32::from_str(t).ok())
        .collect();

    let (bus_id, wait_time) = bus_identifiers
        .iter()
        .map(|id| (id, id - (timestamp % id)))
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();

    println!("Part 1 Solution: {}", bus_id * wait_time);
}
