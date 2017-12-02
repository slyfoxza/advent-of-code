use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut accum = (0, 0);
    let input = input.trim().chars().map(|d| d.to_digit(10).unwrap()).collect::<Vec<_>>();
    let half = input.len() / 2;
    for (i, c) in input.iter().enumerate() {
        if input[(i + 1) % input.len()] == *c {
            accum.0 += *c;
        }
        if input[(i + half) % input.len()] == *c {
            accum.1 += *c;
        }
    }
    println!("{} {}", accum.0, accum.1);
}
