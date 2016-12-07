use std::io::{self, Read};

fn is_abba(s: &str) -> bool {
    let sb = s.as_bytes();
    assert_eq!(4, sb.len());
    let a = sb[0];
    let b = sb[1];
    return (a != b) && (sb[2] == b) && (sb[3] == a);
}

fn has_abba(s: &str) -> bool {
    let r = (0..s.len()-3).any(|i| is_abba(&s[i..i+4]));
    r
}

fn supports_tls(ip: &str) -> bool {
    let mut tls = false;
    let mut s = ip;
    loop {
        match s.find('[') {
            None => return tls || has_abba(s),
            Some(il) => {
                tls = tls || has_abba(&s[..il]);
                let ir = s.find(']').unwrap();
                if has_abba(&s[il+1..ir]) {
                    return false;
                }
                s = &s[ir+1..];
            }
        };
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("IPs supporting TLS: {}",
             input.lines().filter(|ip| supports_tls(ip)).count());
}

#[cfg(test)]
mod tests {
    use super::{is_abba, supports_tls};

    #[test]
    fn is_abba_positive() {
        assert!(is_abba("abba"));
    }

    #[test]
    fn is_abba_negative() {
        assert!(!is_abba("abab"));
    }

    #[test]
    fn supports_tls_given() {
        assert!(supports_tls("abba[mnop]qrst"));
        assert!(!supports_tls("abcd[bddb]xyyx"));
        assert!(!supports_tls("aaaa[qwer]tyui"));
        assert!(supports_tls("ioxxoj[asdfhg]zxcvbn"));
    }

    #[test]
    fn supports_tls_positive() {
        assert!(supports_tls("qrst[mnop]abba[ponm]tsrq"));
    }
}
