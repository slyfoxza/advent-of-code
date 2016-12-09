use std::io::{self, Read};

fn decompress(input: &str) -> String {
    let mut s = input;
    let mut result = String::with_capacity(input.len());
    loop {
        match s.find('(') {
            Some(il) => {
                result.push_str(&s[..il]);
                s = &s[il+1..];
                let ir = s.find(')').unwrap();
                let marker = &s[..ir];
                let (count, repeat) =
                    marker.split_at(marker.find('x').unwrap());
                let count = count.parse::<usize>().unwrap();
                let repeat = (&repeat[1..]).parse().unwrap();
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

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let output = decompress(input.trim());
    println!("{}", output.len());
}

#[cfg(test)]
mod tests {
    use super::decompress;

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
    fn test_reddit() {
        /* Darn trailing newline that wasn't trimmed made me look up a test
         * case from /r/adventofcode */
        assert_eq!("X(3x3)AB((3x3)AB(2x2)CY",
                   decompress("X(8x2)(3x3)AB(2x2)CY"));
    }
}
