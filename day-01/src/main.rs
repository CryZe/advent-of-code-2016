#![no_main]
extern crate libc;

use libc::c_char;
use std::ffi::CStr;
use std::ops::AddAssign;
use std::io::{self, Write};
use std::collections::HashSet;

mod grammar;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Vec2(i64, i64);

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl Vec2 {
    fn dist(&self) -> u64 {
        let &Vec2(x, y) = self;
        x.abs() as u64 + y.abs() as u64
    }
}

#[derive(Copy, Clone)]
pub enum Turn {
    Left(u64),
    Right(u64),
}

#[derive(Copy, Clone)]
enum Dir {
    North,
    East,
    South,
    West,
}

use self::Turn::*;
use self::Dir::*;

impl Dir {
    fn turn_right(&mut self) {
        *self = match *self {
            North => East,
            East => South,
            South => West,
            West => North,
        };
    }

    fn turn_left(&mut self) {
        *self = match *self {
            East => North,
            South => East,
            West => South,
            North => West,
        };
    }

    fn to_vec(self, distance: u64) -> Vec2 {
        let d = distance as i64;
        match self {
            North => Vec2(0, d),
            East => Vec2(d, 0),
            South => Vec2(0, -d),
            West => Vec2(-d, 0),
        }
    }
}

struct Santa {
    coord: Vec2,
    dir: Dir,
    history: HashSet<Vec2>,
}

impl Santa {
    fn new() -> Self {
        Santa {
            coord: Vec2(0, 0),
            dir: North,
            history: HashSet::new(),
        }
    }

    fn execute(&mut self, turn: Turn, check_history: bool) -> bool {
        let distance = match turn {
            Left(x) => {
                self.dir.turn_left();
                x
            }
            Right(x) => {
                self.dir.turn_right();
                x
            }
        };
        if check_history {
            let vec = self.dir.to_vec(1);
            for _ in 0..distance {
                if !self.history.insert(self.coord) {
                    return false;
                }
                self.coord += vec;
            }
        } else {
            self.coord += self.dir.to_vec(distance);
        }
        true
    }

    fn execute_all(&mut self, turns: &[Turn], check_history: bool) {
        for &turn in turns {
            if !self.execute(turn, check_history) {
                break;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn calculate(text: *const c_char, check_history: bool) -> u64 {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let result = grammar::parse_turns(text);
    match result {
        Ok(turns) => {
            let mut santa = Santa::new();
            santa.execute_all(&turns, check_history);
            santa.coord.dist()
        }
        Err(e) => {
            writeln!(io::stderr(), "Error: {:?}", e).unwrap();
            !0
        }
    }
}
