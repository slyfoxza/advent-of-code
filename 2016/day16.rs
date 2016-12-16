use std::io::{self, Read};

fn calc_checksum(disk_length: usize, initial_state: &str) -> String {
    let mut data = String::from(initial_state);
    while data.len() < disk_length {
        let mut next_data = String::with_capacity(data.len());
        for c in data.chars().rev() {
            next_data.push(if c == '0' { '1' } else { '0' });
        }
        data.push('0');
        data.push_str(next_data.as_str());
    }
    data.truncate(disk_length);
    let mut checksum = String::new();
    while checksum.is_empty() || checksum.len() & 1 == 0 {
        checksum.clear();
        for pair in data.as_str().as_bytes().chunks(2) {
            checksum.push(if pair[0] == pair[1] { '1' } else { '0' });
        }
        data = checksum.clone();
    }
    checksum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{} {}", calc_checksum(272, input.trim()),
             calc_checksum(35651584, input.trim()));
}

#[cfg(test)]
mod test {
    use super::calc_checksum;

    #[test]
    fn test_given() {
        assert_eq!("01100", calc_checksum(20, "10000"));
    }
}
