#![no_main]
extern crate libc;
#[macro_use]
extern crate lazy_static;
extern crate image;
extern crate base64;

use libc::c_char;
use std::ffi::CStr;
use std::str::{self, Lines};
use std::sync::Mutex;
use std::fmt::{self, Write};
use std::mem::uninitialized;
use std::ops::Deref;
use image::{RgbaImage, GenericImage, Pixel};
use image::png::PNGEncoder;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

use self::Instruction::*;

fn parse_instruction(text: &str) -> Result<Instruction, &'static str> {
    let mut splits = text.split_whitespace();
    let typ = splits.next().ok_or("Type missing")?;
    if typ == "rect" {
        let arg = splits.next().ok_or("Rect argument missing")?;
        let mut splits = arg.split('x');
        let w = splits.next()
            .ok_or("Rect width missing")?
            .parse()
            .map_err(|_| "Can't parse Rect width")?;
        let h = splits.next()
            .ok_or("Rect height missing")?
            .parse()
            .map_err(|_| "Can't parse Rect height")?;
        if w > WIDTH {
            Err("Width out of bounds")
        } else if h > HEIGHT {
            Err("Height out of bounds")
        } else {
            Ok(Rect(w, h))
        }
    } else if typ == "rotate" {
        let dir = splits.next().ok_or("Rotate direction missing")?;
        let pos = splits.next().ok_or("Rotate position missing")?;
        if pos.len() < 2 {
            return Err("Rotate position too short");
        }
        let pos = pos[2..].parse().map_err(|_| "Can't parse Rotate position")?;
        splits.next().ok_or("Rotate by keyword missing")?;
        let count = splits.next()
            .ok_or("Rotate count missing")?
            .parse()
            .map_err(|_| "Can't parse Rotate count")?;
        if dir == "row" {
            if pos < HEIGHT {
                Ok(RotateRow(pos, count))
            } else {
                Err("Position out of bounds")
            }
        } else if dir == "column" {
            if pos < WIDTH {
                Ok(RotateColumn(pos, count))
            } else {
                Err("Position out of bounds")
            }
        } else {
            Err("Unknown rotate direction")
        }
    } else {
        Err("Unknown type")
    }
}

struct Iter<'a>(Lines<'a>);

impl<'a> Iterator for Iter<'a> {
    type Item = Result<Instruction, &'static str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(str::trim).map(parse_instruction)
    }
}

fn parse(text: &str) -> Iter {
    Iter(text.lines())
}

struct Screen([[bool; WIDTH]; HEIGHT]);

impl Screen {
    fn new() -> Self {
        Screen([[false; WIDTH]; HEIGHT])
    }

    fn count_active_pixels(&self) -> usize {
        self.0.iter().map(|r| r.iter().filter(|&&x| x).count()).sum()
    }

    fn to_image(&self) -> RgbaImage {
        let mut image = RgbaImage::new(WIDTH as u32, HEIGHT as u32);

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let value = if self.0[y as usize][x as usize] {
                0xFF
            } else {
                0x00
            };
            pixel.data = [0x00, 0x00, 0x00, value as u8];
        }

        image
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Rect(w, h) => {
                for y in 0..h {
                    for x in 0..w {
                        self.0[y][x] = true;
                    }
                }
            }
            RotateRow(y, count) => {
                let mut tmp: [bool; WIDTH] = unsafe { uninitialized() };
                let count = count % WIDTH;
                let mut src_x = WIDTH - count;
                let row = &mut self.0[y];
                for dst in tmp.iter_mut() {
                    *dst = row[src_x];
                    if src_x == WIDTH - 1 {
                        src_x = 0;
                    } else {
                        src_x += 1;
                    }
                }
                row.copy_from_slice(&tmp);
            }
            RotateColumn(x, count) => {
                let mut tmp: [bool; HEIGHT] = unsafe { uninitialized() };
                let count = count % HEIGHT;
                let mut src_y = HEIGHT - count;
                for tmp in &mut tmp {
                    *tmp = self.0[src_y][x];
                    if src_y == HEIGHT - 1 {
                        src_y = 0;
                    } else {
                        src_y += 1;
                    }
                }
                for (y, &tmp) in (0..HEIGHT).zip(tmp.iter()) {
                    self.0[y][x] = tmp;
                }
            }
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.0 {
            for &column in row.iter() {
                if column {
                    write!(f, "█")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

fn image_to_url<I, P>(image: &I) -> String
    where I: GenericImage<Pixel = P> + Deref<Target = [u8]>,
          P: Pixel<Subpixel = u8>
{
    let mut buf = Vec::new();
    PNGEncoder::new(&mut buf)
        .encode(image, image.width(), image.height(), P::color_type())
        .unwrap();

    let text = base64::encode(&buf);
    format!("data:image/png;base64,{}", text)
}

fn execute(text: &str) -> Result<Screen, (usize, &'static str)> {
    let mut screen = Screen::new();

    for (line, instruction) in parse(text).enumerate() {
        let instruction = instruction.map_err(|e| (line + 1, e))?;
        screen.execute(instruction);
    }

    Ok(screen)
}

lazy_static! {
    static ref OUTPUT: Mutex<String> = Mutex::new(String::new());
}

#[no_mangle]
pub unsafe extern "C" fn part1(text: *const c_char) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();
    output.clear();

    match execute(text) {
        Ok(screen) => writeln!(output, "Active Pixels: {}", screen.count_active_pixels()).unwrap(),
        Err((l, e)) => writeln!(output, "Error in Line {}: {}", l, e).unwrap(),
    }

    output.push('\0');
    output.as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn part2(text: *const c_char) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();
    output.clear();

    if let Ok(screen) = execute(text) {
        let image = screen.to_image();
        *output = image_to_url(&image);
    }

    output.push('\0');
    output.as_ptr() as *const c_char
}
