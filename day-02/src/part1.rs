use Dir;
use Dir::*;

fn shift(pos: u8, dir: Dir) -> u8 {
    match dir {
        Up if pos > 3 => pos - 3,
        Down if pos < 7 => pos + 3,
        Left if (pos - 1) % 3 != 0 => pos - 1,
        Right if pos % 3 != 0 => pos + 1,
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
                result = 10 * result + pos as u32;
                line_has_char = false;
                continue;
            }
            _ => continue,
        };
        line_has_char = true;
        pos = shift(pos, dir);
    }
    if line_has_char {
        result = 10 * result + pos as u32;
    }
    result
}
