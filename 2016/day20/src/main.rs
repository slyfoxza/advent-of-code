use std::fmt::{self, Display, Formatter};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
struct IpRangeParseError;

struct IpRange {
    start: u32,
    end: u32
}

impl Display for IpRange {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl FromStr for IpRange {
    type Err = IpRangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find('-') {
            Some(i) => {
                let parts = s.split_at(i);
                match parts.0.parse() {
                    Ok(start) => match (&parts.1[1..]).parse() {
                        Ok(end) => Ok(IpRange { start: start, end: end }),
                        Err(_) => Err(IpRangeParseError)
                    },
                    Err(_) => Err(IpRangeParseError)
                }
            },
            None => Err(IpRangeParseError)
        }
    }
}

fn process(valid_ranges: &mut Vec<IpRange>, line: &str) {
    let line = line.trim();
    let del = line.parse::<IpRange>().unwrap();
    for i in 0..valid_ranges.len() {
        if del.start > valid_ranges[i].end {
            continue;
        } else if del.end < valid_ranges[i].start {
            return;
        }
        for j in i..valid_ranges.len() {
            if del.end > valid_ranges[j].end {
                if j < valid_ranges.len() - 1 && del.end > valid_ranges[j + 1].start {
                    continue;
                }
            }
            if del.start <= valid_ranges[i].start && del.end >= valid_ranges[j].end {
                for k in (i..j+1).rev() { valid_ranges.remove(k); }
            } else if del.start <= valid_ranges[i].start && del.end < valid_ranges[j].end {
                // Delete everything except j, which gets adjusted
                for k in (i..j).rev() { valid_ranges.remove(k); }
                // j will now be at i
                valid_ranges[i].start = del.end + 1;
            } else if del.start > valid_ranges[i].start && del.end >= valid_ranges[j].end {
                // Delete everything except i, which gets adjusted
                for k in (i+1..j+1).rev() { valid_ranges.remove(k); }
                valid_ranges[i].end = del.start - 1;
            } else if del.start > valid_ranges[i].start && del.end < valid_ranges[j].end {
                if i == j {
                    // Need to split the range
                    let original_end = valid_ranges[i].end;
                    valid_ranges[i].end = del.start - 1;
                    valid_ranges.insert(i + 1, IpRange { start: del.end + 1, end: original_end });
                } else {
                    // Delete everything except i and j, which get adjusted
                    for k in (i+1..j).rev() { valid_ranges.remove(k); }
                    // j will now be at i+1
                    valid_ranges[i].end = del.start - 1;
                    valid_ranges[i + 1].start = del.end + 1;
                }
            } else {
                break;
            }
            return;
        }
        break;
    }
    panic!("No match: {}", line);
}

fn count_valid(valid_ranges: &Vec<IpRange>) -> u32 {
    let mut n_valid = 0;
    for valid_range in valid_ranges {
        let nv = valid_range.end - valid_range.start + 1;
        n_valid += nv;
    }
    n_valid
}

fn main() {
    let mut valid_ranges = vec![IpRange { start: 0, end: u32::max_value() }];
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    for line in input.lines() {
        process(&mut valid_ranges, line);
    }
    println!("First range: {}", valid_ranges[0]);
    println!("# valid values: {}", count_valid(&valid_ranges));
}

#[cfg(test)]
mod test {
    use super::IpRange;

    #[test]
    fn test_given() {
        let mut valid_ranges = vec![IpRange { start: 0, end: 9 }];
        for line in vec!["5-8", "0-2", "4-7"] {
            super::process(&mut valid_ranges, line);
        }
        assert_eq!(3, valid_ranges[0].start);
        assert_eq!(3, valid_ranges[0].end);
        assert_eq!(2, super::count_valid(&valid_ranges));
    }

    #[test]
    fn test_exact_removal() {
        let mut valid_ranges = vec![
            IpRange { start: 0, end: 2 },
            IpRange { start: 4, end: 6 },
            IpRange { start: 8, end: 10 }
        ];
        super::process(&mut valid_ranges, "0-6");
        assert_eq!(8, valid_ranges[0].start);
        assert_eq!(10, valid_ranges[0].end);
        assert_eq!(3, super::count_valid(&valid_ranges));
    }

    #[test]
    fn test_excessive_removal() {
        let mut valid_ranges = vec![
            IpRange { start: 0, end: 2 },
            IpRange { start: 4, end: 6 },
            IpRange { start: 8, end: 10 }
        ];
        super::process(&mut valid_ranges, "0-7");
        assert_eq!(8, valid_ranges[0].start);
        assert_eq!(10, valid_ranges[0].end);
        assert_eq!(3, super::count_valid(&valid_ranges));
    }

    #[test]
    fn test_overlap() {
        let mut valid_ranges = vec![IpRange { start: 0, end: 9 }];
        for line in vec!["0-3", "1-5"] {
            super::process(&mut valid_ranges, line);
        }
        assert_eq!(6, valid_ranges[0].start);
        assert_eq!(9, valid_ranges[0].end);
        assert_eq!(4, super::count_valid(&valid_ranges));
    }

    #[test]
    fn test_bridge_the_gap() {
        let mut valid_ranges = vec![
            IpRange { start: 0, end: 2 },
            IpRange { start: 4, end: 6 }
        ];
        super::process(&mut valid_ranges, "1-5");
        assert_eq!(0, valid_ranges[0].start);
        assert_eq!(0, valid_ranges[0].end);
        assert_eq!(6, valid_ranges[1].start);
        assert_eq!(6, valid_ranges[1].end);
        assert_eq!(2, super::count_valid(&valid_ranges));
    }

    #[test]
    fn test_bridge_the_gaps_left() {
        let mut valid_ranges = vec![
            IpRange { start: 0, end: 2 },
            IpRange { start: 4, end: 6 },
            IpRange { start: 8, end: 10 }
        ];
        super::process(&mut valid_ranges, "1-10");
        assert_eq!(0, valid_ranges[0].start);
        assert_eq!(0, valid_ranges[0].end);
        assert_eq!(1, valid_ranges.len());
        assert_eq!(1, super::count_valid(&valid_ranges));
    }

    #[test]
    fn test_bridge_the_gaps_right() {
        let mut valid_ranges = vec![
            IpRange { start: 0, end: 2 },
            IpRange { start: 4, end: 6 },
            IpRange { start: 8, end: 10 }
        ];
        super::process(&mut valid_ranges, "0-9");
        assert_eq!(10, valid_ranges[0].start);
        assert_eq!(10, valid_ranges[0].end);
        assert_eq!(1, valid_ranges.len());
        assert_eq!(1, super::count_valid(&valid_ranges));
    }

    #[test]
    fn test_actual1() {
        let mut valid_ranges = vec![
            IpRange { start:        0, end:  3021649 },
            IpRange { start: 31053880, end: 32923437 },
            IpRange { start: 48835051, end: 50881440 }
        ];
        super::process(&mut valid_ranges, "31053881-36411395");
        assert_eq!(31053880, valid_ranges[1].start);
        assert_eq!(31053880, valid_ranges[1].end);
    }

    #[test]
    fn test_actual2() {
        let mut valid_ranges = vec![
            IpRange { start: 31053880, end:  31053880 },
            // blacklist     43965520        47689181
            IpRange { start: 48835051, end:  50881440 },
            IpRange { start: 91509386, end: 100343324 }
        ];
        super::process(&mut valid_ranges, "43965520-47689181");
        assert_eq!(48835051, valid_ranges[1].start);
        assert_eq!(50881440, valid_ranges[1].end);
    }

    #[test]
    fn test_adjacent_ranges() {
        let mut valid_ranges = vec![IpRange { start: 0, end: 999 }];
        for line in vec!["0-99", "100-199", "200-299"] {
            super::process(&mut valid_ranges, line);
        }
        assert_eq!(300, valid_ranges[0].start);
        assert_eq!(999, valid_ranges[0].end);
        assert_eq!(700, super::count_valid(&valid_ranges));
    }
}
