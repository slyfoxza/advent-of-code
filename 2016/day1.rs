use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::io::{self, Read};

#[derive(Clone, Eq, Ord)]
struct Point {
    x: i32,
    y: i32
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct IterState {
    dx: i32,
    dy: i32,
    p: Point
}

fn blocks_away(input: &str) -> (i32, Option<i32>) {
    let mut seen = BTreeSet::new();
    let mut state = IterState { dx: 0, dy: 1, p: Point { x: 0, y: 0 } };
    let mut repeat = None;
    for instruction in input.split(',').map(|i| i.trim()) {
        let direction = instruction.chars().next().unwrap();
        if direction == 'L' {
            if state.dx != 0 {
                state.dy = state.dx;
                state.dx = 0;
            } else {
                state.dx = -state.dy;
                state.dy = 0;
            }
        } else {
            if state.dx != 0 {
                state.dy = -state.dx;
                state.dx = 0;
            } else {
                state.dx = state.dy;
                state.dy = 0;
            }
        }
        let distance = i32::from_str_radix(&instruction[1..], 10).unwrap();
        for i in 0..distance {
            let p = Point {
                x: state.p.x + (i + 1) * state.dx,
                y: state.p.y + (i + 1) * state.dy
            };
            if repeat.is_none() && !seen.insert(p.clone()) {
                repeat = Some(p);
            }
        }
        state.p.x += distance * state.dx;
        state.p.y += distance * state.dy;
    }
    let repeat = repeat.map(|p| p.x.abs() + p.y.abs());
    (state.p.x.abs() + state.p.y.abs(), repeat)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (distance, first_repeat) = blocks_away(input.as_str());
    println!("{}", distance);
    match first_repeat {
        Some(fr) => println!("First repeat: {}", fr),
        None => println!("No repeat")
    }
}

#[cfg(tests)]
mod tests {
    use super::blocks_away;

    #[test]
    fn given_tests() {
        assert_eq!((5, None), blocks_away("R2, L3"));
        assert_eq!((2, None), blocks_away("R2, R2, R2"));
        assert_eq!((12, None), blocks_away("R5, L5, R5, R3"));
    }

    #[test]
    fn first_repeat_given_tests() {
        assert_eq!((8, Some(4)), blocks_away("R8, R4, R4, R8"));
    }
}
