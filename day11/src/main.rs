#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Mode {
    Relaxed,
    Strict,
}

#[derive(Debug)]
struct Automata {
    state: Vec<Vec<char>>,
}

impl Automata {
    pub fn new(initial: Vec<Vec<char>>) -> Self {
        Self { state: initial }
    }

    pub fn simulate(&mut self, mode: Mode) -> u32 {
        loop {
            // Copy the state
            let mut next = self.state.clone();

            // Apply the rules to each space
            for i in 0..self.state.len() {
                for j in 0..self.state[i].len() {
                    next[i][j] = self.calculate_next(i, j, mode);
                }
            }

            if self.state == next {
                break self.count_all_occupied();
            }

            self.state = next;
        }
    }

    fn calculate_next(&self, x: usize, y: usize, mode: Mode) -> char {
        let current = self.state[x][y];
        let occupied = self.count_occupied(x, y, mode);

        if current == 'L' && occupied == 0 {
            '#'
        } else if current == '#' && 4 <= occupied && mode == Mode::Relaxed {
            'L'
        } else if current == '#' && 5 <= occupied && mode == Mode::Strict {
            'L'
        } else {
            current
        }
    }

    fn at(&self, x: usize, y: usize) -> Option<char> {
        self.state
            .get(x)
            .map(|row| row.get(y))
            .flatten()
            .map(|c| *c)
    }

    fn is_occupied(&self, x: usize, y: usize) -> bool {
        self.at(x, y) == Some('#')
    }

    pub fn count_occupied(&self, x: usize, y: usize, mode: Mode) -> usize {
        let mut total = 0;

        let lx = if x >= 1 { x - 1 } else { 0 };
        let ly = if y >= 1 { y - 1 } else { 0 };

        if mode == Mode::Relaxed {
            for i in lx..=x + 1 {
                for j in ly..=y + 1 {
                    if i == x && j == y {
                        continue;
                    }

                    if self.is_occupied(i, j) {
                        total += 1;
                    }
                }
            }

            return total;
        }

        let rows = self.state.len();
        let cols = self.state[0].len();

        // Mode is Mode::Strict, begin counting vertically
        if let Some('#') = (0..x)
            .rev()
            .filter_map(|i| self.at(i, y))
            .filter(|c| *c != '.')
            .next()
        {
            total += 1;
        }

        if let Some('#') = (x + 1..rows)
            .filter_map(|i| self.at(i, y))
            .filter(|c| *c != '.')
            .next()
        {
            total += 1;
        }

        // Count horizontally
        if let Some('#') = (0..y)
            .rev()
            .filter_map(|i| self.at(x, i))
            .filter(|c| *c != '.')
            .next()
        {
            total += 1;
        }

        if let Some('#') = (y + 1..cols)
            .filter_map(|i| self.at(x, i))
            .filter(|c| *c != '.')
            .next()
        {
            total += 1;
        }

        // Count the lower left to upper right diagonal
        if let Some('#') = (1..100)
            .filter_map(|i| self.at(x - i, y + i))
            .filter(|c| *c != '.')
            .next()
        {
            total += 1;
        }

        if let Some('#') = (1..100)
            .filter_map(|i| self.at(x + i, y - i))
            .filter(|c| *c != '.')
            .next()
        {
            total += 1;
        }

        // Count the upper left to lower right diagonal
        if let Some('#') = (1..100)
            .filter_map(|i| self.at(x + i, y + i))
            .filter(|c| *c != '.')
            .next()
        {
            total += 1;
        }

        if let Some('#') = (1..100)
            .filter_map(|i| self.at(x - i, y - i))
            .filter(|c| *c != '.')
            .next()
        {
            total += 1;
        }

        total
    }

    fn count_all_occupied(&self) -> u32 {
        self.state
            .iter()
            .map(|row| row.iter().filter(|c| **c == '#').count() as u32)
            .sum()
    }
}

pub fn get_board(filename: &str) -> Vec<Vec<char>> {
    let input = std::fs::read_to_string(filename).unwrap();
    input.trim().lines().map(|l| l.chars().collect()).collect()
}

fn main() {
    let board = get_board("input.txt");

    let mut automata = Automata::new(board.clone());
    let occupied = automata.simulate(Mode::Relaxed);

    println!("Part 1 Solution: {}", occupied);

    let mut automata = Automata::new(board);
    let occupied = automata.simulate(Mode::Strict);

    println!("Part 2 Solution: {}", occupied);
}
