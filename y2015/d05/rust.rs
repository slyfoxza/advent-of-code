/* Copyright 2025 Philip Cronje
 *
 * This file is part of my Advent of Code solution repository. It is free software: you can
 * redistribute it and/or modify it under the terms of the GNU General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or (at your option) any later
 * version.
 *
 * This repository is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with this repository. If
 * not, see <https://www.gnu.org/licenses/>. */
use std::error::Error;
use std::ffi::c_char;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false
    }
}

fn solve() -> Result<(i32, i32), Box<dyn Error>> {
    let mut nice = 0;
    let mut nice2 = 0;
    for line in BufReader::new(File::open("y2015/d05/input")?).lines() {
        if let Ok(ref ln) = line {
            let mut vowels = 0;
            let mut doubled = false;
            let mut forbidden_pair = false;

            let mut double_pair = false;
            let mut sandwich_pair = false;

            let chars = ln.chars().collect::<Vec<char>>();
            if is_vowel(chars[0]) {
                vowels += 1;
            }

            for pair in chars.windows(2) {
                let c0 = pair[0];
                let c1 = pair[1];
                if (c0 == 'a' || c0 == 'c' || c0 == 'p' || c0 == 'x') && c1 as u32 == (c0 as u32) + 1 {
                    forbidden_pair = true;
                }

                if is_vowel(c1) {
                    vowels += 1;
                }
                if c0 == c1 {
                    doubled = true;
                }

                let needle_chars = [c0 as u8, c1 as u8];
                let needle = str::from_utf8(&needle_chars)?;
                if !double_pair {
                    let mut matches = ln.matches(needle);
                    if matches.next().is_some() && matches.next().is_some() {
                        double_pair = true;
                    }
                }
            }

            for triplet in chars.windows(3) {
                if triplet[0] == triplet[2] {
                    sandwich_pair = true;
                    break;
                }
            }

            if doubled && !forbidden_pair && vowels >= 3 {
                nice += 1;
            }
            if double_pair && sandwich_pair {
                nice2 += 1;
            }
        }
    }
    Ok((nice, nice2))
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_y2015_d05(buf1: *mut c_char, buf2: *mut c_char, buflen: usize) {
    crate::copy_int_answers(solve(), buf1, buf2, buflen);
}
