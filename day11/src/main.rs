#[derive(Debug)]
struct Automata {
    state: Vec<Vec<char>>,
}

impl Automata {
    pub fn new(initial: Vec<Vec<char>>) -> Self {
        Self { state: initial }
    }

    pub fn simulate(&mut self) -> u32 {
        loop {
            // Copy the state
            let mut next = self.state.clone();

            // Apply the rules to each space
            for i in 0..self.state.len() {
                for j in 0..self.state[i].len() {
                    next[i][j] = self.calculate_next(i, j);
                }
            }

            if self.state == next {
                break self.count_all_occupied();
            }

            self.state = next;
        }
    }

    fn calculate_next(&self, x: usize, y: usize) -> char {
        let current = self.state[x][y];
        let occupied = self.count_occupied(x, y);

        if current == 'L' && occupied == 0 {
            '#'
        } else if current == '#' && 4 <= occupied {
            'L'
        } else {
            current
        }
    }

    fn count_occupied(&self, x: usize, y: usize) -> usize {
        let mut total = 0;

        let lx = if x >= 1 { x - 1 } else { 0 };
        let ly = if y >= 1 { y - 1 } else { 0 };

        for i in lx..=x + 1 {
            for j in ly..=y + 1 {
                if i == x && j == y {
                    continue;
                }

                // Get the value at this square
                let occupied = self
                    .state
                    .get(i)
                    .map(|row| row.get(j))
                    .flatten()
                    .map(|c| *c == '#')
                    .unwrap_or_default();

                if occupied {
                    total += 1;
                }
            }
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

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let formatted: Vec<Vec<_>> = input.trim().lines().map(|l| l.chars().collect()).collect();

    let mut automata = Automata::new(formatted);
    let occupied = automata.simulate();

    println!("Part 1 Solution: {}", occupied);
}
