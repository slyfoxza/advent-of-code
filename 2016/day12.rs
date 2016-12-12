use std::io::{self, Read};

fn execute(input: &str) -> [i32; 4] {
    let mut registers = [0; 4];
    let lines = input.lines().collect::<Vec<_>>();
    let mut ip: i32 = 0;
    while ip < lines.len() as i32 {
        println!("{}: {} {} {} {}: {}", ip, registers[0], registers[1],
                 registers[2], registers[3], lines[ip as usize]);
        match lines[ip as usize] {
            ln if ln.starts_with("inc ") => {
                let register = (&ln[4..].as_bytes()[0] - b'a') as usize;
                registers[register] += 1;
                ip += 1;
            },
            ln if ln.starts_with("dec ") => {
                let register = (&ln[4..].as_bytes()[0] - b'a') as usize;
                registers[register] -= 1;
                ip += 1;
            },
            ln if ln.starts_with("cpy ") => {
                let operands = &ln[4..].split_whitespace().collect::<Vec<_>>();
                let register = (operands[1].as_bytes()[0] - b'a') as usize;
                if let Ok(value) = operands[0].parse() {
                    registers[register] = value;
                } else {
                    let src = (operands[0].as_bytes()[0] - b'a') as usize;
                    registers[register] = registers[src];
                }
                ip += 1;
            },
            ln if ln.starts_with("jnz ") => {
                let operands = &ln[4..].split_whitespace().collect::<Vec<_>>();
                let distance = operands[1].parse::<i32>().unwrap();
                if let Ok(value) = operands[0].parse::<i32>() {
                    ip += if value != 0 { distance } else { 1 };
                } else {
                    let register = (operands[0].as_bytes()[0] - b'a') as usize;
                    ip += if registers[register] != 0 { distance} else { 1 };
                }
            },
            ln => {
                println!("Unhandled instruction '{}'", ln);
                ip += 1;
            }
        }
    }
    registers
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let state = execute(input.as_str());
    println!("Register `a`: {}", state[0]);
}

#[cfg(tests)]
mod tests {
    use super::execute;

    #[test]
    fn test_given() {
        let state = execute("cpy 41 a\ninc a\ninc a\ndec a\njnz a 2\ndec a");
        assert_eq!(42, state[0]);
    }
}
