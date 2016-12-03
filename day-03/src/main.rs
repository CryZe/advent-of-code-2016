#![no_main]
extern crate libc;

use libc::c_char;
use std::ffi::CStr;
use std::iter::Map;
use std::str::Lines;

fn check_validity(triangle: &[u64; 3]) -> bool {
    [triangle[0], triangle[1], triangle[2], triangle[0], triangle[1]]
        .windows(3)
        .all(|t| t[0] + t[1] > t[2])
}

fn parse_line(line: &str) -> [u64; 3] {
    let mut s = line.split_whitespace();
    let a = s.next().unwrap().parse().unwrap();
    let b = s.next().unwrap().parse().unwrap();
    let c = s.next().unwrap().parse().unwrap();
    [a, b, c]
}

fn parse(text: &str) -> Map<Lines, fn(&str) -> [u64; 3]> {
    text.lines().map(parse_line)
}

fn count_valid_horizontal(text: &str) -> usize {
    parse(text).filter(check_validity).count()
}

fn count_valid_vertical(text: &str) -> usize {
    let triangles = parse(text).collect::<Vec<_>>();
    let mut count = 0;
    for chunk in triangles.chunks(3) {
        if chunk.len() == 3 {
            for ((&a, &b), &c) in chunk[0].iter().zip(chunk[1].iter()).zip(chunk[2].iter()) {
                count += check_validity(&[a, b, c]) as usize;
            }
        }
    }
    count
}

#[no_mangle]
pub unsafe extern "C" fn calculate(text: *const c_char, part2: bool) -> u64 {
    let text = CStr::from_ptr(text).to_str().unwrap();
    if part2 {
        count_valid_vertical(text) as u64
    } else {
        count_valid_horizontal(text) as u64
    }
}
