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

fn get_code(input: &str) -> Cow<str> {
    let mut result = String::new();
    let mut last = '5';
    for line in input.lines() {
        let button = get_button(line, last);
        result.push(button);
        last = button;
    }
    result.into()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", get_code(input.as_str()));
}

#[cfg(tests)]
mod test {
    use super::get_code;

    #[test]
    fn given_tests() {
        assert_eq!("1985", get_code("ULL\nRRDDD\nLURDL\nUUUUD"));
    }
}
