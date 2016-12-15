extern crate crypto;
extern crate regex;

use crypto::digest::Digest;
use crypto::md5::Md5;
use regex::Regex;
use std::io::{self, Read};

fn find_key_index(n_key: u32, salt: &str) -> u32 {
    let re3 = Regex::new(
        "(000|111|222|333|444|555|666|777|888|999|aaa|bbb|ccc|ddd|eee|fff)")
        .unwrap();
    let mut i = 0;
    let mut k = 0;
    let mut md5 = Md5::new();
    let mut value = 0;
    while k < n_key {
        let mut hash_input = String::from(salt);
        hash_input.push_str(i.to_string().as_str());
        md5.input_str(hash_input.as_str());
        let hash = md5.result_str();
        md5.reset();
        if let Some(cap) = re3.captures(hash.as_str()) {
            let mut re5 = String::new();
            re5.push_str(&cap.at(1).unwrap()[0..1]);
            re5.push_str("{5}");
            let re5 = Regex::new(re5.as_str()).unwrap();
            let mut j = 1;
            while j < 1000 {
                let mut hash_input = String::from(salt);
                hash_input.push_str((i + j).to_string().as_str());
                md5.input_str(hash_input.as_str());
                let hash = md5.result_str();
                md5.reset();
                if re5.is_match(hash.as_str()) {
                    value = i;
                    k += 1;
                    break;
                }
                j += 1;
            }
        }
        i += 1;
    }
    value
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", find_key_index(64, input.trim()));
}
