use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, SetBackgroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};

#[derive(Clone, Copy, PartialEq)]
enum Masu {
    Empty,
    Black,
    White,
}

fn main() -> Result<()> {
    let mut field = [[Masu::Empty; 8]; 8];
    let mut cursor = (0, 0);
    enable_raw_mode()?;
    execute!(std::io::stdout(), Hide, EnterAlternateScreen,)?;
    loop {
        execute!(std::io::stdout(), MoveTo(0, 0))?;
        for y in 0..8 {
            for x in 0..8 {
                if x == cursor.0 && y == cursor.1 {
                    execute!(std::io::stdout(), SetBackgroundColor(Color::Reset),)?;
                } else {
                    execute!(std::io::stdout(), SetBackgroundColor(Color::DarkGreen),)?;
                }
                match field[y][x] {
                    Masu::Empty => execute!(std::io::stdout(), Print("  "))?,
                    Masu::Black => execute!(std::io::stdout(), Print('⚫'))?,
                    Masu::White => execute!(std::io::stdout(), Print('⚪'))?,
                }
            }
            execute!(std::io::stdout(), Print("\n"))?;
        }
        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => {
                if cursor.1 > 0 {
                    cursor.1 -= 1;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => {
                if cursor.1 < 7 {
                    cursor.1 += 1;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => {
                if cursor.0 > 0 {
                    cursor.0 -= 1;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => {
                if cursor.0 < 7 {
                    cursor.0 += 1;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) => field[cursor.1][cursor.0] = Masu::Empty,
            Event::Key(KeyEvent {
                code: KeyCode::Char('b'),
                ..
            }) => field[cursor.1][cursor.0] = Masu::Black,
            Event::Key(KeyEvent {
                code: KeyCode::Char('w'),
                ..
            }) => field[cursor.1][cursor.0] = Masu::White,
            _ => continue,
        }
    }

    execute!(std::io::stdout(), Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
