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

    pub fn rows(&self) -> usize {
        self.map.len()
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
    (0..map.rows())
        .filter(|i| map.at(*i * 3, *i) == Position::Tree)
        .count()
}

fn second(map: &TobogganMap) -> usize {
    let a = (0..map.rows())
        .filter(|i| map.at(*i, *i) == Position::Tree)
        .count();

    let b = first(map);

    let c = (0..map.rows())
        .filter(|i| map.at(*i * 5, *i) == Position::Tree)
        .count();

    let d = (0..map.rows())
        .filter(|i| map.at(*i * 7, *i) == Position::Tree)
        .count();

    let e = (0..map.rows())
        .filter(|i| map.at(*i, *i * 2) == Position::Tree)
        .count();

    a * b * c * d * e
}

fn main() {
    let input = read_input();
    let map = TobogganMap::new(input);

    dbg!(first(&map));
    dbg!(second(&map));
}
