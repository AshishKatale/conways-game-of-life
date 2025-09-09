use std::{collections::VecDeque, fmt::Display, fs, path::Path};

pub struct GameState {
    title: String,
    enable_grid: bool,
    state: Vec<Vec<bool>>,
    state_history: VecDeque<Vec<Vec<bool>>>,
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut state_str = String::new();

        for row in &self.state {
            for cell in row {
                if *cell {
                    state_str.push_str(" \x1b[1;92m0");
                } else if self.enable_grid {
                    state_str.push_str(" \x1b[0;94m-");
                } else {
                    state_str.push_str("  ");
                }
            }
            state_str.push_str("\r\n");
        }
        write!(f, "{}\r\n \x1b[1;97m{}\x1b[0m\r\n", state_str, self.title,)
    }
}

impl GameState {
    pub fn new(state: Vec<Vec<bool>>, title: &str) -> Self {
        GameState {
            state,
            enable_grid: false,
            title: title.to_owned(),
            state_history: VecDeque::with_capacity(16),
        }
    }

    fn get_neighbours(&self, i: usize, j: usize) -> Vec<bool> {
        #[rustfmt::skip]
        let neighbours_lookup = vec![
            (-1, -1), (-1, 0), (-1, 1),
            ( 0, -1), /*Cell*/ ( 0, 1),
            ( 1, -1), ( 1, 0), ( 1, 1),
        ];

        let validate = |x: i32, y: i32| {
            x >= 0 && x < (self.state.len() as i32) && y >= 0 && y < (self.state[0].len() as i32)
        };

        neighbours_lookup
            .iter()
            .map(|(x, y)| (x + (i as i32), y + (j as i32)))
            .filter(|(x, y)| validate(*x, *y))
            .map(|(x, y)| self.state[x as usize][y as usize])
            .collect::<Vec<_>>()
    }

    fn dead_or_alive(&self, i: usize, j: usize) -> bool {
        let alive_neighbours = self.get_neighbours(i, j).iter().filter(|x| **x).count();
        if self.state[i][j] {
            alive_neighbours == 2 || alive_neighbours == 3
        } else {
            alive_neighbours == 3
        }
    }

    fn update(&mut self) {
        if self.state_history.len() == self.state_history.capacity() {
            self.state_history.pop_front(); // remove oldest
        }
        self.state_history.push_back(self.state.clone()); // push current

        let mut new_state_row1: Vec<bool> = vec![];
        let mut new_state_row2: Vec<bool> = vec![];
        let len = self.state.len();
        for i in 0..len {
            if i > 1 {
                self.state[i - 2] = new_state_row1;
            }
            new_state_row1 = new_state_row2;
            new_state_row2 = vec![];
            for j in 0..self.state[i].len() {
                new_state_row2.push(self.dead_or_alive(i, j));
            }
        }
        self.state[len - 2] = new_state_row1;
        self.state[len - 1] = new_state_row2;
    }

    pub fn toggle_grid(&mut self) {
        self.enable_grid = !self.enable_grid;
    }

    pub fn revert(&mut self) {
        if let Some(prev_state) = self.state_history.pop_back() {
            self.state = prev_state;
        };
    }

    pub fn print(&mut self) {
        print!("{}", self);
    }

    pub fn update_and_print(&mut self) {
        self.update();
        print!("{}", self);
    }

    #[allow(dead_code)]
    pub fn load_from_csv(state_csv_file_path: &str) -> Self {
        let csv = fs::read_to_string(state_csv_file_path).expect("Failed to read csv");
        let state = csv.lines().fold(Vec::new(), |mut state, line| {
            state.push(
                line.split(',')
                    .map(|c| if c == "0" { true } else { false })
                    .collect::<Vec<_>>(),
            );
            state
        });

        Self::new(
            state,
            Path::new(state_csv_file_path)
                .file_stem()
                .and_then(|f| f.to_str())
                .unwrap_or(state_csv_file_path),
        )
    }

    pub fn gosper_glider_gun() -> Self {
        let mut state = vec![];
        for _ in 0..25 {
            let mut row = vec![];
            for _ in 0..40 {
                row.push(false);
            }
            state.push(row);
        }

        #[rustfmt::skip]
        let initial_alive_cells = vec![
            ( 5,  1), ( 5,  2), (6,  1), (6,  2), ( 6, 10), ( 6, 11),
            ( 5, 11), ( 6, 11), (7, 11), (4, 12), ( 5, 12), ( 6, 12),
            ( 7, 12), ( 8, 12), (3, 13), (4, 13), ( 8, 13), ( 9, 13),
            ( 5, 17), ( 6, 17), (7, 17), (5, 18), ( 6, 18), ( 7, 18),
            ( 4, 20), ( 3, 21), (5, 21), (2, 22), ( 6, 22), ( 3, 23),
            ( 4, 23), ( 5, 23), (1, 24), (2, 24), ( 6, 24), ( 7, 24),
            ( 3, 35), ( 4, 35), (3, 36), (4, 36), (23, 37), (23, 38),
            (24, 37), (24, 38),
        ];

        for (i, j) in initial_alive_cells {
            state[i][j] = true;
        }

        Self::new(state, "Gosper Glider Gun")
    }
}
