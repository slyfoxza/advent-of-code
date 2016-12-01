use std::io::{self, Read};

struct Point {
    x: i32,
    y: i32
}

struct IterState {
    dx: i32,
    dy: i32,
    p: Point
}

fn blocks_away(input: &str) -> i32 {
    let mut state = IterState { dx: 0, dy: 1, p: Point { x: 0, y: 0 } };
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
        state.p.x += distance * state.dx;
        state.p.y += distance * state.dy;
    }
    state.p.x.abs() + state.p.y.abs()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", blocks_away(input.as_str()));
}

#[cfg(tests)]
mod tests {
    use super::blocks_away;

    #[test]
    fn given_tests() {
        assert_eq!(5, blocks_away("R2, L3"));
        assert_eq!(2, blocks_away("R2, R2, R2"));
        assert_eq!(12, blocks_away("R5, L5, R5, R3"));
    }
}
