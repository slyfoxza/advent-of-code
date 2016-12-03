use std::io::{self, Read};

fn line_to_vec(input: &str) -> Vec<u16> {
    input.split_whitespace()
        .map(|x| u16::from_str_radix(x, 10).unwrap())
        .collect::<Vec<_>>()
}

fn count_possible_triangles(input: &str) -> usize {
    input.lines()
        .map(line_to_vec)
        .filter(|x| x[0] + x[1] > x[2])
        .filter(|x| x[1] + x[2] > x[0])
        .filter(|x| x[0] + x[2] > x[1])
        .count()
}

fn count_vertically(input: &str) -> usize {
    let mut count = 0;
    let mut rows = input.lines().map(line_to_vec);
    loop {
        let next = rows.next();
        if next.is_some() {
            let rows = [next.unwrap(), rows.next().unwrap(),
                        rows.next().unwrap()];
            for i in 0..3 {
                if rows[0][i] + rows[1][i] > rows[2][i]
                        && rows[1][i] + rows[2][i] > rows[0][i]
                        && rows[0][i] + rows[2][i] > rows[1][i] {
                            count += 1;
                        }
            }
        } else {
            break;
        }
    }
    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{} {}", count_possible_triangles(input.as_str()),
             count_vertically(input.as_str()));
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
