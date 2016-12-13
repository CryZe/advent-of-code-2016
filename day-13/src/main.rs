#![no_main]
extern crate libc;
#[macro_use]
extern crate lazy_static;
extern crate smallvec;

use libc::c_char;
use std::ffi::CStr;
use std::str;
use std::sync::Mutex;
use std::fmt::Write;
use std::collections::BinaryHeap;
use std::cmp::{PartialOrd, Ord, Eq, PartialEq, Ordering};

use smallvec::SmallVec;

#[derive(Debug, Clone)]
struct Pos {
    x: usize,
    y: usize,
    steps: usize,
    min_steps: usize,
}

impl PartialEq for Pos {
    fn eq(&self, other: &Pos) -> bool {
        other.min_steps.eq(&self.min_steps)
    }
}

impl Eq for Pos {}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Pos) -> Option<Ordering> {
        other.min_steps.partial_cmp(&self.min_steps)
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Pos) -> Ordering {
        other.min_steps.cmp(&self.min_steps)
    }
}

impl Pos {
    fn new() -> Self {
        let mut pos = Pos {
            x: 1,
            y: 1,
            steps: 0,
            min_steps: 0,
        };
        pos.calculate_min_steps();
        pos
    }

    fn calculate_min_steps(&mut self) {
        let (x, y) = (self.x, self.y);
        let dx = if x >= 31 { x - 31 } else { 31 - x };
        let dy = if y >= 39 { y - 39 } else { 39 - y };
        self.min_steps = dx + dy + self.steps;
    }

    fn right(&self) -> Self {
        Pos {
            x: self.x + 1,
            steps: self.steps + 1,
            ..self.clone()
        }
    }

    fn down(&self) -> Self {
        Pos {
            y: self.y + 1,
            steps: self.steps + 1,
            ..self.clone()
        }
    }

    fn left(&self) -> Option<Self> {
        if self.x > 0 {
            let mut pos = Pos {
                x: self.x - 1,
                y: self.y,
                steps: self.steps + 1,
                min_steps: 0,
            };
            pos.calculate_min_steps();
            Some(pos)
        } else {
            None
        }
    }

    fn up(&self) -> Option<Self> {
        if self.y > 0 {
            let mut pos = Pos {
                x: self.x,
                y: self.y - 1,
                steps: self.steps + 1,
                min_steps: 0,
            };
            pos.calculate_min_steps();
            Some(pos)
        } else {
            None
        }
    }

    fn try_visit(&self, field: &mut Field, input: usize) -> bool {
        field.try_visit(self.x, self.y, input)
    }
}

struct Field {
    visited: SmallVec<[SmallVec<[bool; 32]>; 32]>,
}

impl Field {
    fn new() -> Self {
        Field { visited: Default::default() }
    }

    fn try_visit(&mut self, x: usize, y: usize, input: usize) -> bool {
        if y >= self.visited.len() {
            let elements = y + 1 - self.visited.len();
            self.visited.reserve(elements);
            for _ in 0..elements {
                self.visited.push(Default::default());
            }
        }
        let mut row = &mut self.visited[y];
        if x >= row.len() {
            let elements = x + 1 - row.len();
            row.reserve(elements);
            for _ in 0..elements {
                row.push(true);
            }
        }
        let mut cell = &mut row[x];
        if *cell {
            *cell = false;
            let val = x * x + 3 * x + 2 * x * y + y + y * y + input;
            val.count_ones() & 1 == 0
        } else {
            false
        }
    }
}

// fn draw_field<W>(mut write: W, field: &Field, player: &Pos)
//     where W: Write
// {
//     for (y, row) in field.visited.iter().enumerate() {
//         for (x, &state) in row.iter().enumerate() {
//             if player.x == x && player.y == y {
//                 write!(write, "O").unwrap();
//             } else if state {
//                 write!(write, ".").unwrap();
//             } else {
//                 write!(write, "#").unwrap();
//             }
//         }
//         writeln!(write, "").unwrap();
//     }
// }

fn traverse<F, E, R>(input: usize, mut filter: F, mut end: E) -> Result<R, &'static str>
    where F: FnMut(&Pos) -> bool,
          E: FnMut(&Pos) -> Option<R>
{
    let mut field = &mut Field::new();
    let mut heap = BinaryHeap::new();
    field.try_visit(1, 1, input);
    heap.push(Pos::new());

    while let Some(pos) = heap.pop() {
        if let Some(result) = end(&pos) {
            return Ok(result);
        }
        if !filter(&pos) {
            continue;
        }

        let right = pos.right();
        if right.try_visit(field, input) {
            heap.push(right);
        }

        let down = pos.down();
        if down.try_visit(field, input) {
            heap.push(down);
        }

        if let Some(left) = pos.left() {
            if left.try_visit(field, input) {
                heap.push(left);
            }
        }

        if let Some(up) = pos.up() {
            if up.try_visit(field, input) {
                heap.push(up);
            }
        }
    }

    Err("Can't finish")
}

fn traverse_part1(input: usize) -> Result<usize, &'static str> {
    traverse(input, |_| true, |pos| if pos.x == 31 && pos.y == 39 {
        Some(pos.steps)
    } else {
        None
    })
}

fn traverse_part2(input: usize) -> usize {
    let mut count = 0;
    traverse(input, |pos| pos.steps < 50, |_| {
            count += 1;
            None::<()>
        })
        .ok();
    count
}

lazy_static! {
    static ref OUTPUT: Mutex<String> = Mutex::new(String::new());
}

#[no_mangle]
pub unsafe extern "C" fn part1(text: *const c_char) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();
    output.clear();

    match text.parse().map_err(|_| "Can't parse input").and_then(traverse_part1) {
        Ok(steps) => writeln!(output, "Steps: {}", steps).unwrap(),
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

    match text.parse().map_err(|_| "Can't parse input").map(traverse_part2) {
        Ok(count) => writeln!(output, "Count: {}", count).unwrap(),
        Err(e) => writeln!(output, "Error: {}", e).unwrap(),
    }

    output.push('\0');
    output.as_ptr() as *const c_char
}
