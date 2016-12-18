use std::io::{self, Read};

fn next_line(prev: &str) -> String {
    let mut next = String::with_capacity(prev.len());
    let prevb = prev.as_bytes();
    for i in 0..prevb.len() {
        let left = if i > 0 { prevb.get(i - 1) } else { None };
        let left = left.map(|x| *x).unwrap_or(b'.');
        let center = prevb[i];
        let right = prevb.get(i + 1).map(|x| *x).unwrap_or(b'.');
        if left == b'^' && center == b'^' && right == b'.' {
            next.push('^');
        } else if left == b'.' && center == b'^' && right == b'^' {
            next.push('^');
        } else if left == b'^' && center == b'.' && right == b'.' {
            next.push('^');
        } else if left == b'.' && center == b'.' && right == b'^' {
            next.push('^');
        } else {
            next.push('.');
        }
    }
    return next
}

fn safe_tile_count(start: &str, rows: u32) -> usize {
    let mut line = String::from(start);
    let mut safe_tiles = start.chars().filter(|&x| x == '.').count();
    for _ in 1..rows {
        line = next_line(line.as_str());
        safe_tiles += line.chars().filter(|&x| x == '.').count();
    }
    return safe_tiles
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("# safe tiles (40   rows): {}", safe_tile_count(input.trim(), 40));
    println!("# safe tiles (400k rows): {}", safe_tile_count(input.trim(),
                                                             400000));
}

#[cfg(test)]
mod test {
    #[test]
    fn test_next_line_given() {
        assert_eq!(".^^^^", super::next_line("..^^."));
    }

    #[test]
    fn test_sum_given() {
        assert_eq!(38, super::safe_tile_count(".^^.^.^^^^", 10));
    }
}
