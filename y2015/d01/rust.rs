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
use std::ffi::c_char;
use std::fs::File;
use std::io::{self, BufReader, Read};

fn solve() -> io::Result<(i32, i32)> {
    let mut floor = 0;
    let mut position = -1;
    let mut i = 0;
    for c in BufReader::new(File::open("y2015/d01/input")?).bytes() {
        match c? {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => {}
        };

        i += 1;
        if position == -1 && floor < 0 {
            position = i;
        }
    }
    Ok((floor, position))
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_y2015_d01(buf1: *mut c_char, buf2: *mut c_char, buflen: usize) {
    crate::copy_int_answers(solve(), buf1, buf2, buflen);
}
