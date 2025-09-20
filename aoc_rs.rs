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
use std::ffi::{CString, c_char};

#[path = "y2015/d01/rust.rs"] mod y2015_d01;
#[path = "y2015/d02/rust.rs"] mod y2015_d02;

/// Converts a pair of integer answers to strings, and copies them into the C buffers.
pub fn copy_int_answers<E: std::fmt::Debug>(
    result: Result<(i32, i32), E>,
    buf1: *mut c_char,
    buf2: *mut c_char,
    buflen: usize
) {
    let result = result.unwrap();

    let mut str1: String = result.0.to_string();
    str1.truncate(buflen - 1);
    let cstr1 = CString::new(str1).unwrap();

    let mut str2: String = result.1.to_string();
    str2.truncate(buflen - 1);
    let cstr2 = CString::new(str2).unwrap();

    unsafe {
        let bytes = cstr1.as_bytes_with_nul();
        std::ptr::copy(bytes.as_ptr(), buf1.cast::<u8>(), std::cmp::min(bytes.len(), buflen));
    }
    unsafe {
        let bytes = cstr2.as_bytes_with_nul();
        std::ptr::copy(bytes.as_ptr(), buf2.cast::<u8>(), std::cmp::min(bytes.len(), buflen));
    }
}
