use crossterm::{cursor, event, execute, style, terminal};
use std::io;

mod args;
mod game;
mod grid;

use crate::args::parse_args;
use crate::game::Game;
use crate::grid::Grid;

// Display to terminal
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
    // Parse arguments from command line
    let args = parse_args().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });

    let frame_duration = std::time::Duration::from_secs_f64(1.0 / args.speed);

    let (cols, rows) = terminal::size().expect("Error in fetching terminal size.");
    let grid = Grid::new_random(rows, cols, args.spawn, args.seed);
    let mut game = Game::new(grid, args.rulestring);
    let mut is_paused = false;

    terminal::enable_raw_mode()?;
    execute!(io::stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;

    'outer: loop {
        let frame_start = std::time::Instant::now();

        render(&game)?;
        game.tick();

        // Check if p is pressed
        if event::poll(std::time::Duration::ZERO)?
            && let event::Event::Key(k) = event::read()?
            && k.kind == event::KeyEventKind::Press
            && matches!(k.code, event::KeyCode::Char('p'))
        {
            is_paused = true;
        }

        // Pause game until p is pressed again
        if is_paused {
            loop {
                if let event::Event::Key(k) = event::read()?
                    && k.kind == event::KeyEventKind::Press
                {
                    if matches!(k.code, event::KeyCode::Char('p')) {
                        is_paused = false;
                        break;
                    }

                    if matches!(k.code, event::KeyCode::Char('q') | event::KeyCode::Esc) {
                        break 'outer;
                    }
                }
            }
        }

        // Exit if q or Esc pressed
        if event::poll(std::time::Duration::ZERO)?
            && let event::Event::Key(k) = event::read()?
            && k.kind == event::KeyEventKind::Press
            && matches!(k.code, event::KeyCode::Char('q') | event::KeyCode::Esc)
        {
            break;
        }

        // Cap frame rate by sleeping for the remaining time in the frame
        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }

    execute!(io::stdout(), terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
