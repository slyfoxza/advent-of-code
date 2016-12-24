use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};

macro_rules! push_neighbours {
    ($p:expr, $se:expr, $g:expr, $xy:expr, $pois:expr, $s:expr) => {
        if $xy.0 > 0 && $g[$xy.1 * $s.0 + $xy.0 - 1] != 0xFF {
            if !$se.iter().any(|x| x.0 == $xy.0 - 1 && x.1 == $xy.1 && x.2 == $pois) {
                $p.push_back(($xy.0 - 1, $xy.1, $xy.2 + 1, $pois));
            }
        }
        if $xy.1 > 0 && $g[($xy.1 - 1) * $s.0 + $xy.0] != 0xFF {
            if !$se.iter().any(|x| x.0 == $xy.0     && x.1 == $xy.1 - 1 && x.2 == $pois) {
                $p.push_back(($xy.0,     $xy.1 - 1, $xy.2 + 1, $pois));
            }
        }
        if $xy.0 < $s.0 - 1 && $g[$xy.1 * $s.0 + $xy.0 + 1] != 0xFF {
            if !$se.iter().any(|x| x.0 == $xy.0 + 1 && x.1 == $xy.1 && x.2 == $pois) {
                $p.push_back(($xy.0 + 1, $xy.1, $xy.2 + 1, $pois));
            }
        }
        if $xy.1 < $s.1 - 1 && $g[($xy.1 + 1) * $s.0 + $xy.0] != 0xFF {
            if !$se.iter().any(|x| x.0 == $xy.0     && x.1 == $xy.1 + 1 && x.2 == $pois) {
                $p.push_back(($xy.0,     $xy.1 + 1, $xy.2 + 1, $pois));
            }
        }
    }
}

fn path_to_poi(grid: &Vec<u8>, size: (usize, usize), pois: u8,
               start: (usize, usize)) -> (u8, usize, (usize, usize)) {
    let mut seen = HashSet::new();
    let mut pending = VecDeque::new();
    pending.push_back((start.0, start.1, 0, pois));
    while let Some(p) = pending.pop_front() {
        if !seen.insert((p.0, p.1, p.3)) {
            continue;
        }
        let mut pois = p.3;
        let g = grid[p.1 * size.0 + p.0];
        if (g > 0) && (g < 8) && (pois & (1 << g) == 0) {
            pois |= 1 << g;
        }
        if pois.count_ones() == 7 {
            return (pois, p.2, (p.0, p.1));
        }
        push_neighbours!(pending, seen, grid, p, pois, size);
        //println!("POIs: {:08b}, Pending: {:?}", pois, pending);
        println!("POIs: {:08b} / {}, Pending: {}", pois, pois.count_ones(), pending.len());
    }
    panic!("Ruh roh!");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut grid = Vec::new();
    let mut size = (0, 0);
    for b in input.bytes() {
        if b == b'\n' {
            size.1 += 1;
            continue;
        }
        if size.1 == 0 { size.0 += 1; }
        if b == b'#' {
            grid.push(0xFF);
        } else if b == b'.' {
            grid.push(0x7F);
        } else {
            grid.push(b - b'0');
        }
    }
    let start = grid.iter().position(|&c| c == 0).unwrap();
    let mut start = (start % size.0, start / size.0);
    println!("{} x {}, start @ {:?}", size.0, size.1, start);
    let mut pois: u8 = 0;
    let mut total_steps = 0;
    while pois.count_ones() != 7 {
        println!("{} POI(s) found so far", pois.count_ones());
        let (new_pois, steps, pos) = path_to_poi(&grid, size, pois, start);
        println!("Found POI after {} steps at {:?}", steps, pos);
        pois = new_pois;
        start = pos;
        total_steps += steps;
    }
    println!("{}", total_steps);
}
