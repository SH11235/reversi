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
    Putted(DiscColor),
}

#[derive(Clone, Copy, PartialEq)]
enum DiscColor {
    Black,
    White,
}

fn input(
    event: Event,
    field: &mut [[Masu; 8]; 8],
    cursor: &mut (usize, usize),
    end: &mut bool,
    turn_color: &mut DiscColor,
) -> Result<()> {
    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Esc, ..
        }) => *end = true,
        Event::Key(KeyEvent {
            code: KeyCode::Char('p'),
            ..
        }) => *turn_color = get_another_color(*turn_color),
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
            if check_putable(&field, &cursor, *turn_color) {
                field[cursor.1][cursor.0] = Masu::Putted(*turn_color);
                auto_reverse(field, *cursor, *turn_color);
                *turn_color = get_another_color(*turn_color);
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
    color: &DiscColor,
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
                Masu::Putted(DiscColor::Black) => execute!(output, Print('⚫'))?,
                Masu::Putted(DiscColor::White) => execute!(output, Print('⚪'))?,
            }
        }
        execute!(output, Print("\n"))?;
    }
    match color {
        DiscColor::Black => execute!(output, Print("黒のターンです"))?,
        DiscColor::White => execute!(output, Print("白のターンです"))?,
    }
    Ok(())
}

fn init_field(field: &mut [[Masu; 8]; 8]) {
    *field = [[Masu::Empty; 8]; 8];
    field[3][3] = Masu::Putted(DiscColor::Black);
    field[4][4] = Masu::Putted(DiscColor::Black);
    field[3][4] = Masu::Putted(DiscColor::White);
    field[4][3] = Masu::Putted(DiscColor::White);
}

fn auto_reverse(field: &mut [[Masu; 8]; 8], point: (usize, usize), turn_color: DiscColor) {
    get_reversable_masu(field, &point, turn_color)
        .iter()
        .for_each(|p| {
            field[p.1][p.0] = field[point.1][point.0];
        });
}

fn get_reversable_masu(
    field: &[[Masu; 8]; 8],
    point: &(usize, usize),
    color: DiscColor,
) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];
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
            if field[y as usize][x as usize] == Masu::Putted(color) {
                break reverse_count;
            }
        };
        for j in 1..reverse_count {
            let x = point.0 as i32 + direction.0 * j;
            let y = point.1 as i32 + direction.1 * j;
            result.push((x as usize, y as usize));
        }
    }
    result
}

fn check_putable(field: &[[Masu; 8]; 8], point: &(usize, usize), turn_color: DiscColor) -> bool {
    if field[point.0][point.1] != Masu::Empty {
        return false;
    }
    if get_reversable_masu(field, point, turn_color).len() == 0 {
        return false;
    }
    return true;
}

fn get_another_color(color: DiscColor) -> DiscColor {
    match color {
        DiscColor::Black => DiscColor::White,
        DiscColor::White => DiscColor::Black,
    }
}

fn main() -> Result<()> {
    let mut field = [[Masu::Empty; 8]; 8];
    init_field(&mut field);
    let mut cursor = (0, 0);
    let mut end = false;
    let mut color = DiscColor::White;
    enable_raw_mode()?;
    execute!(std::io::stdout(), Hide, EnterAlternateScreen,)?;
    while !end {
        view(&mut std::io::stdout(), &field, &cursor, &color)?;
        input(read()?, &mut field, &mut cursor, &mut end, &mut color)?;
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
        let mut color = DiscColor::White;
        let rightkey = Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
        super::input(rightkey, &mut field, &mut cursor, &mut end, &mut color).unwrap();
        assert!(cursor.0 == 1);
        assert!(cursor.1 == 0);
        let rightkey = Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
        super::input(rightkey, &mut field, &mut cursor, &mut end, &mut color).unwrap();
        assert!(cursor.0 == 2);
        assert!(cursor.1 == 0);
        let downkey = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        super::input(downkey, &mut field, &mut cursor, &mut end, &mut color).unwrap();
        assert!(cursor.0 == 2);
        assert!(cursor.1 == 1);
        let downkey = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        super::input(downkey, &mut field, &mut cursor, &mut end, &mut color).unwrap();
        assert!(cursor.0 == 2);
        assert!(cursor.1 == 2);
        let downkey = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        super::input(downkey, &mut field, &mut cursor, &mut end, &mut color).unwrap();
        assert!(cursor.0 == 2);
        assert!(cursor.1 == 3);
        let white_turn_enter = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        super::input(
            white_turn_enter,
            &mut field,
            &mut cursor,
            &mut end,
            &mut color,
        )
        .unwrap();
        assert!(field[3][2] == Masu::Putted(DiscColor::White));
        assert!(field[3][3] == Masu::Putted(DiscColor::White));
        let upkey = Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        super::input(upkey, &mut field, &mut cursor, &mut end, &mut color).unwrap();
        assert!(cursor.0 == 2);
        assert!(cursor.1 == 2);
        let black_turn_enter = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        super::input(
            black_turn_enter,
            &mut field,
            &mut cursor,
            &mut end,
            &mut color,
        )
        .unwrap();
        assert!(field[2][2] == Masu::Putted(DiscColor::Black));
        assert!(field[3][3] == Masu::Putted(DiscColor::Black));
        let leftkey = Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
        super::input(leftkey, &mut field, &mut cursor, &mut end, &mut color).unwrap();
        assert!(cursor.0 == 1);
        assert!(cursor.1 == 2);
        let esc = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        super::input(esc, &mut field, &mut cursor, &mut end, &mut color).unwrap();
        assert!(end);
    }

    #[test]
    fn view_test() {
        let mut field = [[Masu::Empty; 8]; 8];
        let cursor = (0, 0);
        let color = DiscColor::White;
        field[3][3] = Masu::Putted(DiscColor::Black);
        field[4][4] = Masu::Putted(DiscColor::Black);
        field[3][4] = Masu::Putted(DiscColor::White);
        field[4][3] = Masu::Putted(DiscColor::White);
        let mut buf = Vec::<u8>::new();
        let mut assert_buf = Vec::<u8>::new();
        view(&mut buf, &field, &cursor, &color).unwrap();
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
        assert!(field[3][3] == Masu::Putted(DiscColor::Black));
        assert!(field[4][4] == Masu::Putted(DiscColor::Black));
        assert!(field[3][4] == Masu::Putted(DiscColor::White));
        assert!(field[4][3] == Masu::Putted(DiscColor::White));
    }
    #[test]
    fn auto_reverse_test() {
        let mut field = [[Masu::Empty; 8]; 8];
        init_field(&mut field);
        field[4][5] = Masu::Putted(DiscColor::White);
        auto_reverse(&mut field, (5, 4), DiscColor::White);
        assert!(field[4][4] == Masu::Putted(DiscColor::White));

        init_field(&mut field);
        field[3][5] = Masu::Putted(DiscColor::White);
        field[3][6] = Masu::Putted(DiscColor::White);
        field[3][7] = Masu::Putted(DiscColor::Black);
        auto_reverse(&mut field, (7, 3), DiscColor::Black);
        assert!(field[3][5] == Masu::Putted(DiscColor::Black));
        assert!(field[3][6] == Masu::Putted(DiscColor::Black));
        assert!(field[3][7] == Masu::Putted(DiscColor::Black));
    }
    #[test]
    fn check_putable_test() {
        let mut field = [[Masu::Empty; 8]; 8];
        init_field(&mut field);
        let color = DiscColor::Black;
        assert!(!check_putable(&field, &(0, 0), color));
        assert!(check_putable(&field, &(4, 2), color));
    }
    #[test]
    fn get_reversable_masu_test() {
        let mut field = [[Masu::Empty; 8]; 8];
        init_field(&mut field);
        let color = DiscColor::White;
        let point = (2, 3);
        let mut reversable_masu = Vec::<(usize, usize)>::new();
        reversable_masu.push((3, 3));
        assert!(get_reversable_masu(&field, &point, color) == reversable_masu);
    }
    #[test]
    fn get_another_color_test() {
        assert!(get_another_color(DiscColor::White) == DiscColor::Black);
        assert!(get_another_color(DiscColor::Black) == DiscColor::White);
    }
}
