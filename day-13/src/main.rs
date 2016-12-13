#![no_main]
extern crate libc;
#[macro_use]
extern crate lazy_static;
extern crate smallvec;
extern crate base64;
extern crate image;
extern crate gif;

use libc::c_char;
use std::ffi::CStr;
use std::str;
use std::sync::Mutex;
use std::fmt::Write;
use std::collections::BinaryHeap;
use std::cmp::{PartialOrd, Ord, Eq, PartialEq, Ordering};
use image::{RgbImage, Rgb};
use gif::{Encoder, Frame};

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
        let mut pos = Pos {
            x: self.x + 1,
            y: self.y,
            steps: self.steps + 1,
            min_steps: 0,
        };
        pos.calculate_min_steps();
        pos
    }

    fn down(&self) -> Self {
        let mut pos = Pos {
            x: self.x,
            y: self.y + 1,
            steps: self.steps + 1,
            min_steps: 0,
        };
        pos.calculate_min_steps();
        pos
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

    fn try_visit<F>(&self, field: &mut Field<F>, input: usize) -> bool
        where F: FnMut(usize, usize, usize) -> bool
    {
        field.try_visit(self.x, self.y, input)
    }
}

struct Field<F>
    where F: FnMut(usize, usize, usize) -> bool
{
    visited: Vec<SmallVec<[bool; 32]>>,
    is_empty: F,
}

impl<F> Field<F>
    where F: FnMut(usize, usize, usize) -> bool
{
    fn new(is_empty: F) -> Self {
        Field {
            visited: Vec::with_capacity(45),
            is_empty: is_empty,
        }
    }

    fn try_visit(&mut self, x: usize, y: usize, input: usize) -> bool {
        if y >= self.visited.len() {
            self.visited.resize(y + 1, Default::default());
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
            (self.is_empty)(x, y, input)
        } else {
            false
        }
    }
}

fn is_empty(x: usize, y: usize, input: usize) -> bool {
    let val = x * x + 3 * x + 2 * x * y + y + y * y + input;
    val.count_ones() & 1 == 0
}

fn is_empty_draw(x: usize, y: usize, input: usize) -> bool {
    let val = x * x + 3 * x + 2 * x * y + y + y * y + input + 2;
    (val.count_ones() % 5) % 2 == 0
}

fn traverse<I, F, E, R>(input: usize,
                        is_empty: I,
                        mut filter: F,
                        mut end: E)
                        -> Result<R, &'static str>
    where F: FnMut(&Pos) -> bool,
          E: FnMut(&Pos) -> Option<R>,
          I: FnMut(usize, usize, usize) -> bool
{
    let mut field = &mut Field::new(is_empty);
    let mut heap = BinaryHeap::with_capacity(32);
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
    traverse(input,
             is_empty,
             |_| true,
             |pos| if pos.x == 31 && pos.y == 39 {
                 Some(pos.steps)
             } else {
                 None
             })
}

fn traverse_part2(input: usize) -> usize {
    let mut count = 0;
    traverse(input, is_empty, |pos| pos.steps < 50, |_| {
            count += 1;
            None::<()>
        })
        .ok();
    count
}

fn gif_to_url(buf: &[u8], out: &mut String) {
    let text = base64::encode(buf);
    write!(out, "data:image/gif;base64,{}", text).unwrap()
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

#[no_mangle]
pub unsafe extern "C" fn draw_original(text: *const c_char) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();
    output.clear();

    if let Ok(input) = text.parse() {
        if is_empty(30, 29, input) {
            let nx: u16 = 50;
            let ny = nx;

            let mut image = RgbImage::new(nx as u32, ny as u32);

            for (x, y, pixel) in image.enumerate_pixels_mut() {
                let value = if is_empty(x as usize, y as usize, input) {
                    0xFF
                } else {
                    0x00
                };
                pixel.data = [value, value, value];
            }

            image.put_pixel(31, 39, Rgb { data: [0x00, 0x0, 0xFF] });
            image.put_pixel(1, 1, Rgb { data: [0x00, 0x0, 0xFF] });

            let mut data = Vec::new();
            {
                let mut encoder = Encoder::new(&mut data, nx, ny, &[]).unwrap();
                let mut frame = Frame::from_rgb(nx, ny, &image);
                frame.delay = 50;
                encoder.write_frame(&frame).unwrap();

                traverse(input, is_empty, |_| true, |pos| {
                        let x = pos.x as u32;
                        let y = pos.y as u32;
                        if x < nx as u32 && y < ny as u32 {
                            image.put_pixel(x,
                                            y,
                                            Rgb {
                                                data: [0,
                                                       if pos.steps < 256 {
                                                           pos.steps as u8
                                                       } else {
                                                           255
                                                       },
                                                       255u8.saturating_sub(pos.steps as u8)],
                                            });
                            let frame = Frame::from_rgb(nx, ny, &image);
                            encoder.write_frame(&frame).unwrap();
                        }

                        if pos.min_steps > 300 || (pos.x == 31 && pos.y == 39) {
                            Some(())
                        } else {
                            None
                        }
                    })
                    .ok();
            }

            gif_to_url(&data, &mut output);
        } else {
            output.push_str("Error: Seed produces a wall at the target location.");
        }
    } else {
        output.push_str("Error: Can't parse the input as a number.");
    }

    output.push('\0');
    output.as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn draw_modified(text: *const c_char) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();
    output.clear();

    if let Ok(input) = text.parse() {
        if is_empty_draw(30, 29, input) {
            let nx: u16 = 70;
            let ny = nx;

            let mut image = RgbImage::new(nx as u32, ny as u32);

            for (x, y, pixel) in image.enumerate_pixels_mut() {
                let value = if is_empty_draw(x as usize, y as usize, input) {
                    0xFF
                } else {
                    0x00
                };
                pixel.data = [value, value, value];
            }

            image.put_pixel(30, 29, Rgb { data: [0x00, 0x0, 0xFF] });
            image.put_pixel(1, 1, Rgb { data: [0x00, 0x0, 0xFF] });

            let mut data = Vec::new();
            {
                let mut encoder = Encoder::new(&mut data, nx, ny, &[]).unwrap();
                let mut frame = Frame::from_rgb(nx, ny, &image);
                frame.delay = 50;
                encoder.write_frame(&frame).unwrap();

                let mut i = 0;

                traverse(input, is_empty_draw, |_| true, |pos| {
                        let x = pos.x as u32;
                        let y = pos.y as u32;
                        if x < nx as u32 && y < ny as u32 {
                            image.put_pixel(x,
                                            y,
                                            Rgb {
                                                data: [0,
                                                       (18 * pos.steps / 10) as u8,
                                                       255 - (18 * pos.steps / 10) as u8],
                                            });
                            if i % 10 == 0 {
                                let frame = Frame::from_rgb(nx, ny, &image);
                                encoder.write_frame(&frame).unwrap();
                            }
                            i += 1;
                        }

                        if pos.min_steps > 150 || (pos.x == 30 && pos.y == 29) {
                            let frame = Frame::from_rgb(nx, ny, &image);
                            encoder.write_frame(&frame).unwrap();
                            Some(())
                        } else {
                            None
                        }
                    })
                    .ok();
            }

            gif_to_url(&data, &mut output);
        } else {
            output.push_str("Error: Seed produces a wall at the target location.");
        }
    } else {
        output.push_str("Error: Can't parse the input as a number.");
    }

    output.push('\0');
    output.as_ptr() as *const c_char
}
