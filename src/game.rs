use crate::grid::Grid;

pub struct Game {
    grid: Grid,
    birth: [bool; 9],
    survival: [bool; 9],
}

impl Game {
    pub fn new(grid: Grid, rulestring: Option<&str>) -> Option<Self> {
        let rule = match rulestring {
            Some(rule) => rule,
            None => "B3/S23",
        };

        let (birth, survival) = Self::parse_rulestring(rule)?;

        Some(Self {
            grid,
            birth,
            survival,
        })
    }

    fn parse_rulestring(rulestring: &str) -> Option<([bool; 9], [bool; 9])> {
        let mut birth_rules = [false; 9];
        let mut survival_rules = [false; 9];

        let parts: Vec<&str> = rulestring.split("/").collect();

        let birth = parts.get(0)?.strip_prefix("B")?;
        let survival = parts.get(1)?.strip_prefix("S")?;

        for b in birth.chars() {
            let num = b.to_digit(10)?;
            if num > 8 {
                return None;
            }

            birth_rules[num as usize] = true;
        }

        for s in survival.chars() {
            let num = s.to_digit(10)?;
            if num > 8 {
                return None;
            }

            survival_rules[num as usize] = true;
        }

        Some((birth_rules, survival_rules))
    }

    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    fn next_cell_state(&self, row: usize, col: usize) -> bool {
        let curr_state = self.grid.get_cell_state(row, col);
        let mut live_neighbors: usize = 0;

        for r in row.saturating_sub(1)..=row.saturating_add(1) {
            for c in col.saturating_sub(1)..=col.saturating_add(1) {
                if r >= self.grid.rows() || c >= self.grid.cols() {
                    continue;
                }
                if r == row && c == col {
                    continue;
                }

                if self.grid.get_cell_state(r, c) {
                    live_neighbors += 1;
                }
            }
        }

        if curr_state && self.survival[live_neighbors] {
            return true;
        }

        if !curr_state && self.birth[live_neighbors] {
            return true;
        }

        false
    }

    pub fn tick(&mut self) {
        let rows = self.grid.rows();
        let cols = self.grid.cols();

        let mut next_grid = Grid::new((cols as u16, rows as u16));

        for row in 0..rows {
            for col in 0..cols {
                if Self::next_cell_state(&self, row, col) {
                    next_grid.set_cell_state(row, col, '█');
                } else {
                    next_grid.set_cell_state(row, col, ' ');
                }
            }
        }

        self.grid = next_grid;
    }
}
