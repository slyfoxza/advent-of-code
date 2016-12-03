use std::io::{self, Read};

fn count_possible_triangles(input: &str) -> usize {
    input.lines()
        .map(|x| {
            x.split_whitespace()
                .map(|y| u16::from_str_radix(y, 10).unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|x| x[0] + x[1] > x[2])
        .filter(|x| x[1] + x[2] > x[0])
        .filter(|x| x[0] + x[2] > x[1])
        .count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", count_possible_triangles(input.as_str()));
}

#[cfg(tests)]
mod test {
    use super::count_possible_triangles;

    #[test]
    fn valid_triangle() {
        assert_eq!(1, count_possible_triangles("3 4 5"))
    }

    #[test]
    fn given_tests() {
        assert_eq!(0, count_possible_triangles("5 10 25"));
    }

    #[test]
    fn mixed() {
        assert_eq!(1, count_possible_triangles("3 4 5\n5 10 25"));
    }
}
