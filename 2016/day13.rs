use std::cmp;
use std::collections::{HashSet, VecDeque};

fn is_wall(x: u32, y: u32, n: u32) -> bool {
    let v: u32 = x*x + 3*x + 2*x*y + y + y*y + n;
    v.count_ones() & 0x1 == 1
}

macro_rules! push_if_not_wall {
    ($next:expr, $seen:expr, $n: expr, $x:expr, $y:expr, $s:expr) => {
        if !$seen.iter().any(|p| p.0 == ($x) && p.1 == ($y)) {
            if !is_wall($x, $y, $n) {
                $next.push_back(($x, $y, $s));
            }
        }
    }
}

fn main() {
    let n = 1350;
    //let n = 10;
    let mut seen = HashSet::new();
    let mut next = VecDeque::new();
    let mut max_x = 1;
    let mut max_y = 1;
    next.push_back((1, 1, 0));
    while let Some(p) = next.pop_front() {
        max_x = cmp::max(max_x, p.0);
        max_y = cmp::max(max_y, p.1);
        seen.insert(p);
        if p.0 == 50 && p.1 == 50 {
        //if p.0 == 31 && p.1 == 39 {
        //if p.0 == 7 && p.1 == 4 {
            println!("1: {}", p.2);
            break;
        }
        push_if_not_wall!(next, seen, n, p.0 + 1, p.1, p.2 + 1);
        push_if_not_wall!(next, seen, n, p.0, p.1 + 1, p.2 + 1);
        if p.0 != 0 {
            push_if_not_wall!(next, seen, n, p.0 - 1, p.1, p.2 + 1);
        }
        if p.1 != 0 {
            push_if_not_wall!(next, seen, n, p.0, p.1 - 1, p.2 + 1);
        }
    }
    println!("2: {}", seen.iter().filter(|p| p.2 <= 50).count());
    for y in 0..(max_y + 1) {
        for x in 0..(max_x + 1) {
            if is_wall(x, y, n) {
                print!("#");
            } else if seen.iter().any(|p| p.0 == x && p.1 == y) {
                print!(".");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}
