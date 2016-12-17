extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;
use std::io::{self, Read};
use std::collections::VecDeque;

struct State {
    path: String,
    x: u8, y: u8
}

fn next_path(current_path: &String, next_char: char) -> String {
    let mut next_path = current_path.clone();
    next_path.push(next_char);
    return next_path
}

fn find_path(passcode: &str) -> Result<String, String> {
    let mut queue = VecDeque::new();
    queue.push_back(State { path: String::new(), x: 0, y: 0 });
    let mut md5 = Md5::new();
    while let Some(current) = queue.pop_front() {
        if current.x == 3 && current.y == 3 {
            return Ok(current.path)
        }
        let mut hash_input = String::from(passcode);
        hash_input.push_str(current.path.as_str());
        md5.input_str(hash_input.as_str());
        let mut hash = [0; 16];
        md5.result(&mut hash);
        md5.reset();
        if current.y > 0 && (hash[0] & 0xF0) >= 0xB0 {
            queue.push_back(State {
                path: next_path(&current.path, 'U'),
                x: current.x, y: current.y - 1
            });
        }
        if current.y < 3 && (hash[0] & 0x0F) >= 0xB {
            queue.push_back(State {
                path: next_path(&current.path, 'D'),
                x: current.x, y: current.y + 1
            });
        }
        if current.x > 0 && (hash[1] & 0xF0) >= 0xB0 {
            queue.push_back(State {
                path: next_path(&current.path, 'L'),
                x: current.x - 1, y: current.y
            });
        }
        if current.x < 3 && (hash[1] & 0x0F) >= 0xB {
            queue.push_back(State {
                path: next_path(&current.path, 'R'),
                x: current.x + 1, y: current.y
            });
        }
        if queue.is_empty() {
            return Err(current.path)
        }
    }
    Err(String::from(""))
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    match find_path(input.trim()) {
        Ok(p) => println!("Shortest path: {}", p),
        Err(p) => println!("All doors locked after: {}", p)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_given_err() {
        assert_eq!(Err(String::from("DUR")), super::find_path("hijkl"));
    }

    #[test]
    fn test_given_ok1() {
        assert_eq!(Ok(String::from("DDRRRD")), super::find_path("ihgpwlah"));
    }

    #[test]
    fn test_given_ok2() {
        assert_eq!(Ok(String::from("DDUDRLRRUDRD")),
                   super::find_path("kglvqrro"));
    }

    #[test]
    fn test_given_ok3() {
        assert_eq!(Ok(String::from("DRURDRUDDLLDLUURRDULRLDUUDDDRR")),
                   super::find_path("ulqzkmiv"));
    }
}
