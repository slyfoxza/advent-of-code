use std::cmp;
use std::collections::LinkedList;
use std::f32::consts::FRAC_PI_2;
use std::io::{self, Read};

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32
}

struct Span {
    begin: i32,
    end: i32,
    horizontal: bool,
    z: i32
}

impl Span {
    fn new(a: i32, b: i32, horizontal: bool, z: i32) -> Span {
        Span {
            begin: cmp::min(a, b),
            end: cmp::max(a, b),
            horizontal: horizontal,
            z: z
        }
    }
}

struct IterState {
    a: f32,
    p: Point
}

fn blocks_away(input: &str) -> (i32, Option<i32>) {
    let mut spans: LinkedList<Span> = LinkedList::new();
    let mut state = IterState { a: FRAC_PI_2, p: Point { x: 0, y: 0 } };
    let mut repeat = None;
    for instruction in input.split(',').map(|i| i.trim()) {
        state.a += match instruction.chars().next().unwrap() {
            'L' => FRAC_PI_2,
            'R' => -FRAC_PI_2,
            _ => continue
        };
        let dx = state.a.sin() as i32;
        let dy = state.a.cos() as i32;
        let distance = i32::from_str_radix(&instruction[1..], 10).unwrap();
        let x1 = state.p.x + distance * dx;
        let y1 = state.p.y + distance * dy;
        let horizontal = dy == 0;
        if repeat.is_none() {
            let span = match horizontal {
                true => Span::new(state.p.x, x1, true, y1),
                false => Span::new(state.p.y, y1, false, x1)
            };
            {
                let mut hit_spans = spans.iter_mut()
                    .filter(|s| {
                        s.horizontal != span.horizontal
                            && match span.horizontal {
                                true => s.begin < y1 && s.end > y1
                                    && s.z >= cmp::min(x1, state.p.x)
                                    && s.z <= cmp::max(x1, state.p.x),
                                false => s.begin < x1 && s.end > x1
                                    && s.z >= cmp::min(y1, state.p.y)
                                    && s.z <= cmp::max(y1, state.p.y)
                        }
                    })
                    .collect::<Vec<_>>();
                hit_spans.sort_by(|a, b| {
                    if dx == 1 || dy == 1 {
                        a.z.cmp(&b.z)
                    } else {
                        b.z.cmp(&a.z)
                    }
                });
                match hit_spans.first() {
                        Some(hs) => {
                            repeat = Some(match span.horizontal {
                                true => Point { x: hs.z, y: y1 },
                                false => Point { x: x1, y: hs.z }
                            });
                        },
                        None => {}
                    };
            }
            spans.push_back(span);
        }
        state.p.x = x1;
        state.p.y = y1;
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
