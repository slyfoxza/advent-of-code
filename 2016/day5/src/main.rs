extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;
use std::io::{self, Read};

fn h4xx0r(door_id: &str) -> String {
    let mut password = String::new();
    let mut i = 0;
    let mut md5 = Md5::new();
    loop {
        let mut candidate = String::from(door_id);
        candidate.push_str(i.to_string().as_str());
        md5.input_str(candidate.as_str());
        let hash = md5.result_str();
        md5.reset();
        if hash.starts_with("00000") {
            password.push(hash.chars().nth(5).unwrap());
            if password.len() == 8 {
                return password;
            }
        }
        i += 1;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", h4xx0r(input.trim()));
}

#[cfg(test)]
mod test {
    use super::h4xx0r;
    
    #[test]
    fn sample() {
        assert_eq!("18f47a30", h4xx0r("abc"));
    }
}
