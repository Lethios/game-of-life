use crossterm::{cursor, event, execute, style, terminal};
use std::io;

mod game;
mod grid;

use crate::game::Game;
use crate::grid::Grid;

fn render(game: &Game) -> io::Result<()> {
    let rows = game.grid().rows();
    let cols = game.grid().cols();

    let mut output = String::with_capacity(rows * (cols + 1));

    for row in 0..rows {
        let start = row * cols;
        let end = start + cols;

        output.extend(game.grid().buffer()[start..end].iter());

        if row < rows - 1 {
            output.push('\n');
        }
    }

    execute!(io::stdout(), cursor::MoveTo(0, 0), style::Print(output))
}

fn main() -> io::Result<()> {
    let size: (u16, u16) = terminal::size().expect("Error in fetching terminal size.");

    let grid = Grid::new_random((size.0, size.1), 0.5, Some(0));
    let mut game = Game::new(grid, None).ok_or("err").expect("err");

    let game_speed = 15.0;
    let frame_duration = std::time::Duration::from_secs_f64(1.0 / game_speed);

    terminal::enable_raw_mode()?;
    execute!(io::stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;

    loop {
        let frame_start = std::time::Instant::now();

        render(&game)?;
        game.tick();

        if event::poll(std::time::Duration::ZERO)? {
            if let event::Event::Key(k) = event::read()? {
                if k.kind == event::KeyEventKind::Press
                    && matches!(k.code, event::KeyCode::Char('q') | event::KeyCode::Esc)
                {
                    break;
                }
            }
        }

        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }
    execute!(io::stdout(), terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
