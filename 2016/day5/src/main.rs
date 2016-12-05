extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;
use std::io::{self, Read};
use std::str;

fn h4xx0r(door_id: &str) -> (String, String) {
    static LEN: usize = 8;
    let mut password = String::new();
    let mut password2 = Vec::with_capacity(LEN);
    password2.resize(LEN, b'_');
    let mut pw2filled = 0;
    let mut i = 0;
    let mut md5 = Md5::new();
    while password.len() < LEN || pw2filled < LEN {
        let mut candidate = String::from(door_id);
        candidate.push_str(i.to_string().as_str());
        md5.input_str(candidate.as_str());
        let hash = md5.result_str();
        md5.reset();
        if hash.starts_with("00000") {
            if password.len() < LEN {
                password.push(hash.chars().nth(5).unwrap());
            }
            if pw2filled < LEN {
                let mut bytes = hash.bytes();
                let pos = (bytes.nth(5).unwrap() - b'0') as usize;
                if pos < LEN && password2[pos] == b'_' {
                    password2[pos] = bytes.next().unwrap();
                    pw2filled += 1;
                }
            }
        }
        i += 1;
    }
    (password, String::from_utf8(password2).unwrap())
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (password, password2) = h4xx0r(input.trim());
    println!("{} {}", password, password2);
}

#[cfg(test)]
mod test {
    use super::h4xx0r;
    
    #[test]
    fn sample() {
        let (password, password2) = h4xx0r("abc");
        assert_eq!("18f47a30", password);
        assert_eq!("05ace8e3", password2);
    }
}
