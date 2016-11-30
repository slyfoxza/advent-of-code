use std::io::{self, Read};

fn find_floor(input: String) -> i32 {
    input.chars().map(|c| {
        match c {
            '(' => 1,
            ')' => -1,
            _ => 0
        }
    }).fold(0, |a, x| a + x)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", find_floor(input));
}

#[cfg(tests)]
mod tests {
    use super::find_floor;

    #[test]
    fn empty_input() { assert_eq!(0, find_floor(String::new())); }

    #[test]
    fn up_only() { assert_eq!(1, find_floor(String::from("("))); }

    #[test]
    fn down_only() { assert_eq!(-1, find_floor(String::from(")"))); }

    #[test]
    fn invalid_input() { assert_eq!(0, find_floor(String::from("!"))); }

    #[test]
    fn given_tests() {
        assert_eq!(0, find_floor(String::from("(())")));
        assert_eq!(0, find_floor(String::from("()()")));
        assert_eq!(3, find_floor(String::from("(((")));
        assert_eq!(3, find_floor(String::from("(()(()(")));
        assert_eq!(3, find_floor(String::from("))(((((")));
        assert_eq!(-1, find_floor(String::from("())")));
        assert_eq!(-1, find_floor(String::from("))(")));
        assert_eq!(-3, find_floor(String::from(")))")));
        assert_eq!(-3, find_floor(String::from(")())())")));
    }
}
