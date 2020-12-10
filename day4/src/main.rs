use std::collections::HashMap;
use std::str::FromStr;

const EXPECTED_FIELDS: [&str; 7] = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];

fn parse_input() -> Vec<String> {
    let contents = std::fs::read_to_string("input.txt").unwrap();

    contents
        .trim()
        .split("\n\n")
        .map(|x| x.replace("\n", " "))
        .collect::<Vec<_>>()
}

fn get_hashmap(passport: &str) -> HashMap<&str, &str> {
    passport
        .split(' ')
        .map(|x| {
            let mut split = x.split(':');
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect()
}

fn is_valid(passport: &str) -> bool {
    // Split at the spaces and then the ':'
    let map = get_hashmap(passport);

    EXPECTED_FIELDS.iter().all(|x| map.contains_key(x))
}

fn validate_number(value: &str, min: usize, max: usize) -> bool {
    // Try and parse it
    let parsed = match usize::from_str(value) {
        Ok(v) => v,
        Err(_) => return false,
    };

    min <= parsed && parsed <= max
}

fn validate_height(value: &str) -> bool {
    // Check it ends in cm or in and validate the number remaining
    if let Some(cm) = value.strip_suffix("cm") {
        return validate_number(cm, 150, 193);
    }

    if let Some(inches) = value.strip_suffix("in") {
        return validate_number(inches, 59, 76);
    }

    false
}

fn validate_hcl(value: &str) -> bool {
    // Ensure the first char is #
    let mut chars = value.chars();

    if !chars.next().map(|x| x == '#').unwrap_or_default() {
        return false;
    }

    let mut lowercase = chars.map(|c| c.to_ascii_lowercase());

    if !lowercase.all(|c| c.is_ascii_hexdigit()) {
        return false;
    }

    value.len() == 7
}

fn validate_ecl(value: &str) -> bool {
    match value {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn validate_pid(value: &str) -> bool {
    value.len() == 9
}

fn validate(key: &str, value: &str) -> bool {
    match key {
        "byr" => validate_number(value, 1920, 2002),
        "iyr" => validate_number(value, 2010, 2020),
        "eyr" => validate_number(value, 2020, 2030),
        "hgt" => validate_height(value),
        "hcl" => validate_hcl(value),
        "ecl" => validate_ecl(value),
        "pid" => validate_pid(value),
        "cid" => return true,
        _ => unreachable!(),
    }
}

fn increased_validation(passport: &str) -> bool {
    let map = get_hashmap(passport);

    if !is_valid(passport) {
        return false;
    }

    map.iter().all(|(k, v)| validate(k, v))
}

fn main() {
    let passports = parse_input();

    let valid = passports.iter().filter(|x| is_valid(x)).count();

    println!("Part 1 Solution: {}", valid);

    let stricter = passports.iter().filter(|x| increased_validation(x)).count();

    println!("Part 2 Solution: {}", stricter);
}
