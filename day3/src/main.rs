#[derive(Debug, Eq, PartialEq)]
enum Position {
    Empty,
    Tree,
    Finish,
}

#[derive(Debug)]
struct TobogganMap {
    map: Vec<Vec<char>>,
}

impl TobogganMap {
    pub fn new(map: Vec<Vec<char>>) -> Self {
        Self { map }
    }

    pub fn at(&self, x: usize, y: usize) -> Position {
        // Check if this is off the bottom
        if y >= self.map.len() {
            return Position::Finish;
        }

        let row = &self.map[y];
        let index = x % row.len();

        match row[index] {
            '.' => Position::Empty,
            '#' => Position::Tree,
            _ => unreachable!(),
        }
    }

    pub fn trees_along<F: Fn(usize) -> (usize, usize)>(&self, angle: F) -> usize {
        (0..self.map.len())
            .filter(|i| {
                let (x, y) = angle(*i);
                self.at(x, y) == Position::Tree
            })
            .count()
    }
}

fn read_input() -> Vec<Vec<char>> {
    let contents = std::fs::read_to_string("input.txt").unwrap();

    contents
        .trim()
        .split('\n')
        .map(|line| line.chars().collect())
        .collect()
}

fn first(map: &TobogganMap) -> usize {
    // As we move 3 right, 1 down, we need to make `rows` moves
    map.trees_along(|i| (i * 3, i))
}

fn second(map: &TobogganMap) -> usize {
    let a = map.trees_along(|i| (i, i));
    let b = first(map);
    let c = map.trees_along(|i| (i * 5, i));
    let d = map.trees_along(|i| (i * 7, i));
    let e = map.trees_along(|i| (i, i * 2));

    a * b * c * d * e
}

fn main() {
    let input = read_input();
    let map = TobogganMap::new(input);

    println!("Part 1 Solution: {}", first(&map));
    println!("Part 2 Solution: {}", second(&map));
}
