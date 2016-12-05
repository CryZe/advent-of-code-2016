#![no_main]
extern crate libc;
extern crate md5;
#[macro_use]
extern crate lazy_static;

use libc::c_char;
use std::ffi::CStr;
use std::sync::Mutex;
use std::fmt::Write;
use std::{char, str};

fn calculate_hash(base: &mut String, index: u64) -> Option<[u8; 2]> {
    let len = base.len();
    write!(base, "{}", index).unwrap();
    let digest = md5::compute(base.as_bytes());
    base.truncate(len);

    if digest[0] | digest[1] | digest[2] >> 4 == 0 {
        Some([digest[2] & 0xF, digest[3] >> 4])
    } else {
        None
    }
}

fn find_result_part1(base: &str, result: &mut String) {
    let mut base = base.to_owned();
    result.clear();

    for index in 0.. {
        let digits = calculate_hash(&mut base, index);

        if let Some(digits) = digits {
            result.push(char::from_digit(digits[0] as u32, 16).unwrap());

            if result.len() == 8 {
                return;
            }
        }
    }
}

fn find_result_part2(base: &str, result: &mut String) {
    let mut base = base.to_owned();
    result.clear();
    let mut bytes = [0; 8];
    let mut remaining = 8;

    for index in 0.. {
        let digits = calculate_hash(&mut base, index);

        if let Some(digits) = digits {
            let i = digits[0] as usize;
            if i < 8 && bytes[i] == 0 {
                let c = char::from_digit(digits[1] as u32, 16).unwrap();
                bytes[i] = c as u8;
                remaining -= 1;
                if remaining == 0 {
                    let bytes = unsafe { str::from_utf8_unchecked(&bytes) };
                    result.push_str(bytes);
                    return;
                }
            }
        }
    }
}

lazy_static! {
    static ref OUTPUT: Mutex<String> = Mutex::new(String::new());
}

#[no_mangle]
pub unsafe extern "C" fn part1(text: *const c_char) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();

    find_result_part1(text, &mut output);

    output.as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn part2(text: *const c_char) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();
    output.clear();

    find_result_part2(text, &mut output);

    output.as_ptr() as *const c_char
}
