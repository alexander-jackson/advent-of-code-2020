use std::str::FromStr;

const INPUT_FILE: &str = "input.txt";

fn read_input(path: &str) -> Vec<u64> {
    std::fs::read_to_string(path)
        .unwrap()
        .trim()
        .split('\n')
        .map(|x| u64::from_str(x).unwrap())
        .collect()
}

pub fn first() -> u64 {
    let input = read_input(INPUT_FILE);

    for x in &input {
        for y in &input {
            if x + y == 2020 {
                return x * y;
            }
        }
    }

    unreachable!()
}

pub fn second() -> u64 {
    let input = read_input(INPUT_FILE);

    for x in &input {
        for y in &input {
            for z in &input {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }

    unreachable!()
}
