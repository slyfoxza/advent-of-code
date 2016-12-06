use std::cmp::Ordering;
use std::io::{self, Read};

fn decode(input: &str) -> String {
    let mut output = Vec::with_capacity(input.len());
    let line_length = input.lines().next().unwrap().len();
    for i in 0..line_length {
        let mut histo = [(0, 0); 26];
        for j in 0..histo.len() {
            histo[j].0 = j as u8;
        }
        for b in input.lines().map(|x| x.bytes().nth(i).unwrap()) {
            let j = (b - b'a') as usize;
            histo[j].1 += 1;
        }
        histo.sort_by(|a, b| {
            match b.1.cmp(&a.1) {
                Ordering::Equal => a.0.cmp(&b.0),
                c @ _ => c
            }
        });
        output.push(histo[0].0 + b'a');
    }
    String::from_utf8(output).unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", decode(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::decode;

    #[test]
    fn simple() {
        assert_eq!("abc", decode("aaa\nabc\nbbc"));
    }
}
