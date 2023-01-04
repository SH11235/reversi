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

enum Turn {
    Black,
    White,
}

fn input(
    event: Event,
    field: &mut [[Masu; 8]; 8],
    cursor: &mut (usize, usize),
    end: &mut bool,
    turn: &mut Turn,
) -> Result<()> {
    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Esc, ..
        }) => *end = true,
        Event::Key(KeyEvent {
            code: KeyCode::Char('p'),
            ..
        }) => match turn {
            Turn::Black => {
                *turn = Turn::White;
            }
            Turn::White => {
                *turn = Turn::Black;
            }
        },
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
            code: KeyCode::Enter,
            ..
        }) => {
            if check_putable(&field, &cursor, &turn) {
                match turn {
                    Turn::Black => {
                        field[cursor.1][cursor.0] = Masu::Black;
                        auto_reverse(field, *cursor);
                        *turn = Turn::White;
                    }
                    Turn::White => {
                        field[cursor.1][cursor.0] = Masu::White;
                        auto_reverse(field, *cursor);
                        *turn = Turn::Black;
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn view<T: std::io::Write>(
    output: &mut T,
    field: &[[Masu; 8]; 8],
    cursor: &(usize, usize),
    turn: &Turn,
) -> Result<()> {
    execute!(output, MoveTo(0, 0))?;
    for y in 0..8 {
        for x in 0..8 {
            if x == cursor.0 && y == cursor.1 {
                execute!(output, SetBackgroundColor(Color::Reset),)?;
            } else {
                if (x + y) % 2 == 0 {
                    execute!(output, SetBackgroundColor(Color::DarkGreen),)?;
                } else {
                    execute!(output, SetBackgroundColor(Color::Green),)?;
                }
            }
            match field[y][x] {
                Masu::Empty => execute!(output, Print("  "))?,
                Masu::Black => execute!(output, Print('⚫'))?,
                Masu::White => execute!(output, Print('⚪'))?,
            }
        }
        execute!(output, Print("\n"))?;
    }
    match turn {
        Turn::Black => execute!(output, Print("黒のターンです"))?,
        Turn::White => execute!(output, Print("白のターンです"))?,
    }
    Ok(())
}

fn init_field(field: &mut [[Masu; 8]; 8]) {
    *field = [[Masu::Empty; 8]; 8];
    field[3][3] = Masu::Black;
    field[4][4] = Masu::Black;
    field[3][4] = Masu::White;
    field[4][3] = Masu::White;
}

fn auto_reverse(field: &mut [[Masu; 8]; 8], point: (usize, usize)) {
    // 8方向に対して、反転できるかを調べる
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for direction in directions.iter() {
        let mut x = point.0 as i32;
        let mut y = point.1 as i32;
        let mut reverse_count = 0;
        let reverse_count = loop {
            reverse_count += 1;
            x += direction.0;
            y += direction.1;
            if x < 0 || x > 7 || y < 0 || y > 7 {
                break 0;
            }
            if field[y as usize][x as usize] == Masu::Empty {
                break 0;
            }
            if field[y as usize][x as usize] == field[point.1][point.0] {
                break reverse_count;
            }
        };
        if reverse_count > 0 {
            x = point.0 as i32;
            y = point.1 as i32;
            for _ in 0..reverse_count {
                x += direction.0;
                y += direction.1;
                field[y as usize][x as usize] = field[point.1][point.0];
            }
        }
    }
}

fn check_putable(field: &[[Masu; 8]; 8], point: &(usize, usize), turn: &Turn) -> bool {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let turn_color = match turn {
        Turn::Black => Masu::Black,
        Turn::White => Masu::White,
    };
    for direction in directions.iter() {
        let mut x = point.0 as i32;
        let mut y = point.1 as i32;
        let mut reverse_count = 0;
        let reverse_count = loop {
            reverse_count += 1;
            x += direction.0;
            y += direction.1;
            if x < 0 || x > 7 || y < 0 || y > 7 {
                break 0;
            }
            if field[y as usize][x as usize] == Masu::Empty {
                break 0;
            }
            if field[y as usize][x as usize] == turn_color {
                break reverse_count;
            }
        };
        if reverse_count > 1 {
            return true;
        }
    }
    false
}

fn main() -> Result<()> {
    let mut field = [[Masu::Empty; 8]; 8];
    init_field(&mut field);
    let mut cursor = (0, 0);
    let mut end = false;
    let mut turn = Turn::White;
    enable_raw_mode()?;
    execute!(std::io::stdout(), Hide, EnterAlternateScreen,)?;
    while !end {
        view(&mut std::io::stdout(), &field, &cursor, &turn)?;
        input(read()?, &mut field, &mut cursor, &mut end, &mut turn)?;
    }

    execute!(std::io::stdout(), Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn input_test() {
        let mut field = [[Masu::Empty; 8]; 8];
        init_field(&mut field);
        let mut cursor = (0, 0);
        let mut end = false;
        let mut turn = Turn::White;
        let rightkey = Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
        super::input(rightkey, &mut field, &mut cursor, &mut end, &mut turn).unwrap();
        assert!(cursor.0 == 1);
        assert!(cursor.1 == 0);
        let rightkey = Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
        super::input(rightkey, &mut field, &mut cursor, &mut end, &mut turn).unwrap();
        assert!(cursor.0 == 2);
        assert!(cursor.1 == 0);
        let downkey = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        super::input(downkey, &mut field, &mut cursor, &mut end, &mut turn).unwrap();
        assert!(cursor.0 == 2);
        assert!(cursor.1 == 1);
        let downkey = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        super::input(downkey, &mut field, &mut cursor, &mut end, &mut turn).unwrap();
        assert!(cursor.0 == 2);
        assert!(cursor.1 == 2);
        let downkey = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        super::input(downkey, &mut field, &mut cursor, &mut end, &mut turn).unwrap();
        assert!(cursor.0 == 2);
        assert!(cursor.1 == 3);
        let white_turn_enter = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        super::input(
            white_turn_enter,
            &mut field,
            &mut cursor,
            &mut end,
            &mut turn,
        )
        .unwrap();
        assert!(field[3][2] == Masu::White);
        assert!(field[3][3] == Masu::White);
        let upkey = Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        super::input(upkey, &mut field, &mut cursor, &mut end, &mut turn).unwrap();
        assert!(cursor.0 == 2);
        assert!(cursor.1 == 2);
        let black_turn_enter = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        super::input(
            black_turn_enter,
            &mut field,
            &mut cursor,
            &mut end,
            &mut turn,
        )
        .unwrap();
        assert!(field[2][2] == Masu::Black);
        assert!(field[3][3] == Masu::Black);
        let leftkey = Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
        super::input(leftkey, &mut field, &mut cursor, &mut end, &mut turn).unwrap();
        assert!(cursor.0 == 1);
        assert!(cursor.1 == 2);
        let esc = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        super::input(esc, &mut field, &mut cursor, &mut end, &mut turn).unwrap();
        assert!(end);
    }

    #[test]
    fn view_test() {
        let mut field = [[Masu::Empty; 8]; 8];
        let cursor = (0, 0);
        let turn = Turn::White;
        field[3][3] = Masu::Black;
        field[4][4] = Masu::Black;
        field[3][4] = Masu::White;
        field[4][3] = Masu::White;
        let mut buf = Vec::<u8>::new();
        let mut assert_buf = Vec::<u8>::new();
        view(&mut buf, &field, &cursor, &turn).unwrap();
        // let mut f = File::create("testdata/initview").unwrap();
        // use std::io::Write;
        // f.write_all(buf.into_boxed_slice().as_ref()).unwrap();
        let mut f = File::open("testdata/initview").unwrap();
        f.read_to_end(&mut assert_buf).unwrap();
        assert!(buf == assert_buf);
    }

    #[test]
    fn init_field_test() {
        let mut field = [[Masu::Empty; 8]; 8];
        init_field(&mut field);
        assert!(field[3][3] == Masu::Black);
        assert!(field[4][4] == Masu::Black);
        assert!(field[3][4] == Masu::White);
        assert!(field[4][3] == Masu::White);
    }
    #[test]
    fn auto_reverse_test() {
        let mut field = [[Masu::Empty; 8]; 8];
        init_field(&mut field);
        field[4][5] = Masu::White;
        auto_reverse(&mut field, (5, 4));
        assert!(field[4][4] == Masu::White);

        init_field(&mut field);
        field[3][5] = Masu::White;
        field[3][6] = Masu::White;
        field[3][7] = Masu::Black;
        auto_reverse(&mut field, (7, 3));
        assert!(field[3][5] == Masu::Black);
        assert!(field[3][6] == Masu::Black);
        assert!(field[3][7] == Masu::Black);
    }
    #[test]
    fn check_putable_test() {
        let mut field = [[Masu::Empty; 8]; 8];
        init_field(&mut field);
        let turn = Turn::Black;
        assert!(!check_putable(&field, &(0, 0), &turn));
        assert!(check_putable(&field, &(4, 2), &turn));
    }
}
