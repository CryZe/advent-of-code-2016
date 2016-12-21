#![no_main]
extern crate libc;
#[macro_use]
extern crate lazy_static;

use libc::c_char;
use std::ffi::CStr;
use std::str;
use std::sync::Mutex;
use std::fmt::Write;

fn find_elf_when_stealing_from_next(n: u32) -> u32 {
    let clz = n.leading_zeros();
    (((1 << 31) >> clz) ^ n) << 1 | 1
}

fn find_elf_when_stealing_from_other_side(n: u32) -> u32 {
    let b = f64::powf(3.0, f64::log(n as f64, 3.0).floor()) as u32;
    if n == b {
        n
    } else if n <= 2 * b {
        n - b
    } else {
        2 * n - 3 * b
    }
}

lazy_static! {
    static ref OUTPUT: Mutex<String> = Mutex::new(String::new());
}

#[no_mangle]
pub unsafe extern "C" fn part1(text: *const c_char) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();
    output.clear();

    match text.trim().parse().map(find_elf_when_stealing_from_next) {
        Ok(elf) => writeln!(output, "Elf with the presents: #{}", elf).unwrap(),
        Err(e) => writeln!(output, "Error: {}", e).unwrap(),
    }

    output.push('\0');
    output.as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn part2(text: *const c_char) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();
    output.clear();

    // for c in 1..201 {
    //     writeln!(output,
    //              "{} -> {}",
    //              c,
    //              find_elf_when_stealing_from_other_side(c))
    //         .ok();
    // }

    match text.trim().parse().map(find_elf_when_stealing_from_other_side) {
        Ok(elf) => writeln!(output, "Elf with the presents: #{}", elf).unwrap(),
        Err(e) => writeln!(output, "Error: {}", e).unwrap(),
    }

    output.push('\0');
    output.as_ptr() as *const c_char
}
