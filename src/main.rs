use crossterm::{cursor, event, execute, style, terminal};
use rand::{self, RngExt, rngs::StdRng};
use std::io;

struct Grid {
    rows: usize,
    cols: usize,
    buffer: Vec<char>,
}

impl Grid {
    fn new(size: (u16, u16)) -> Self {
        let rows = size.1 as usize;
        let cols = size.0 as usize;

        Self {
            rows,
            cols,
            buffer: vec![' '; rows * cols],
        }
    }

    fn new_random(size: (u16, u16), spawn_probability: f64, seed: Option<u64>) -> Self {
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

    fn get_cell_state(&self, row: usize, col: usize) -> bool {
        if row >= self.rows || col >= self.cols {
            panic!("Index out of bounds.");
        }

        if self.buffer[row * self.cols + col] == '█' {
            return true;
        }

        false
    }

    fn set_cell_state(&mut self, row: usize, col: usize, c: char) {
        if row >= self.rows || col >= self.cols {
            panic!("Index out of bounds.");
        }

        self.buffer[row * self.cols + col] = c;
    }
}

struct Game {
    game: Grid,
    birth: [bool; 9],
    survival: [bool; 9],
}

impl Game {
    fn new(game: Grid, rulestring: Option<String>) -> Option<Self> {
        let rule = match rulestring {
            Some(rule) => rule,
            None => "B3/S23".to_string(),
        };

        let (birth, survival) = Self::parse_rulestring(&rule)?;

        Some(Self {
            game,
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

    fn next_cell_state(&self, row: usize, col: usize) -> bool {
        let curr_state = self.game.get_cell_state(row, col);
        let mut live_neighbors: usize = 0;

        for r in (row - 1)..=(row + 1) {
            for c in (col - 1)..=(col + 1) {
                if !(0..self.game.rows).contains(&r) || !(0..self.game.cols).contains(&c) {
                    continue;
                }
                if r == row && c == col {
                    continue;
                }

                if self.game.get_cell_state(r, c) {
                    live_neighbors += 1;
                }
            }
        }

        if curr_state {
            if self.survival[live_neighbors] {
                return true;
            }
        } else {
            if self.birth[live_neighbors] {
                return true;
            }
        }

        false
    }

    fn tick(&self) -> Grid {
        let mut next_grid = Grid::new((self.game.rows as u16, self.game.cols as u16));

        for row in 0..self.game.rows {
            for col in 0..self.game.cols {
                if Self::next_cell_state(&self, row, col) {
                    next_grid.set_cell_state(row, col, '█');
                } else {
                    next_grid.set_cell_state(row, col, ' ');
                }
            }
        }

        next_grid
    }
}

fn render(grid: &Grid) -> io::Result<()> {
    let mut output = String::with_capacity(grid.rows * (grid.cols + 1));

    for row in 0..grid.rows {
        let start = row * grid.cols;
        let end = start + grid.cols;

        output.extend(grid.buffer[start..end].iter());
        output.push('\n');
    }

    execute!(io::stdout(), cursor::MoveTo(0, 0), style::Print(output))
}

fn main() -> io::Result<()> {
    let size: (u16, u16) = terminal::size().expect("Error in fetching terminal size.");
    let grid = Grid::new_random(size, 0.5, Some(0));

    terminal::enable_raw_mode()?;
    execute!(io::stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;

    render(&grid)?;

    loop {
        if let event::Event::Key(k) = event::read()? {
            if k.kind == event::KeyEventKind::Press
                && matches!(k.code, event::KeyCode::Char('q') | event::KeyCode::Esc)
            {
                break;
            }
        }
    }

    execute!(io::stdout(), terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
