use std::str::FromStr;

fn find_sum_pair(subset: &[usize], value: usize) -> Option<(usize, usize)> {
    for i in 0..subset.len() {
        for j in 0..subset.len() {
            if i == j {
                continue;
            }

            let (x, y) = (subset[i], subset[j]);

            if x + y == value {
                return Some((x, y));
            }
        }
    }

    None
}

fn find_first_violation(values: &[usize], window_size: usize) -> usize {
    for i in window_size..values.len() {
        if find_sum_pair(&values[i - window_size..i], values[i]).is_none() {
            return values[i];
        }
    }

    0
}

fn find_contiguous_set(values: &[usize], to_find: usize) -> &[usize] {
    // Try each index
    for i in 0..values.len() {
        let mut sum = values[i];

        for j in i + 1..values.len() {
            sum += values[j];

            if sum == to_find {
                return &values[i..=j];
            } else if sum > to_find {
                break;
            }
        }
    }

    values
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let values: Vec<_> = input
        .trim()
        .lines()
        .map(|x| usize::from_str(x).unwrap())
        .collect();

    dbg!(&values);

    let first_violation = find_first_violation(&values, 25);
    dbg!(&first_violation);

    let contiguous_set = find_contiguous_set(&values, first_violation);
    dbg!(&contiguous_set);

    let min = contiguous_set.iter().min().unwrap();
    let max = contiguous_set.iter().max().unwrap();

    println!("Result: {}", min + max);
}
