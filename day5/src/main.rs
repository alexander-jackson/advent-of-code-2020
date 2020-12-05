fn read_input() -> Vec<String> {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|x| x.to_string())
        .collect()
}

fn get_seat_id(x: &str) -> u32 {
    let binary: String = x
        .chars()
        .map(|c| match c {
            'F' | 'L' => '0',
            'B' | 'R' => '1',
            _ => unreachable!(),
        })
        .collect();

    let row_str = &binary[..7];
    let col_str = &binary[7..];

    let row = u32::from_str_radix(row_str, 2).unwrap();
    let col = u32::from_str_radix(col_str, 2).unwrap();

    row * 8 + col
}

fn main() {
    let input = read_input();

    let highest_seat_id = input.iter().map(|x| get_seat_id(x)).max().unwrap();
    dbg!(&highest_seat_id);
}
