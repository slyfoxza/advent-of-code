use std::borrow::Cow;
use std::char;
use std::cmp;
use std::io::{self, Read};

fn get_button(line: &str, last: char) -> char {
    let digit = last.to_digit(10).unwrap();
    let mut x = (digit - 1) % 3;
    let mut y = (digit - 1) / 3;
    for c in line.chars() {
        match c {
            'U' => y = y.saturating_sub(1),
            'D' => y = cmp::min(2, y + 1),
            'L' => x = x.saturating_sub(1),
            'R' => x = cmp::min(2, x + 1),
            _ => {}
        }
    }
    char::from_digit(y * 3 + x + 1, 10).unwrap()
}

static MINS: [u8; 5] = [1, 2, 5, 10, 13];
static MAXES: [u8; 5] = [1, 4, 9, 12, 13];
fn diamond_vert(digit: u8, row: usize, next_row: usize) -> u8 {
    static LENGTHS: [u8; 5] = [1, 3, 5, 3, 1];
    static PADDING: [u8; 5] = [2, 1, 0, 1, 2];
    let offset = digit - MINS[row];
    if LENGTHS[next_row] > LENGTHS[row] {
        return MINS[next_row] + offset + 1;
    } else {
        let next = (MINS[next_row] + offset).checked_sub(
            PADDING[next_row] - PADDING[row]);
        if next.is_some() {
            let next = next.unwrap();
            if next >= MINS[next_row] && next <= MAXES[next_row] {
                return next
            }
        }
    }
    digit
}

fn get_diamond_button(line: &str, last: char) -> char {
    let mut digit = last.to_digit(16).unwrap() as u8;
    for c in line.chars() {
        let row = MAXES.iter().position(|&x| digit <= x).unwrap();
        match c {
            'R' if digit + 1 <= MAXES[row] => digit += 1,
            'L' if digit - 1 >= MINS[row] => digit -= 1,
            'U' if row != 0 => digit = diamond_vert(digit, row, row - 1),
            'D' if row != 4 => digit = diamond_vert(digit, row, row + 1),
            _ => {}
        }
    }
    char::from_digit(digit as u32, 16).unwrap()
}

fn get_code<'a>(f: &Fn(&str, char) -> char, input: &str) -> Cow<'a, str> {
    let mut result = String::new();
    let mut last = '5';
    for line in input.lines() {
        let button = f(line, last);
        result.push(button);
        last = button;
    }
    result.to_uppercase().into()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{} {}", get_code(&get_button, input.as_str()),
             get_code(&get_diamond_button, input.as_str()));
}

#[cfg(tests)]
mod test {
    use super::{get_button, get_code, get_diamond_button};

    #[test]
    fn given_tests() {
        assert_eq!("1985", get_code(&get_button, "ULL\nRRDDD\nLURDL\nUUUUD"));
    }

    #[test]
    fn given_diamond_tests() {
        assert_eq!("5DB3", get_code(&get_diamond_button,
                                    "ULL\nRRDDD\nLURDL\nUUUUD"));
    }

    #[test]
    fn diamond_up_to_shorter() {
        assert_eq!("5", get_code(&get_diamond_button, "U"));
        assert_eq!("2", get_code(&get_diamond_button, "RU"));
        assert_eq!("1", get_code(&get_diamond_button, "RURU"));
    }
}
