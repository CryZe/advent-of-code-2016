#![feature(conservative_impl_trait)]

#![no_main]
extern crate libc;
#[macro_use]
extern crate lazy_static;

use libc::c_char;
use std::ffi::CStr;
use std::str;
use std::sync::Mutex;
use std::fmt::Write;
use std::iter::once;
use std::mem::swap;

fn is_safe(window: &[bool]) -> bool {
    window[0] == window[2]
}

fn next_row_iter<'a>(row: &'a [bool]) -> impl Iterator<Item = bool> + 'a {
    once(row[1]).chain(row.windows(3).map(is_safe)).chain(once(row[row.len() - 2]))
}

fn count(row: Vec<bool>, row_count: u64) -> u64 {
    let (mut dst, mut src) = (Vec::with_capacity(row.len()), row);
    let mut row = 0;
    let mut count = 0;

    loop {
        count += src.iter().filter(|&&x| x).count() as u64;

        row += 1;
        if row == row_count {
            return count;
        }

        dst.clear();
        dst.extend(next_row_iter(&src));

        swap(&mut dst, &mut src);
    }
}

fn parse(text: &str) -> Result<Vec<bool>, &'static str> {
    text.bytes()
        .map(|b| match b {
            b'.' => Ok(true),
            b'^' => Ok(false),
            _ => Err("Unknown character"),
        })
        .collect()
}

lazy_static! {
    static ref OUTPUT: Mutex<String> = Mutex::new(String::new());
}

#[no_mangle]
pub unsafe extern "C" fn count_safe_tiles(text: *const c_char,
                                          row_count: *const c_char)
                                          -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let row_count = CStr::from_ptr(row_count).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();
    output.clear();

    match parse(text.trim())
        .and_then(|r| row_count.trim().parse().map_err(|_| "Can't parse row count").map(|c| (r, c)))
        .map(|(r, c)| count(r, c)) {
        Ok(count) => writeln!(output, "Safe Tiles: {}", count).unwrap(),
        Err(e) => writeln!(output, "Error: {}", e).unwrap(),
    }

    output.push('\0');
    output.as_ptr() as *const c_char
}
