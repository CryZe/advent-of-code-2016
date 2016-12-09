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
    type Item = Result<(u64, &'a str), (&'static str, &'a str)>;

    fn next(&mut self) -> Option<Self::Item> {
        let (state, result) = match *self {
            Iter::Done => {
                return None;
            }
            Iter::Normal(remaining) => {
                if let Some(index) = remaining.find('(') {
                    let (part, paren) = remaining.split_at(index);
                    (Iter::Parentheses(paren), Ok((1, part)))
                } else {
                    (Iter::Done, Ok((1, remaining)))
                }
            }
            Iter::Parentheses(remaining) => {
                fn parse(text: &str) -> Result<(u64, &str, &str), (&'static str, &str)> {
                    let index = text.find(')').ok_or(("Can't find closing parentheses", text))?;
                    let whole_paren = &text[..index + 1];
                    let paren = &text[1..index];
                    let remaining = &text[index + 1..];
                    let index = paren.find('x').ok_or(("Can't find x", whole_paren))?;
                    let len = paren[..index].parse()
                        .map_err(|_| ("Can't parse length", whole_paren))?;
                    let count = paren[index + 1..].parse()
                        .map_err(|_| ("Can't parse count", whole_paren))?;
                    if len <= remaining.len() {
                        let (part, remaining) = remaining.split_at(len);
                        Ok((count, part, remaining))
                    } else {
                        Err(("Length is out of bounds", text))
                    }
                }
                match parse(remaining) {
                    Ok((count, part, remaining)) => (Iter::Normal(remaining), Ok((count, part))),
                    Err(e) => (Iter::Done, Err(e)),
                }
            }
        };
        *self = state;
        Some(result)
    }
}

fn part_iter(text: &str) -> Iter {
    Iter::Normal(text)
}

fn len_v1(text: &str) -> Result<u64, (&'static str, &str)> {
    let mut len = 0;
    for part in part_iter(text) {
        let (count, part) = part?;
        len += count * part.len() as u64;
    }
    Ok(len)
}

fn len_v2(text: &str) -> Result<u64, (&'static str, &str)> {
    let mut len = 0;
    for part in part_iter(text) {
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
        Err((error, part)) => write!(output, "{} in '{}'", error, part).unwrap(),
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
        Err((error, part)) => write!(output, "{} in '{}'", error, part).unwrap(),
    }

    output.push('\0');
    output.as_ptr() as *const c_char
}
