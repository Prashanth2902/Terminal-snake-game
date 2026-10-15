mod game;

use std::io::{self, Stdout, Write};
use std::time::Instant;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute, queue,
    style::{self, Color, Stylize},
    terminal::{self, ClearType},
};

use game::{Direction, Game, State};

const WIDTH: u16 = 30;
const HEIGHT: u16 = 20;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let result = run(&mut stdout);

    // Always restore the terminal, even if the game loop errored.
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    result
}

fn run(stdout: &mut Stdout) -> io::Result<()> {
    let mut game = Game::new(WIDTH, HEIGHT);
    let mut paused = false;
    let mut last_tick = Instant::now();

    execute!(stdout, terminal::Clear(ClearType::All))?;

    loop {
        draw(stdout, &game, paused)?;

        let timeout = game.tick_interval().saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // Windows reports both press and release; only act on presses.
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('p') | KeyCode::Char(' ') if game.state == State::Playing => {
                        paused = !paused
                    }
                    KeyCode::Char('r') if game.state == State::GameOver => {
                        game = Game::new(WIDTH, HEIGHT);
                        paused = false;
                        execute!(stdout, terminal::Clear(ClearType::All))?;
                    }
                    KeyCode::Up | KeyCode::Char('w') => game.turn(Direction::Up),
                    KeyCode::Down | KeyCode::Char('s') => game.turn(Direction::Down),
                    KeyCode::Left | KeyCode::Char('a') => game.turn(Direction::Left),
                    KeyCode::Right | KeyCode::Char('d') => game.turn(Direction::Right),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= game.tick_interval() {
            if !paused {
                game.tick();
            }
            last_tick = Instant::now();
        }
    }
}

/// Each board cell is drawn two characters wide so it looks roughly square.
fn draw(stdout: &mut Stdout, game: &Game, paused: bool) -> io::Result<()> {
    let wall = "██".with(Color::DarkGrey);

    queue!(stdout, cursor::MoveTo(0, 0))?;
    for _ in 0..game.width + 2 {
        queue!(stdout, style::PrintStyledContent(wall))?;
    }

    for y in 0..game.height {
        queue!(stdout, cursor::MoveTo(0, y + 1), style::PrintStyledContent(wall))?;
        for x in 0..game.width {
            let pos = (x, y);
            if game.snake.front() == Some(&pos) {
                queue!(stdout, style::PrintStyledContent("██".with(Color::Green)))?;
            } else if game.snake.contains(&pos) {
                queue!(stdout, style::PrintStyledContent("██".with(Color::DarkGreen)))?;
            } else if game.food == pos {
                queue!(stdout, style::PrintStyledContent("()".with(Color::Red)))?;
            } else {
                queue!(stdout, style::Print("  "))?;
            }
        }
        queue!(stdout, style::PrintStyledContent(wall))?;
    }

    queue!(stdout, cursor::MoveTo(0, game.height + 1))?;
    for _ in 0..game.width + 2 {
        queue!(stdout, style::PrintStyledContent(wall))?;
    }

    let status = match (game.state, paused) {
        (State::GameOver, _) => format!("GAME OVER  Score: {}   [R] restart  [Q] quit", game.score),
        (State::Won, _) => format!("YOU WIN!  Score: {}   [Q] quit", game.score),
        (_, true) => format!("PAUSED  Score: {}   [P] resume  [Q] quit", game.score),
        _ => format!("Score: {}   [Arrows/WASD] move  [P] pause  [Q] quit", game.score),
    };
    queue!(
        stdout,
        cursor::MoveTo(0, game.height + 3),
        terminal::Clear(ClearType::CurrentLine),
        style::Print(status)
    )?;

    stdout.flush()
}
