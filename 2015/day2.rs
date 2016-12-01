use std::cmp;
use std::io::{self, Read};

fn paper_needed(l: u32, w: u32, h: u32) -> u32 {
    let area = 2*l*w + 2*w*h + 2*h*l;
    let smallest_area = cmp::min(l*w, cmp::min(w*h, h*l));
    area + smallest_area
}

fn ribbon_needed(l: u32, w: u32, h: u32) -> u32 {
    let smallest_perimeter = cmp::min(2*l+2*w, cmp::min(2*w+2*h, 2*h+2*l));
    let volume = l * w * h;
    volume + smallest_perimeter
}

fn paper_and_ribbon_needed(input: &str) -> (u32, u32) {
    let mut paper: u32 = 0;
    let mut ribbon: u32 = 0;
    for line in input.lines() {
        let dims: Vec<_> = line.split('x')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        paper += paper_needed(dims[0], dims[1], dims[2]);
        ribbon += ribbon_needed(dims[0], dims[1], dims[2]);
    }
    (paper, ribbon)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (paper, ribbon) = paper_and_ribbon_needed(input.as_str());
    println!("Paper: {}, ribbon: {}", paper, ribbon);
}

#[cfg(tests)]
mod test {
    use super::paper_and_ribbon_needed;

    #[test]
    fn given_tests() {
        assert_eq!((58, 34), paper_and_ribbon_needed("2x3x4"));
        assert_eq!((43, 14), paper_and_ribbon_needed("1x1x10"));
    }
}
