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

fn input(
    event: Event,
    field: &mut [[Masu; 8]; 8],
    cursor: &mut (usize, usize),
    end: &mut bool,
) -> Result<()> {
    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Esc, ..
        }) => *end = true,
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
        _ => {}
    }
    Ok(())
}

fn view<T: std::io::Write>(
    output: &mut T,
    field: &[[Masu; 8]; 8],
    cursor: &(usize, usize),
) -> Result<()> {
    execute!(output, MoveTo(0, 0))?;
    for y in 0..8 {
        for x in 0..8 {
            if x == cursor.0 && y == cursor.1 {
                execute!(output, SetBackgroundColor(Color::Reset),)?;
            } else {
                execute!(output, SetBackgroundColor(Color::DarkGreen),)?;
            }
            match field[y][x] {
                Masu::Empty => execute!(output, Print("  "))?,
                Masu::Black => execute!(output, Print('⚫'))?,
                Masu::White => execute!(output, Print('⚪'))?,
            }
        }
        execute!(output, Print("\n"))?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut field = [[Masu::Empty; 8]; 8];
    let mut cursor = (0, 0);
    let mut end = false;
    enable_raw_mode()?;
    execute!(std::io::stdout(), Hide, EnterAlternateScreen,)?;
    while !end {
        view(&mut std::io::stdout(), &field, &cursor)?;
        input(read()?, &mut field, &mut cursor, &mut end)?;
    }

    execute!(std::io::stdout(), Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
    #[test]
    fn input_test() {
        let mut field = [[Masu::Empty; 8]; 8];
        let mut cursor = (0, 0);
        let mut end = false;
        let wkey = Event::Key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
        super::input(wkey, &mut field, &mut cursor, &mut end).unwrap();
        assert!(field[0][0] == Masu::White);
        let rightkey = Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
        super::input(rightkey, &mut field, &mut cursor, &mut end).unwrap();
        assert!(cursor.0 == 1);
        assert!(cursor.1 == 0);
        let downkey = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        super::input(downkey, &mut field, &mut cursor, &mut end).unwrap();
        assert!(cursor.0 == 1);
        assert!(cursor.1 == 1);
        let bkey = Event::Key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
        super::input(bkey, &mut field, &mut cursor, &mut end).unwrap();
        assert!(field[1][1] == Masu::Black);
        let leftkey = Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
        super::input(leftkey, &mut field, &mut cursor, &mut end).unwrap();
        assert!(cursor.0 == 0);
        assert!(cursor.1 == 1);
        let upkey = Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        super::input(upkey, &mut field, &mut cursor, &mut end).unwrap();
        assert!(cursor.0 == 0);
        assert!(cursor.1 == 0);
        let backspace = Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
        super::input(backspace, &mut field, &mut cursor, &mut end).unwrap();
        assert!(field[0][0] == Masu::Empty);
        let esc = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        super::input(esc, &mut field, &mut cursor, &mut end).unwrap();
        assert!(end);
    }
}
