#![no_main]
extern crate libc;
#[macro_use]
extern crate lazy_static;
extern crate itertools;
extern crate smallvec;

use libc::c_char;
use std::ffi::CStr;
use std::sync::Mutex;
use std::{char, str};
use std::cmp::{min, max};
use itertools::Itertools;
use smallvec::SmallVec;

fn find_pattern<F>(text: &str, output: &mut String, mut cmp: F)
    where F: FnMut(usize, usize) -> usize
{
    let mut columns = text.lines()
        .next()
        .map(|l| l.trim().bytes().map(|_| [0; 26]).collect())
        .unwrap_or_else(SmallVec::<[_; 8]>::new);

    for line in text.lines() {
        for (b, column) in line.trim().bytes().zip(columns.iter_mut()) {
            if let Some(count) = column.get_mut((b.wrapping_sub(b'a')) as usize) {
                *count += 1;
            }
        }
    }

    output.clear();
    output.extend(columns.iter()
        .map(|column| {
            column.iter()
                .cloned()
                .enumerate()
                .filter(|&(_, c)| c != 0)
                .fold1(|(a, c1), (b, c2)| if cmp(c1, c2) == c1 { (a, c1) } else { (b, c2) })
                .map_or(' ', |(b, _)| (b as u8 + b'a') as char)
        }))
}

lazy_static! {
    static ref OUTPUT: Mutex<String> = Mutex::new(String::new());
}

#[no_mangle]
pub unsafe extern "C" fn part1(text: *const c_char) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();

    find_pattern(text, &mut output, max);

    output.push('\0');
    output.as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn part2(text: *const c_char) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();

    find_pattern(text, &mut output, min);

    output.push('\0');
    output.as_ptr() as *const c_char
}
