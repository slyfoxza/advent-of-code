extern crate crypto;
extern crate regex;

use crypto::digest::Digest;
use crypto::md5::Md5;
use regex::Regex;
use std::io::{self, Read};

macro_rules! try_cache {
    ($cache:expr, $cache_range:expr, $i:expr) => {
        if ($i < $cache_range.0) || ($i > $cache_range.1) {
            Err(())
        } else {
            Ok($cache.iter().find(|x| x.0 == $i).map(|x| x.1.clone()))
        }
    }
}

fn generate_hash(salt: &str, i: u32, stretching: u32) -> String {
    let mut hash = String::from(salt);
    hash.push_str(i.to_string().as_str());
    let mut md5 = Md5::new();
    md5.input_str(hash.as_str());
    hash = md5.result_str();
    for _ in 0..stretching {
        md5.reset();
        md5.input_str(hash.as_str());
        hash = md5.result_str();
    }
    return hash
}

fn find_key_index(n_key: u32, salt: &str, stretching: u32) -> u32 {
    let mut cache = Vec::<(u32, String)>::new();
    let mut cache_range = (1, 0);
    let re3 = Regex::new(
        "(000|111|222|333|444|555|666|777|888|999|aaa|bbb|ccc|ddd|eee|fff)")
        .unwrap();
    let mut i_key = 0;
    let mut i = 0;
    let mut value = 0;
    while i_key < n_key {
        if i > cache_range.0 {
            let mut drop_first = false;
            if let Some(x) = cache.first() {
                if x.0 < i {
                    drop_first = true;
                }
            }
            if drop_first {
                cache.remove(0);
            }
        }
        let hash = if let Ok(hash_option) = try_cache!(cache, cache_range, i) {
            match hash_option {
                Some(hash) => {
                    hash
                },
                None => {
                    i += 1;
                    continue
                }
            }
        } else {
            if cache_range.0 > i { cache_range.0 = i; }
            if cache_range.1 < i { cache_range.1 = i; }
            let hash = generate_hash(salt, i, stretching);
            if let Some(cap) = re3.captures(hash.as_str()) {
                cache.push((i, hash.clone()));
                hash.clone()
            } else {
                i += 1;
                continue
            }
        };

        for j in 1..1000 {
            let hash2 = if let Ok(hash_option) = try_cache!(cache, cache_range,
                                                            i + j) {
                match hash_option {
                    Some(hash) => {
                        hash
                    },
                    None => continue
                }
            } else {
                if cache_range.0 > i + j { cache_range.0 = i + j; }
                if cache_range.1 < i + j { cache_range.1 = i + j; }
                let hash = generate_hash(salt, i + j, stretching);
                if let Some(cap) = re3.captures(hash.as_str()) {
                    cache.push((i + j, hash.clone()));
                    hash.clone()
                } else {
                    continue
                }
            };

            let hash_rep = re3.captures(hash.as_str()).unwrap();
            let mut re5 = String::new();
            re5.push_str(&hash_rep.at(1).unwrap()[0..1]);
            re5.push_str("{5}");
            let re5 = Regex::new(re5.as_str()).unwrap();
            if re5.is_match(hash2.as_str()) {
                println!("Value @ {} = {}", i, value);
                value = i;
                i_key += 1;
                break;
            }
        }
        i += 1;
    }
    value
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("64th key index w/    0 stretching: {}",
             find_key_index(64, input.trim(), 0));
    println!("64th key index w/ 2016 stretching: {}",
             find_key_index(64, input.trim(), 2016));
}

#[cfg(test)]
mod test {
    use super::find_key_index;

    #[test]
    fn test_unstretched() {
        assert_eq!(39, find_key_index(1, "abc", 0));
    }

    #[test]
    fn test_stretched() {
        assert_eq!(10, find_key_index(1, "abc", 2016));
    }
}
