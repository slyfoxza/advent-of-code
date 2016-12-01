use std::cmp;
use std::io::{self, Read};

fn paper_needed(input: &str) -> u32 {
    let mut paper_needed: u32 = 0;
    for line in input.lines() {
        let dims: Vec<_> = line.split('x')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let l = dims[0];
        let w = dims[1];
        let h = dims[2];
        let area = 2*l*w + 2*w*h + 2*h*l;
        let smallest_area = cmp::min(l*w, cmp::min(w*h, h*l));
        paper_needed += area + smallest_area
    }
    paper_needed
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", paper_needed(input.as_str()));
}

#[cfg(tests)]
mod test {
    use super::paper_needed;

    #[test]
    fn given_tests() {
        assert_eq!(58, paper_needed("2x3x4"));
        assert_eq!(43, paper_needed("1x1x10"));
    }
}
