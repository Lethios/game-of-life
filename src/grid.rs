use rand::{self, RngExt, rngs::StdRng};

pub struct Grid {
    rows: usize,
    cols: usize,
    buffer: Vec<char>,
}

impl Grid {
    pub fn new(size: (u16, u16)) -> Self {
        let rows = size.0 as usize;
        let cols = size.1 as usize;

        Self {
            rows,
            cols,
            buffer: vec![' '; rows * cols],
        }
    }

    pub fn new_random(size: (u16, u16), spawn_probability: f64, seed: Option<u64>) -> Self {
        let rows = size.1 as usize;
        let cols = size.0 as usize;

        let mut rng: StdRng = match seed {
            Some(seed) => rand::SeedableRng::seed_from_u64(seed),
            None => rand::make_rng(),
        };

        Self {
            rows,
            cols,
            buffer: (0..rows * cols)
                .map(|_| {
                    if rng.random_bool(spawn_probability) {
                        '█'
                    } else {
                        ' '
                    }
                })
                .collect(),
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn buffer(&self) -> &Vec<char> {
        &self.buffer
    }

    pub fn get_cell_state(&self, row: usize, col: usize) -> bool {
        if row >= self.rows || col >= self.cols {
            panic!("Index out of bounds.");
        }

        self.buffer[row * self.cols + col] == '█'
    }

    pub fn set_cell_state(&mut self, row: usize, col: usize, c: char) {
        if row >= self.rows || col >= self.cols {
            panic!("Index out of bounds.");
        }

        self.buffer[row * self.cols + col] = c;
    }
}
