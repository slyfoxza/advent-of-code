use std::io::{self, Read};

fn input_map(c: char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0
    }
}

fn find_basement(input: &String) -> Option<i32> {
    let iter = input.chars().map(input_map).enumerate();
    let mut accum = 0;
    for item in iter {
        let (i, value) = item;
        accum = accum + value;
        if accum == -1 {
            return Some((i + 1) as i32);
        }
    }
    None
}

fn find_floor(input: &String) -> i32 {
    input.chars().map(input_map).sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", find_floor(&input));
    match find_basement(&input) {
        Some(i) => println!("Basement entered at character position {}", i),
        None => println!("Basement never entered")
    }
}

#[cfg(test)]
mod tests {
    use super::{find_basement, find_floor};

    #[test]
    fn empty_input() { assert_eq!(0, find_floor(&String::new())); }

    #[test]
    fn up_only() { assert_eq!(1, find_floor(&String::from("("))); }

    #[test]
    fn down_only() { assert_eq!(-1, find_floor(&String::from(")"))); }

    #[test]
    fn invalid_input() { assert_eq!(0, find_floor(&String::from("!"))); }

    #[test]
    fn given_tests() {
        assert_eq!(0, find_floor(&String::from("(())")));
        assert_eq!(0, find_floor(&String::from("()()")));
        assert_eq!(3, find_floor(&String::from("(((")));
        assert_eq!(3, find_floor(&String::from("(()(()(")));
        assert_eq!(3, find_floor(&String::from("))(((((")));
        assert_eq!(-1, find_floor(&String::from("())")));
        assert_eq!(-1, find_floor(&String::from("))(")));
        assert_eq!(-3, find_floor(&String::from(")))")));
        assert_eq!(-3, find_floor(&String::from(")())())")));
    }

    #[test]
    fn basement_no_match() {
        assert_eq!(None, find_basement(&String::from("(")));
    }

    #[test]
    fn basement_given_tests() {
        assert_eq!(Some(1), find_basement(&String::from(")")));
        assert_eq!(Some(5), find_basement(&String::from("()())")));
    }
}
