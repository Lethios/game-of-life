use crossterm::cursor::{MoveTo, MoveToNextLine};
use crossterm::event;
use crossterm::style::Print;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, terminal};
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

    fn get_char(&self, row: usize, col: usize) -> char {
        if row >= self.rows || col >= self.cols {
            panic!("Index out of bounds.");
        }

        self.buffer[row * self.cols + col]
    }

    fn set_char(&mut self, row: usize, col: usize, c: char) {
        if row >= self.rows || col >= self.cols {
            panic!("Index out of bounds.");
        }

        self.buffer[row * self.cols + col] = c;
    }
}

fn display(grid: &Grid) -> io::Result<()> {
    execute!(io::stdout(), MoveTo(0, 0))?;

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            execute!(io::stdout(), Print(grid.buffer[row * grid.cols + col]))?;
        }
        execute!(io::stdout(), MoveToNextLine(1))?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let size: (u16, u16) = terminal::size().expect("Error in fetching terminal size.");

    let mut grid = Grid::new(size);

    Ok(())
}
