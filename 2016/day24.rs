use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Read};

fn load_grid(input: &str) -> (Vec<u8>, (usize, usize)) {
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
    (grid, size)
}

fn poi_pos(grid: &Vec<u8>, width: usize, poi: u8) -> (usize, usize) {
    let p = grid.iter().position(|&c| c == poi).unwrap();
    (p % width, p / width)
}

fn pos_to_i(pos: (usize, usize), width: usize) -> usize {
    pos.1 * width + pos.0
}

macro_rules! push_neighbours {
    ($seen:expr, $pending:expr, $grid:expr, $p:expr, $size:expr) => {
        let s = $p.2 + 1;
        if $p.0 > 0 {
            let p = ($p.0 - 1, $p.1);
            if $grid[pos_to_i(p, $size.0)] != 0xFF && !$seen.iter().any(|&e| e == p) {
                $pending.push_back((p.0, p.1, s));
            }
        }
        if $p.0 < $size.0 - 1 {
            let p = ($p.0 + 1, $p.1);
            if $grid[pos_to_i(p, $size.0)] != 0xFF && !$seen.iter().any(|&e| e == p) {
                $pending.push_back((p.0, p.1, s));
            }
        }
        if $p.1 > 0 {
            let p = ($p.0, $p.1 - 1);
            if $grid[pos_to_i(p, $size.0)] != 0xFF && !$seen.iter().any(|&e| e == p) {
                $pending.push_back((p.0, p.1, s));
            }
        }
        if $p.1 < $size.1 - 1 {
            let p = ($p.0, $p.1 + 1);
            if $grid[pos_to_i(p, $size.0)] != 0xFF && !$seen.iter().any(|&e| e == p) {
                $pending.push_back((p.0, p.1, s));
            }
        }
    }
}

fn path_length(grid: &Vec<u8>, size: (usize, usize), start: (usize, usize), goal: (usize, usize)) -> usize {
    let mut seen = HashSet::new();
    let mut pending = VecDeque::new();
    pending.push_back((start.0, start.1, 0));
    while let Some(p) = pending.pop_front() {
        if !seen.insert((p.0, p.1)) {
            continue;
        }
        if (p.0, p.1) == goal {
            return p.2;
        }
        push_neighbours!(seen, pending, grid, p, size);
    }
    panic!("Ran out of pending points");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (grid, size) = load_grid(&input);
    let start = grid.iter().position(|&c| c == 0).unwrap();
    let start = (start % size.0, start / size.0);
    println!("{}x{} grid, start @ {:?}", size.0, size.1, start);
    let mut dists = HashMap::<(usize, usize), usize>::new();
    for i in 0..8 {
        print!("{} | ", i);
        let posi = poi_pos(&grid, size.0, i);
        for _ in 0..i {
            print!("    ");
        }
        print!("  0 ");
        for j in i+1..8 {
            let posj = poi_pos(&grid, size.0, j);
            let len = path_length(&grid, size, posi, posj);
            dists.insert((i as usize, j as usize), len);
            dists.insert((j as usize, i as usize), len);
            print!("{:3} ", len);
        }
        print!("\n");
    }
    let mut path = [0; 7];
    let mut min_dist = *dists.get(&(0, 1)).unwrap();
    for i in 0..path.len() {
        path[i] = i + 1;
        if i > 1 {
            min_dist += *dists.get(&((i - 1), i)).unwrap();
        }
    }
    let mut min_dist_return = min_dist + *dists.get(&(7, 0)).unwrap();
    println!("Distance for path {:?}: {} / {}", path, min_dist, min_dist_return);
    let mut permi = [0; 7];
    let mut i = 0;
    while i < path.len() {
        if permi[i] < i {
            if i & 0x1 == 0 {
                let tmp = path[0];
                path[0] = path[i];
                path[i] = tmp;
            } else {
                let tmp = path[permi[i]];
                path[permi[i]] = path[i];
                path[i] = tmp;
            }
            let mut dist = *dists.get(&(0, path[0])).unwrap();
            for j in 1..path.len() {
                dist += *dists.get(&(path[j - 1], path[j])).unwrap();
            }
            if dist < min_dist {
                min_dist = dist;
                println!("{:?} is shorter: {}", path, min_dist);
            }
            let dist_return = dist + *dists.get(&(path[6], 0)).unwrap();
            if dist_return < min_dist_return {
                min_dist_return = dist_return;
                println!("{:?} has shorter return distance: {}", path, min_dist_return);
            }
            permi[i] += 1;
            i = 0;
        } else {
            permi[i] = 0;
            i += 1;
        }
    }
}
