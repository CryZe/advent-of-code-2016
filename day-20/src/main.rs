#![no_main]
extern crate libc;
#[macro_use]
extern crate lazy_static;

use libc::c_char;
use std::ffi::CStr;
use std::str;
use std::sync::Mutex;
use std::fmt::Write;

type Error = &'static str;
type Result<T> = ::std::result::Result<T, Error>;

fn parse_range(text: &str) -> Result<(u32, u32)> {
    let mut splits = text.split('-');
    let min = splits.next()
        .ok_or("Missing range minimum")?
        .parse()
        .map_err(|_| "Couldn't parse range minimum")?;
    let max = splits.next()
        .ok_or("Missing range maximum")?
        .parse()
        .map_err(|_| "Couldn't parse range maximum")?;
    Ok((min, max))
}

fn parse_all(text: &str) -> Result<Vec<(u32, u32)>> {
    text.lines().map(str::trim).map(parse_range).collect::<Result<Vec<_>>>().map(|mut v| {
        v.sort();
        v
    })
}

fn find_min_allowed(ranges: &[(u32, u32)]) -> u32 {
    let mut min_allowed = 0;
    for &(min, max) in ranges {
        if min > min_allowed {
            break;
        }
        if max >= min_allowed {
            min_allowed = max + 1;
        }
    }
    min_allowed
}

fn count_ips_allowed(ranges: &[(u32, u32)]) -> u32 {
    let mut count = 0;
    let mut min_allowed = 0;
    for &(min, max) in ranges {
        if min > min_allowed {
            count += min - min_allowed;
        }
        if max >= min_allowed {
            if let Some(new_min_allowed) = max.checked_add(1) {
                min_allowed = new_min_allowed;
            } else {
                return count;
            }
        }
    }
    count + (u32::max_value() - min_allowed) + 1
}

lazy_static! {
    static ref OUTPUT: Mutex<String> = Mutex::new(String::new());
}

#[no_mangle]
pub unsafe extern "C" fn part1(text: *const c_char) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();
    output.clear();

    match parse_all(text).map(|r| find_min_allowed(&r)) {
        Ok(min) => writeln!(output, "Minimum IP: {}", min).unwrap(),
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

    match parse_all(text).map(|r| count_ips_allowed(&r)) {
        Ok(count) => writeln!(output, "IPs allowed: {}", count).unwrap(),
        Err(e) => writeln!(output, "Error: {}", e).unwrap(),
    }

    output.push('\0');
    output.as_ptr() as *const c_char
}
