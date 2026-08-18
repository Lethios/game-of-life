use rand::{self, RngExt, rngs::StdRng};

pub struct Grid {
    rows: usize,
    cols: usize,
    buffer: Vec<char>,
}

impl Grid {
    // Construct Grid with empty buffer
    pub fn new(size: (u16, u16)) -> Self {
        let rows = size.1 as usize;
        let cols = size.0 as usize;

        Self {
            rows,
            cols,
            buffer: vec![' '; rows * cols],
        }
    }

    // Construct Grid with randomized buffer
    pub fn new_random(size: (u16, u16), spawn_probability: f64, seed: u64) -> Self {
        let rows = size.1 as usize;
        let cols = size.0 as usize;

        let mut rng: StdRng = rand::SeedableRng::seed_from_u64(seed);

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

    pub fn buffer(&self) -> &[char] {
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
