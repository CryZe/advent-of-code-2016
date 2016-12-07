#![no_main]
extern crate libc;
extern crate smallvec;

use libc::c_char;
use std::ffi::CStr;
use std::str;
use std::slice::Windows;
use std::iter::Fuse;
use smallvec::SmallVec;

struct Iter<'a> {
    inner: Fuse<Windows<'a, u8>>,
    is_in_brackets: bool,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a [u8], bool);

    fn next(&mut self) -> Option<Self::Item> {
        let window = self.inner.next();
        if let Some(window) = window {
            self.is_in_brackets = if self.is_in_brackets {
                window[0] != b']'
            } else {
                window[0] == b'['
            };
            Some((window, self.is_in_brackets))
        } else {
            None
        }
    }
}

fn iter_ip(text: &str, window_len: usize) -> Iter {
    Iter {
        inner: text.as_bytes().windows(window_len).fuse(),
        is_in_brackets: false,
    }
}

fn supports_tls(ip: &str) -> bool {
    let mut found_abba = false;

    for (w, is_in_brackets) in iter_ip(ip, 4) {
        if w[0] == w[3] && w[1] == w[2] && w[0] != w[1] {
            if is_in_brackets {
                return false;
            } else {
                found_abba = true;
            }
        }
    }

    found_abba
}

fn supports_ssl(ip: &str) -> bool {
    let mut pairs = SmallVec::<[((u8, u8), [bool; 2]); 32]>::new();

    for (w, is_in_brackets) in iter_ip(ip, 3) {
        if w[0] == w[2] && w[0] != w[1] {
            let (key, index) = if is_in_brackets {
                ((w[0], w[1]), 0)
            } else {
                ((w[1], w[0]), 1)
            };

            let mut found = false;
            for &mut (k, ref mut flags) in &mut pairs {
                if k == key {
                    flags[index] = true;
                    if flags[0] && flags[1] {
                        return true;
                    }
                    found = true;
                    break;
                }
            }
            if !found {
                let flags = if index == 0 {
                    [true, false]
                } else {
                    [false, true]
                };
                pairs.push((key, flags));
            }
        }
    }

    false
}

fn count_tls_support(text: &str) -> usize {
    text.lines().map(|l| l.trim()).filter(|ip| supports_tls(ip)).count()
}

fn count_ssl_support(text: &str) -> usize {
    text.lines().map(|l| l.trim()).filter(|ip| supports_ssl(ip)).count()
}

#[no_mangle]
pub unsafe extern "C" fn part1(text: *const c_char) -> usize {
    let text = CStr::from_ptr(text).to_str().unwrap();

    count_tls_support(text)
}

#[no_mangle]
pub unsafe extern "C" fn part2(text: *const c_char) -> usize {
    let text = CStr::from_ptr(text).to_str().unwrap();

    count_ssl_support(text)
}
