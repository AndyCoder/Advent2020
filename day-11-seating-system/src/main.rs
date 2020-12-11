use std::fs;

fn main() {
    let adapters: Vec<String> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split("\n")
        .map(|s| String::from(s))
        .collect();
    let width: usize = adapters[0].chars().count();
    let height: usize = adapters.iter().count();
    let spaces: Vec<Space> = adapters.iter().flat_map(|s| to_space_vec(s)).collect();
    let mut ferry = Ferry::new(spaces, width, height);
    // let result = part_1(&mut ferry);
    let result = part_2(&mut ferry);
    println!("{}", result);
}

fn part_1(ferry: &mut Ferry) -> usize {
    loop {
        let prev = ferry.spaces.clone();
        ferry.tick();
        if cmp(&ferry.spaces, &prev) {
            return ferry
                .spaces
                .iter()
                .filter(|s| s.to_owned().to_owned() == Space::Occupied)
                .count();
        };
    }
}

fn part_2(ferry: &mut Ferry) -> usize {
    loop {
        let prev = ferry.spaces.clone();
        ferry.tick_2();
        if cmp(&ferry.spaces, &prev) {
            return ferry
                .spaces
                .iter()
                .filter(|s| s.to_owned().to_owned() == Space::Occupied)
                .count();
        };
    }
}

fn cmp(left: &[Space], right: &[Space]) -> bool {
    for (i, x) in left.iter().enumerate() {
        if x.to_owned() != right[i] {
            return false;
        }
    }
    true
}

fn to_space_vec(line: &str) -> Vec<Space> {
    line.chars()
        .map(|c| match c {
            '#' => Space::Occupied,
            'L' => Space::Empty,
            _ => Space::Floor,
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Space {
    Occupied,
    Empty,
    Floor,
}

struct Ferry {
    width: usize,
    height: usize,
    spaces: Vec<Space>,
}

impl Ferry {
    fn new(spaces: Vec<Space>, width: usize, height: usize) -> Ferry {
        Ferry {
            width,
            height,
            spaces,
        }
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    fn occupied_neighbor_count_2(&self, row: usize, column: usize) -> usize {
        let mut count = 0;
        if row < self.height {
            count += self.check_direction(row as isize + 1, column as isize, 1, 0);
        }
        if row > 0 {
            count += self.check_direction(row as isize - 1, column as isize, -1, 0);
        }
        if column < self.width {
            count += self.check_direction(row as isize, column as isize + 1, 0, 1);
        }
        if column > 0 {
            count += self.check_direction(row as isize, column as isize - 1, 0, -1);
        }
        if row < self.height && column < self.width {
            count += self.check_direction(row as isize + 1, column as isize + 1, 1, 1);
        }
        if row > 0 && column > 0 {
            count += self.check_direction(row as isize - 1, column as isize - 1, -1, -1);
        }
        if row > 0 && column < self.width {
            count += self.check_direction(row as isize - 1, column as isize + 1, -1, 1);
        }
        if row < self.height && column > 0 {
            count += self.check_direction(row as isize + 1, column as isize - 1, 1, -1);
        }
        count
    }

    fn check_direction(&self, row: isize, column: isize, rise: isize, run: isize) -> usize {
        if row < 0 || column < 0 || row >= self.height as isize || column >= self.width as isize {
            0
        } else if self.spaces[self.get_index(row as usize, column as usize)] == Space::Empty {
            0
        } else if self.spaces[self.get_index(row as usize, column as usize)] == Space::Occupied {
            1
        } else {
            self.check_direction(row + rise, column + run, rise, run)
        }
    }

    fn tick_2(&mut self) {
        let mut next = self.spaces.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.spaces[idx];
                let live_neighbors = self.occupied_neighbor_count_2(row, col);
                let next_cell = match (cell, live_neighbors) {
                    (Space::Empty, 0) => Space::Occupied,
                    (Space::Occupied, x) if x > 4 => Space::Empty,
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.spaces = next;
    }

    fn occupied_neighbor_count(&self, row: usize, column: usize) -> usize {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                if row == 0 && neighbor_row == self.height - 1 {
                    continue;
                }
                if column == 0 && neighbor_col == self.width - 1 {
                    continue;
                }
                if neighbor_row == 0 && row == self.height - 1 {
                    continue;
                }
                if neighbor_col == 0 && column == self.width - 1 {
                    continue;
                }
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += match self.spaces[idx] {
                    Space::Occupied => 1,
                    _ => 0,
                };
            }
        }
        count
    }

    fn tick(&mut self) {
        let mut next = self.spaces.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.spaces[idx];
                let live_neighbors = self.occupied_neighbor_count(row, col);
                let next_cell = match (cell, live_neighbors) {
                    (Space::Empty, 0) => Space::Occupied,
                    (Space::Occupied, x) if x > 3 => Space::Empty,
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.spaces = next;
    }
}
