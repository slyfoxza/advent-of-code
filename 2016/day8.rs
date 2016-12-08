use std::fmt::{self, Display, Formatter};
use std::io::{self, Read};
use std::iter::Iterator;

struct Screen {
    w: usize,
    h: usize,
    a: Vec<u8>
}

impl Screen {
    fn new(w: usize, h: usize) -> Screen {
        Screen { w: w, h: h, a: vec![0; w * h] }
    }

    fn count(&self) -> usize {
        self.a.iter().filter(|&x| *x == 1).count()
    }

    fn execute<'a, I: Iterator<Item = &'a str>>(&mut self, instructions: I) {
        for instruction in instructions {
            if instruction.starts_with("re") {
                let s = &instruction[5..];
                let (w, h) = s.split_at(s.find('x').unwrap());
                let w = w.parse().unwrap();
                let h = (&h[1..]).parse().unwrap();
                for y in 0..h {
                    for x in 0..w {
                        self.a[y * self.w + x] = 1;
                    }
                }
            } else if instruction.starts_with("rotate c") {
                let mut s = (&instruction[16..]).split(" by ");
                let x = s.next().unwrap().parse::<usize>().unwrap();
                let n = s.next().unwrap().parse::<usize>().unwrap();
                let mut shift = vec![0; self.h];
                for y in 0..self.h {
                    shift[(y + n) % self.h] = self.a[y * self.w + x];
                }
                for y in 0..self.h {
                    self.a[y * self.w + x] = shift[y];
                }
            } else if instruction.starts_with("rotate r") {
                let mut s = (&instruction[13..]).split(" by ");
                let y = s.next().unwrap().parse::<usize>().unwrap();
                let n = s.next().unwrap().parse::<usize>().unwrap();
                let mut shift = vec![0; self.w];
                let src = &mut self.a[y * self.w..(y + 1) * self.w];
                for x in 0..self.w {
                    shift[(x + n) % self.w] = src[x];
                }
                src.clone_from_slice(shift.as_slice());
            }
        }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut s = String::with_capacity((self.w + 1) * self.h);
        let mut i = 0;
        for c in self.a.iter() {
            s.push(if *c == 0 { ' ' } else { '#' });
            i += 1;
            if i == self.w {
                i = 0;
                s.push('\n');
            }
        }
        f.write_str(s.as_str())
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut screen = Screen::new(50, 6);
    screen.execute(input.lines());
    println!("{}", screen.count());
    println!("{}", screen);
}

#[cfg(test)]
mod tests {
    use super::Screen;

    #[test]
    fn test_given() {
        let mut screen = Screen::new(7, 3);
        screen.execute("rect 3x2\n\
                        rotate column x=1 by 1\n\
                        rotate row y=0 by 4\n\
                        rotate column x=1 by 1\n\
                        rect 2x2\n".lines());
        assert_eq!(8, screen.count());
    }
}
