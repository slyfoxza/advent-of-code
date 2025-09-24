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
use std::collections::HashSet;
use std::ffi::c_char;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn solve() -> std::io::Result<(i32, i32)> {
    let mut solo_santa = Point { x: 0, y: 0 };

    let mut santa = Point { x: 0, y: 0 };
    let mut robo = Point { x: 0, y: 0 };
    let mut is_robo = true;

    let mut visited1 = HashSet::new();
    let mut visited2 = HashSet::new();

    visited1.insert(Point { x: 0, y: 0 });
    visited2.insert(Point { x: 0, y: 0 });

    for c in BufReader::new(File::open("y2015/d03/input")?).bytes() {
        let current = if is_robo { &mut santa } else { &mut robo };
        is_robo = !is_robo;

        match c? {
            b'^' => {
                solo_santa.y -= 1;
                current.y -= 1;
            }
            b'v' => {
                solo_santa.y += 1;
                current.y += 1;
            }
            b'<' => {
                solo_santa.x -= 1;
                current.x -= 1;
            }
            b'>' => {
                solo_santa.x += 1;
                current.x += 1;
            }
            _ => {}
        };

        visited1.insert(solo_santa.clone());
        visited2.insert(current.clone());
    }

    Ok((visited1.len() as i32, visited2.len() as i32))
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_y2015_d03(buf1: *mut c_char, buf2: *mut c_char, buflen: usize) {
    crate::copy_int_answers(solve(), buf1, buf2, buflen);
}
