use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut offsets = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let offsets_copy = offsets.clone();

    let mut i: i32 = 0;
    let mut n1 = 0;
    while i >= 0 && i < offsets.len() as i32 {
        offsets[i as usize] += 1;
        i += offsets[i as usize] - 1;
        n1 += 1;
    }

    let mut offsets = offsets_copy;
    i = 0;
    let mut n2 = 0;
    while i >= 0 && i < offsets.len() as i32 {
        let offset = offsets[i as usize];
        offsets[i as usize] += if offset >= 3 { -1 } else { 1 };
        i += offset;
        n2 += 1;
    }
    println!("{} {}", n1, n2);
}
