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
use std::cmp::{max, min};
use std::error::Error;
use std::ffi::c_char;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn solve() -> Result<(i32, i32), Box<dyn Error>> {
    let mut paper = 0;
    let mut ribbon = 0;
    for line in BufReader::new(File::open("y2015/d02/input")?).lines() {
        if let Ok(ref ln) = line {
            let mut dims = ln.split('x');
            let l = dims.next().unwrap().parse::<i32>()?;
            let w = dims.next().unwrap().parse::<i32>()?;
            let h = dims.next().unwrap().parse::<i32>()?;

            let lw = l * w;
            let wh = w * h;
            let hl = h * l;
            paper += 2 * (lw + wh + hl) + min(lw, min(wh, hl));
            ribbon += 2 * (l + w + h - max(l, max(w, h))) + l * w * h;
        }
    }
    Ok((paper, ribbon))
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_y2015_d02(buf1: *mut c_char, buf2: *mut c_char, buflen: usize) {
    crate::copy_int_answers(solve(), buf1, buf2, buflen);
}
