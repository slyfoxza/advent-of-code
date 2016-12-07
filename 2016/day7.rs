use std::io::{self, Read};

fn is_abba(s: &str) -> bool {
    let sb = s.as_bytes();
    assert_eq!(4, sb.len());
    let a = sb[0];
    let b = sb[1];
    (a != b) && (sb[2] == b) && (sb[3] == a)
}

fn has_abba(s: &str) -> bool {
    (0..s.len()-3).any(|i| is_abba(&s[i..i+4]))
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

fn is_aba(s: &str) -> bool {
    let sb = s.as_bytes();
    assert_eq!(3, sb.len());
    let a = sb[0];
    (a != sb[1]) && (sb[2] == a)
}

fn gather_abas<'a>(s: &'a str, abas: &mut Vec<&'a str>) {
    for aba in (0..s.len()-2).map(|i| &s[i..i+3]).filter(|s| is_aba(s)) {
        abas.push(aba);
    }
}

fn find_aba_bab(abas: &Vec<&str>, babs: &Vec<&str>) -> bool {
    if abas.is_empty() || babs.is_empty() { return false }
    abas.iter()
        .map(|aba| aba.as_bytes())
        .any(|aba| {
            babs.iter()
                .map(|bab| bab.as_bytes())
                .any(|bab| (aba[0] == bab[1]) && (aba[1] == bab[0]))
        })
}

fn supports_ssl(ip: &str) -> bool {
    let mut abas = Vec::new();
    let mut babs = Vec::new();
    let mut s = ip;
    loop {
        match s.find('[') {
            None => {
                abas.clear();
                gather_abas(s, &mut abas);
                return find_aba_bab(&abas, &babs);
            },
            Some(il) => {
                gather_abas(&s[..il], &mut abas);
                if find_aba_bab(&abas, &babs) { return true }
                let ir = s.find(']').unwrap();
                gather_abas(&s[il+1..ir], &mut babs);
                if find_aba_bab(&abas, &babs) { return true }
                s = &s[ir+1..];
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("IPs supporting TLS: {}",
             input.lines().filter(|ip| supports_tls(ip)).count());
    println!("IPs supporting SSL: {}",
             input.lines().filter(|ip| supports_ssl(ip)).count());
}

#[cfg(test)]
mod tests {
    use super::{is_abba, supports_ssl, supports_tls};

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

    #[test]
    fn supports_ssl_given() {
        assert!(supports_ssl("aba[bab]xyz"));
        assert!(!supports_ssl("xyx[xyx]xyx"));
        assert!(supports_ssl("aaa[kek]eke"));
        assert!(supports_ssl("zazbz[bzb]cdb"));
    }
}
