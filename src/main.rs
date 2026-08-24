use crossterm::event::KeyCode;
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

fn quit() -> io::Result<()> {
    execute!(
        io::stdout(),
        terminal::LeaveAlternateScreen,
        cursor::Show,
        cursor::SetCursorStyle::DefaultUserShape
    )?;
    terminal::disable_raw_mode()?;

    std::process::exit(0)
}

fn pause(rows: u16, cols: u16) -> io::Result<()> {
    loop {
        if let event::Event::Key(k) = event::read()?
            && k.kind == event::KeyEventKind::Press
        {
            match k.code {
                event::KeyCode::Char('p') => {
                    break;
                }
                event::KeyCode::Char(' ') => draw(rows, cols)?,
                event::KeyCode::Char('q') | event::KeyCode::Esc => quit()?,
                _ => {}
            }
        }
    }

    Ok(())
}

fn draw(rows: u16, cols: u16) -> io::Result<()> {
    execute!(
        io::stdout(),
        cursor::MoveTo(cols / 2, rows / 2),
        cursor::SetCursorStyle::BlinkingUnderScore,
        cursor::Show
    )?;

    let mut draw_mode = false;

    loop {
        if let event::Event::Key(k) = event::read()? {
            if matches!(
                k.kind,
                event::KeyEventKind::Press | event::KeyEventKind::Repeat
            ) {
                match k.code {
                    event::KeyCode::Char('p') => {
                        // temp
                    }
                    event::KeyCode::Char(' ') => {
                        draw_mode = !draw_mode;

                        if draw_mode {
                            // temp
                        }
                    }
                    event::KeyCode::Char('w') | event::KeyCode::Up => {
                        execute!(io::stdout(), cursor::MoveUp(1))?
                    }
                    event::KeyCode::Char('a') | event::KeyCode::Left => {
                        execute!(io::stdout(), cursor::MoveLeft(1))?
                    }
                    event::KeyCode::Char('s') | event::KeyCode::Down => {
                        execute!(io::stdout(), cursor::MoveDown(1))?
                    }
                    event::KeyCode::Char('d') | event::KeyCode::Right => {
                        execute!(io::stdout(), cursor::MoveRight(1))?
                    }
                    event::KeyCode::Char('q') | event::KeyCode::Esc => quit()?,
                    _ => {}
                }
            }
        }
    }
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

    terminal::enable_raw_mode()?;
    execute!(io::stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;

    loop {
        let frame_start = std::time::Instant::now();

        render(&game)?;
        game.tick();

        // Check if a key is pressed
        if event::poll(std::time::Duration::ZERO)?
            && let event::Event::Key(k) = event::read()?
            && k.kind == event::KeyEventKind::Press
        {
            match k.code {
                // Pause if p is pressed
                event::KeyCode::Char('p') => pause(rows, cols)?,

                // Exit loop if q is pressed
                event::KeyCode::Char('q') | event::KeyCode::Esc => break,

                _ => {}
            }
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
