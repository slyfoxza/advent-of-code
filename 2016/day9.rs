use std::collections::LinkedList;
use std::io::{self, Read};

fn parse_marker(marker: &str) -> (usize, usize) {
    let (count, repeat) = marker.split_at(marker.find('x').unwrap());
    (count.parse().unwrap(), (&repeat[1..]).parse().unwrap())
}

fn decompress(input: &str) -> String {
    let mut s = input;
    let mut result = String::with_capacity(input.len());
    loop {
        match s.find('(') {
            Some(il) => {
                result.push_str(&s[..il]);
                s = &s[il+1..];
                let ir = s.find(')').unwrap();
                let (count, repeat) = parse_marker(&s[..ir]);
                let span = &s[ir + 1..ir + 1 + count];
                for _ in 0..repeat {
                    result.push_str(span);
                }
                s = &s[ir + 1 + count..];
            },
            None => {
                result.push_str(s);
                return result;
            }
        }
    }
}

fn decompress2_len(input: &str) -> usize {
    let mut len = 0;
    let mut segs = LinkedList::new();
    segs.push_front(input);
    while !segs.is_empty() {
        let s = segs.pop_front().unwrap();
        match s.find('(') {
            Some(il) => {
                let ir = s.find(')').unwrap();
                let (count, repeat) = parse_marker(&s[il+1..ir]);
                let span = &s[ir + 1..ir + 1 + count];
                let trailing = &s[ir + 1 + count..];
                if !trailing.is_empty() {
                    segs.push_front(trailing);
                }
                match span.find('(') {
                    Some(_) => {
                        for _ in 0..repeat {
                            segs.push_front(span)
                        }
                    },
                    None => len += span.len() * repeat
                }
                let leading = &s[..il];
                if !leading.is_empty() {
                    segs.push_front(leading);
                }
            },
            None => len += s.len()
        }
    }
    len
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", decompress(input.trim()).len());
    println!("{}", decompress2_len(input.trim()));
}

#[cfg(test)]
mod tests {
    use super::{decompress, decompress2_len};

    #[test]
    fn test_given() {
        assert_eq!("ADVENT", decompress("ADVENT"));
        assert_eq!("ABBBBBC", decompress("A(1x5)BC"));
        assert_eq!("XYZXYZXYZ", decompress("(3x3)XYZ"));
        assert_eq!("ABCBCDEFEFG", decompress("A(2x2)BCD(2x2)EFG"));
        assert_eq!("(1x3)A", decompress("(6x1)(1x3)A"));
        assert_eq!("X(3x3)ABC(3x3)ABCY", decompress("X(8x2)(3x3)ABCY"));
    }

    #[test]
    fn test_given2() {
        assert_eq!(6, decompress2_len("ADVENT"));
        assert_eq!(9, decompress2_len("(3x3)XYZ"));
        assert_eq!(20, decompress2_len("X(8x2)(3x3)ABCY"));
        assert_eq!(241920,
                   decompress2_len("(27x12)(20x12)(13x14)(7x10)(1x12)A"));
        assert_eq!(445,
                   decompress2_len("(25x3)(3x3)ABC(2x3)XY(5x2)\
                                    PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"));
    }

    #[test]
    fn test_reddit() {
        /* Darn trailing newline that wasn't trimmed made me look up a test
         * case from /r/adventofcode */
        assert_eq!("X(3x3)AB((3x3)AB(2x2)CY",
                   decompress("X(8x2)(3x3)AB(2x2)CY"));
    }
}
