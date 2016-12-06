use std::cmp::Ordering;
use std::io::{self, Read};

fn decode(input: &str) -> (String, String) {
    let mut output = Vec::with_capacity(input.len());
    let mut output2 = Vec::with_capacity(input.len());
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
        for i in 0..histo.len() {
            if histo[i].1 == 0 {
                output2.push(histo[i - 1].0 + b'a');
                break;
            }
        }
        if output2.len() != output.len() {
            output2.push(histo[histo.len() - 1].0 + b'a');
        }
    }
    (String::from_utf8(output).unwrap(), String::from_utf8(output2).unwrap())
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (output1, output2) = decode(input.as_str());
    println!("{} {}", output1, output2);
}

#[cfg(test)]
mod tests {
    use super::decode;

    #[test]
    fn simple() {
        let (output1, output2) = decode("aaa\nabc\nbbc");
        assert_eq!("abc", output1);
        assert_eq!("baa", output2);
    }
}
