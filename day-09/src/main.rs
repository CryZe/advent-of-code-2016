#![no_main]
extern crate libc;
#[macro_use]
extern crate lazy_static;

use libc::c_char;
use std::ffi::CStr;
use std::str;
use std::sync::Mutex;
use std::fmt::Write;

enum Iter<'a> {
    Done,
    Normal(&'a str),
    Parentheses(&'a str),
}

impl<'a> Iterator for Iter<'a> {
    type Item = Result<(u64, &'a str), &'static str>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (state, result) = match *self {
                Iter::Done => {
                    return None;
                }
                Iter::Normal(remaining) => {
                    if let Some(index) = remaining.find('(') {
                        (Iter::Parentheses(&remaining[index + 1..]),
                         Some(Ok((1, &remaining[..index]))))
                    } else {
                        (Iter::Done, Some(Ok((1, remaining))))
                    }
                }
                Iter::Parentheses(remaining) => {
                    fn parse(text: &str) -> Result<(u64, &str, &str), &'static str> {
                        let index = text.find('x').ok_or("Can't find x in parentheses")?;
                        let len = text[..index].parse().map_err(|_| "Can't parse length")?;
                        let text = &text[index + 1..];
                        let index = text.find(')').ok_or("Can't find closing parentheses")?;
                        let count = text[..index].parse().map_err(|_| "Can't parse count")?;
                        let text = &text[index + 1..];
                        if len <= text.len() {
                            let (part, remaining) = text.split_at(len);
                            Ok((count, part, remaining))
                        } else {
                            Err("Length is out of bounds")
                        }
                    }
                    match parse(remaining) {
                        Ok((count, part, remaining)) => {
                            (Iter::Normal(remaining), Some(Ok((count, part))))
                        }
                        Err(e) => (Iter::Done, Some(Err(e))),
                    }
                }
            };
            *self = state;
            if let Some(result) = result {
                return Some(result);
            }
        }
    }
}

fn len_v1(text: &str) -> Result<u64, &'static str> {
    let mut len = 0;
    for part in Iter::Normal(text) {
        let (count, part) = part?;
        len += count * part.len() as u64;
    }
    Ok(len)
}

fn len_v2(text: &str) -> Result<u64, &'static str> {
    let mut len = 0;
    for part in Iter::Normal(text) {
        let (count, part) = part?;
        let part_len = if part.contains('(') {
            len_v2(part)?
        } else {
            part.len() as u64
        };
        len += count * part_len;
    }
    Ok(len)
}

lazy_static! {
    static ref OUTPUT: Mutex<String> = Mutex::new(String::new());
}

#[no_mangle]
pub unsafe extern "C" fn part1(text: *const c_char) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();
    output.clear();

    match len_v1(text.trim()) {
        Ok(count) => write!(output, "{}", count).unwrap(),
        Err(e) => output.push_str(e),
    }

    output.push('\0');
    output.as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn part2(text: *const c_char) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();
    output.clear();

    match len_v2(text.trim()) {
        Ok(count) => write!(output, "{}", count).unwrap(),
        Err(e) => output.push_str(e),
    }

    output.push('\0');
    output.as_ptr() as *const c_char
}
