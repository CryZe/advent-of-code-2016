#![feature(conservative_impl_trait)]
#![no_main]

extern crate libc;
#[macro_use]
extern crate lazy_static;
extern crate odds;
extern crate arrayvec;

mod grammar;

use libc::c_char;
use std::ffi::CStr;
use std::str;
use std::sync::Mutex;
use std::fmt::Write;
use arrayvec::ArrayString;
use odds::prelude::*;
use odds::slice::rotate_left;

type Result<T> = ::std::result::Result<T, &'static str>;

#[derive(Debug)]
pub enum Operation {
    SwapPosition(usize, usize),
    SwapLetter(u8, u8),
    RotateLeft(usize),
    RotateRight(usize),
    RotatePosition(u8),
    ReversePositions(usize, usize),
    Move(usize, usize),
}

use self::Operation::*;

fn rotate_right<T>(slice: &mut [T], count: usize) {
    let count = slice.len() - (count % slice.len());
    rotate_left(slice, count);
}

fn forward(buf: &mut [u8; 8], operation: Operation) -> Result<()> {
    match operation {
        SwapPosition(x, y) => {
            if x >= buf.len() || y >= buf.len() {
                return Err("Swap position out of bounds");
            }
            buf.swap(x, y);
        }
        SwapLetter(x, y) => {
            let x = buf.find(&x).ok_or("Couldn't find letter x to swap")?;
            let y = buf.find(&y).ok_or("Couldn't find letter y to swap")?;
            buf.swap(x, y);
        }
        RotateLeft(x) => rotate_left(buf, x),
        RotateRight(x) => rotate_right(buf, x),
        RotatePosition(x) => {
            let x = buf.find(&x).ok_or("Couldn't find letter to rotate")?;
            rotate_right(buf, x + if x >= 4 { 2 } else { 1 });
        }
        ReversePositions(x, y) => {
            if x >= buf.len() || y >= buf.len() {
                return Err("Reverse position out of bounds");
            }
            buf[x..y + 1].reverse();
        }
        Move(x, y) => {
            if x < y {
                if y >= buf.len() {
                    return Err("Move position out of bounds");
                }
                rotate_left(&mut buf[x..y + 1], 1);
            } else if x > y {
                if x >= buf.len() {
                    return Err("Move position out of bounds");
                }
                rotate_right(&mut buf[y..x + 1], 1);
            }
        }
    }

    Ok(())
}

fn backward(buf: &mut [u8; 8], operation: Operation) -> Result<()> {
    let operation = match operation {
        RotateLeft(x) => RotateRight(x),
        RotateRight(x) => RotateLeft(x),
        RotatePosition(x) => {
            let x = buf.find(&x).ok_or("Couldn't find letter to rotate")?;
            let x = [1, 1, 6, 2, 7, 3, 0, 4][x];
            rotate_left(buf, x);
            return Ok(());
        }
        Move(x, y) => Move(y, x),
        op => op,
    };
    forward(buf, operation)
}

fn parse_operations<'a>(text: &'a str) -> impl Iterator<Item = Result<Operation>> + 'a {
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(grammar::parse_operation)
        .map(|o| o.map_err(|_| "Couldn't parse operation"))
}

fn execute<I, F>(operations: I, mut buf: [u8; 8], mut execute_op: F) -> Result<ArrayString<[u8; 8]>>
    where I: IntoIterator<Item = Result<Operation>>,
          F: FnMut(&mut [u8; 8], Operation) -> Result<()>
{
    for operation in operations {
        execute_op(&mut buf, operation?)?;
    }

    Ok(ArrayString::from(unsafe { str::from_utf8_unchecked(&buf) }).unwrap())
}

lazy_static! {
    static ref OUTPUT: Mutex<String> = Mutex::new(String::new());
}

#[no_mangle]
pub unsafe extern "C" fn part1(text: *const c_char) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();
    output.clear();

    match execute(parse_operations(text), *b"abcdefgh", forward) {
        Ok(password) => writeln!(output, "Password: {}", password).unwrap(),
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

    match execute(parse_operations(text).collect::<Vec<_>>().into_iter().rev(),
                  *b"fbgdceah",
                  backward) {
        Ok(password) => writeln!(output, "Unscrambled Password: {}", password).unwrap(),
        Err(e) => writeln!(output, "Error: {}", e).unwrap(),
    }

    output.push('\0');
    output.as_ptr() as *const c_char
}
