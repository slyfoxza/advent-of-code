#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::cmp::Ordering;
use std::io::{self, Read};

struct Room<'a> {
    name: &'a str,
    sector_id: u16,
    checksum: &'a str
}

impl<'a> Room<'a> {
    fn from_string(s: &str) -> Room {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^([-a-z]+)-(\d+)\[([a-z]+)\]$").unwrap();
        }
        let cap = RE.captures(s).unwrap();
        Room {
            name: cap.at(1).unwrap(),
            sector_id: u16::from_str_radix(cap.at(2).unwrap(), 10).unwrap(),
            checksum: cap.at(3).unwrap()
        }
    }

    fn is_valid(&self) -> bool {
        let mut histo = [(0, 0); 26];
        for i in 0..histo.len() {
            histo[i].0 = i;
        }
        for b in self.name.bytes() {
            if b == b'-' {
                continue;
            }
            let bi = (b - b'a') as usize;
            histo[bi].1 += 1;
        }
        histo.sort_by(|a, b| {
            match b.1.cmp(&a.1) {
                Ordering::Equal => a.0.cmp(&b.0),
                c @ _ => c
            }
        });
        self.checksum
            .bytes()
            .enumerate()
            .all(|b| histo[b.0].0 == (b.1 - b'a') as usize)
    }

    fn decrypt_name(&self) -> String {
        let mut result = Vec::with_capacity(self.name.len());
        let shift = (self.sector_id % 26) as u8;
        for b in self.name.bytes() {
            if b == b'-' {
                result.push(b' ');
                continue;
            }
            result.push((b - b'a' + shift) % 26 + b'a')
        }
        String::from_utf8(result).unwrap()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let valid_rooms: Vec<_> = input.lines()
        .map(|i| Room::from_string(i))
        .filter(|ref r| r.is_valid())
        .collect();
    let sector_id_sum: u32 = valid_rooms.iter()
        .map(|r| r.sector_id as u32)
        .sum();
    println!("Sector ID sum: {}", sector_id_sum);
    for room in valid_rooms.iter() {
        let name = room.decrypt_name();
        if name.contains("north") {
            println!("North Pole objects room: {} ({})", name, room.sector_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Room;

    #[test]
    fn room_construction() {
        let room = Room::from_string("ab-cd-123[abcd]");
        assert_eq!("ab-cd", room.name);
        assert_eq!(123, room.sector_id);
        assert_eq!("abcd", room.checksum);
    }

    #[test]
    fn validity() {
        assert!(Room::from_string("a-1[a]").is_valid());
    }

    #[test]
    fn invalidity() {
        assert!(!Room::from_string("aab-2[ba]").is_valid());
    }

    #[test]
    fn decryption() {
        assert_eq!("very encrypted name", Room::from_string(
            "qzmt-zixmtkozy-ivhz-343[a]").decrypt_name());
    }
}
