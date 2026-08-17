use crossterm::cursor;
use crossterm::event;
use crossterm::style;
use crossterm::{execute, terminal};
use std::io;
use std::io::Write;

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

    fn render_to_string(&self) -> String {
        let mut output = String::with_capacity(self.rows * (self.cols + 1));

        for row in 0..self.rows {
            let start = row * self.cols;
            let end = start + self.cols;

            output.extend(self.buffer[start..end].iter());
            output.push('\n');
        }

        output
    }
}

fn display(output: String) -> io::Result<()> {
    execute!(io::stdout(), cursor::MoveTo(0, 0), style::Print(output))?;
    io::stdout().flush()
}

fn main() -> io::Result<()> {
    let size: (u16, u16) = terminal::size().expect("Error in fetching terminal size.");
    let mut grid = Grid::new(size);

    terminal::enable_raw_mode()?;
    execute!(io::stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;

    for i in 0..grid.cols {
        if i >= grid.rows {
            break;
        }

        grid.set_char(i, i, '█');
    }

    display(grid.render_to_string())?;

    while !matches!(event::read()?, event::Event::Key(_)) {}

    execute!(io::stdout(), terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
