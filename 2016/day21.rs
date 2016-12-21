use std::cmp;
use std::io::{self, Read};

fn swap_position(input: &str, x: usize, y: usize) -> String {
    let mut result = String::with_capacity(input.len());
    for c in input.chars().take(x) { result.push(c); }
    if let Some(c) = input.chars().nth(y) { result.push(c); }
    for c in input.chars().skip(x + 1).take(y - x - 1) { result.push(c); }
    if let Some(c) = input.chars().nth(x) { result.push(c); }
    for c in input.chars().skip(y + 1) { result.push(c); }
    return result
}

fn rotate_by(input: &str, n: usize, is_left: bool) -> String {
    let len = input.len();
    let mut result = String::with_capacity(len);
    if n == 0 {
        result.push_str(input);
    } else {
        let n = if is_left { n } else { len - n };
        for c in input.chars().skip(n) { result.push(c); }
        for c in input.chars().take(n) { result.push(c); }
    }
    return result
}

fn reverse(input: &str, x: usize, y: usize) -> String {
    let len = input.len();
    let mut result = String::with_capacity(len);
    for c in input.chars().take(x) { result.push(c); }
    for c in input.chars().rev().skip(len - y - 1).take(y - x + 1) {
        result.push(c);
    }
    for c in input.chars().skip(y + 1) { result.push(c); }
    return result
}

fn move_to(input: &str, src: usize, dst: usize) -> String {
    if let Some(c) = input.chars().nth(src) {
        let mut result = String::with_capacity(input.len());
        for c in input.chars().take(cmp::min(src, dst)) { result.push(c); }
        if src > dst {
            result.push(c);
            for c in input.chars().skip(dst).take(src - dst) {
                result.push(c);
            }
            for c in input.chars().skip(src + 1) { result.push(c); }
        } else {
            for c in input.chars().skip(src + 1).take(dst - src) {
                result.push(c);
            }
            result.push(c);
            for c in input.chars().skip(dst + 1) { result.push(c); }
        }
        return result
    } else {
        String::from(input)
    }
}

fn rotate_by_pos(input: &str, letter: &str) -> String {
    let c = letter.chars().next().unwrap();
    let ci = input.char_indices().find(|ci| ci.1 == c).unwrap();
    let mut n = 1 + ci.0;
    if ci.0 >= 4 {
        n += 1;
    }
    rotate_by(input, n % input.len(), false)
}

fn swap_letter(input: &str, cx: &str, cy: &str) -> String {
    let cx = cx.chars().next().unwrap();
    let cy = cy.chars().next().unwrap();
    let cxi = input.char_indices().find(|ci| ci.1 == cx).unwrap();
    let cyi = input.char_indices().find(|ci| ci.1 == cy).unwrap();
    swap_position(input, cmp::min(cxi.0, cyi.0), cmp::max(cxi.0, cyi.0))
}

fn execute(instr: &str, input: &str) -> String {
    let parts = instr.split_whitespace().collect::<Vec<_>>();
    if instr.starts_with("swap position ") {
        let x = parts[2].parse().unwrap();
        let y = parts[5].parse().unwrap();
        swap_position(input, cmp::min(x, y), cmp::max(x, y))
    } else if instr.starts_with("rotate based on position of letter ") {
        rotate_by_pos(input, parts[6])
    } else if instr.starts_with("rotate ") {
        let is_left = parts[1] == "left";
        let n = parts[2].parse::<usize>().unwrap() % input.len();
        rotate_by(input, n, is_left)
    } else if instr.starts_with("reverse positions ") {
        let x = parts[2].parse().unwrap();
        let y = parts[4].parse().unwrap();
        reverse(input, cmp::min(x, y), cmp::max(x, y))
    } else if instr.starts_with("move position ") {
        let x = parts[2].parse().unwrap();
        let y = parts[5].parse().unwrap();
        move_to(input, x, y)
    } else if instr.starts_with("swap letter ") {
        swap_letter(input, parts[2], parts[5])
    } else {
        panic!("Unrecognized instruction: {}", instr);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut result = String::from("abcdefgh");
    println!("{}", result);
    for line in input.lines() {
        result = execute(line.trim(), &result);
        println!("{} <- {}", result, line);
    }
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::execute;

    #[test]
    fn test_swap_position() {
        assert_eq!("ebcda",
                   execute("swap position 4 with position 0", "abcde"));
    }

    #[test]
    fn test_rotate_right() {
        assert_eq!("dabc", execute("rotate right 1 steps", "abcd"));
    }

    #[test]
    fn test_rotate_left() {
        assert_eq!("bcda", execute("rotate left 1 steps", "abcd"));
    }

    #[test]
    fn test_reverse() {
        assert_eq!("eabcd", execute("reverse positions 1 through 4", "edcba"));
    }

    #[test]
    fn test_move_later() {
        assert_eq!("bdeac", execute("move position 1 to position 4", "bcdea"));
        assert_eq!("bcdaefgh",
                   execute("move position 0 to position 3", "abcdefgh"));
    }

    #[test]
    fn test_move_earlier() {
        assert_eq!("badec", execute("move position 3 to position 1", "bdeac"));
    }

    #[test]
    fn test_rotate_by_pos() {
        assert_eq!("ecabd",
                   execute("rotate based on position of letter b", "abdec"));
    }

    #[test]
    fn test_swap_letter() {
        assert_eq!("edcba", execute("swap letter d with letter b", "ebcda"));
    }
}
