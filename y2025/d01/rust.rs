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

fn solve() -> Result<(i32, i32), Box<dyn Error>> {
    let mut dial = 50;
    let mut zeroes = 0;
    let mut clicks = 0;
    for line in BufReader::new(File::open("y2025/d01/input")?).lines() {
        if let Ok(ref ln) = line {
            let sign = if ln.chars().next().unwrap() == 'L' { -1 } else { 1 };
            let mut value = ln[1..].parse::<i32>()?;

            while value > 100 {
                value -= 100;
                clicks += 1;
            }

            let dial_start = dial;
            dial = (dial + (sign * value)) % 100;
            if dial < 0 {
                dial += 100;
            }

            if dial == 0 {
                zeroes += 1;
            }

            if dial_start != 0 && sign == -1 && value >= dial_start {
                clicks += 1;
            } else if sign == 1 && value >= (100 - dial_start) {
                clicks += 1;
            }
        }
    }

    Ok((zeroes, clicks))
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_y2025_d01(buf1: *mut c_char, buf2: *mut c_char, buflen: usize) {
    crate::copy_int_answers(solve(), buf1, buf2, buflen);
}
