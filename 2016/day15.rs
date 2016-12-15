fn time(discs: Vec<(u32, u32, u32)>) -> u32 {
    let mut t = 0;
    loop {
        if discs.iter().all(|d| (t + d.0 + d.2) % d.1 == 0) {
            break;
        }
        t += 1;
    }
    t
}

fn main() {
    let t = time(vec![(1, 13, 11), (2, 5, 0), (3, 17, 11), (4, 3, 0), (5, 7, 2),
                      (6, 19, 17)]);
    println!("{}", t);
}

#[cfg(test)]
mod test {
    use super::time;

    #[test]
    fn test_given() {
        assert_eq!(5, time(vec![(1, 5, 4), (2, 2, 1)]));
    }
}
