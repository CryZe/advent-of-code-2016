#![no_main]
extern crate libc;
#[macro_use]
extern crate lazy_static;

use libc::c_char;
use std::ffi::CStr;
use std::sync::Mutex;
use std::fmt::Write;

mod part1;
mod part2;

#[derive(Copy, Clone, PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

lazy_static! {
    static ref OUTPUT: Mutex<String> = Mutex::new(String::new());
}

#[no_mangle]
pub unsafe extern "C" fn calculate(text: *const c_char, part2: bool) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();
    output.clear();

    if part2 {
        let result = part2::shift_all(5, text);
        write!(output, "{}\0", result).unwrap();
    } else {
        let result = part1::shift_all(5, text);
        write!(output, "{}\0", result).unwrap();
    }

    output.as_ptr() as *const c_char
}
