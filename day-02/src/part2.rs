use Dir;
use Dir::*;

fn shift(pos: u8, dir: Dir) -> u8 {
    match (pos, dir) {
        (0x1, _) => if dir == Down { 3 } else { pos },
        (0x5, _) => if dir == Right { 6 } else { pos },
        (0x9, _) => if dir == Left { 8 } else { pos },
        (0xD, _) => if dir == Up { 0xB } else { pos },
        (0x3, Up) => 1,
        (0xB, Down) => 0xD,
        (_, Up) if pos > 4 => pos - 4,
        (_, Down) if pos < 0xA => pos + 4,
        (_, Left) if pos != 2 && pos != 0xA => pos - 1,
        (_, Right) if pos != 4 && pos != 0xC => pos + 1,
        _ => pos,
    }
}

pub fn shift_all(mut pos: u8, dirs: &str) -> u32 {
    let mut result = 0;
    let mut line_has_char = false;
    for c in dirs.bytes() {
        let dir = match c {
            b'U' => Up,
            b'D' => Down,
            b'L' => Left,
            b'R' => Right,
            b'\n' => {
                result = 0x10 * result + pos as u32;
                line_has_char = false;
                continue;
            }
            _ => continue,
        };
        line_has_char = true;
        pos = shift(pos, dir);
    }
    if line_has_char {
        result = 0x10 * result + pos as u32;
    }
    result
}
