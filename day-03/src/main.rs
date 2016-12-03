#![no_main]
extern crate libc;

use libc::c_char;
use std::ffi::CStr;
use std::iter::Map;
use std::str::Lines;
use std::cmp::max;

fn check_validity(triangle: &[u32; 3]) -> bool {
    let (a, b, c) = (triangle[0], triangle[1], triangle[2]);
    let sum = a + b + c;
    let max = max(a, max(b, c));
    sum - max > max
}

fn parse_line(line: &str) -> [u32; 3] {
    let mut s = line.split_whitespace();
    let a = s.next().unwrap().parse().unwrap();
    let b = s.next().unwrap().parse().unwrap();
    let c = s.next().unwrap().parse().unwrap();
    [a, b, c]
}

fn parse(text: &str) -> Map<Lines, fn(&str) -> [u32; 3]> {
    text.lines().map(parse_line)
}

fn count_valid_horizontal(text: &str) -> usize {
    parse(text).filter(check_validity).count()
}

fn count_valid_vertical(text: &str) -> usize {
    let mut lines = parse(text).fuse();
    let mut count = 0;
    while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
        for ((&a, &b), &c) in a.iter().zip(b.iter()).zip(c.iter()) {
            count += check_validity(&[a, b, c]) as usize;
        }
    }
    count
}

#[no_mangle]
pub unsafe extern "C" fn calculate(text: *const c_char, part2: bool) -> u32 {
    let text = CStr::from_ptr(text).to_str().unwrap();
    if part2 {
        count_valid_vertical(text) as u32
    } else {
        count_valid_horizontal(text) as u32
    }
}
