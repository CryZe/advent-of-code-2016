#![no_main]
extern crate libc;
extern crate arrayvec;

use libc::c_char;
use std::ffi::CStr;
use arrayvec::ArrayVec;
use std::borrow::Cow;

struct Room<'a> {
    name: &'a str,
    id: u64,
    checksum: &'a str,
}

fn parse_room(room: &str) -> Result<Room, ()> {
    let room = room.trim();

    let index = room.find(|c| char::is_digit(c, 10)).ok_or(())?;
    let (name, rest) = room.split_at(index);

    let dash_count = name.chars().rev().take_while(|&c| c == '-').count();
    let name = &name[..name.len() - dash_count];

    let index = rest.find(|c| c == '[').ok_or(())?;
    let (id, rest) = rest.split_at(index);
    let id = id.parse().map_err(|_| ())?;

    if rest.len() < 2 {
        return Err(());
    }
    let checksum = &rest[1..rest.len() - 1];

    Ok(Room {
        name: name,
        id: id,
        checksum: checksum,
    })
}

fn decipher<'a>(room: &'a Room) -> Cow<'a, str> {
    let shift = (room.id % 26) as u8;
    if shift == 0 {
        room.name.into()
    } else {
        room.name
            .chars()
            .map(|c| match c {
                'a'...'z' => ((((c as u8) - b'a' + shift) % 26) + b'a') as char,
                '-' => ' ',
                c => c,
            })
            .collect::<String>()
            .into()
    }
}

fn is_real(room: &Room) -> bool {
    let mut letters = (b'a'..b'z' + 1).map(|c| (0, c)).collect::<ArrayVec<[_; 26]>>();
    for letter in room.name.bytes() {
        if let Some(&mut (ref mut count, _)) =
               letters.get_mut((letter.wrapping_sub(b'a')) as usize) {
            *count += 1;
        }
    }
    letters.sort_by_key(|&(count, _)| -(count as isize));
    let letters = letters[..5].iter().map(|&(_, letter)| letter).collect::<ArrayVec<[_; 5]>>();

    room.checksum.bytes().all(|l| letters.contains(&l))
}

fn sum_ids(text: &str) -> u64 {
    text.lines()
        .map(parse_room)
        .filter_map(Result::ok)
        .filter(is_real)
        .map(|r| r.id)
        .sum()
}

fn find_id_of_northpole_object_storage(text: &str) -> Option<u64> {
    text.lines()
        .map(parse_room)
        .filter_map(Result::ok)
        .filter(|r| decipher(r) == "northpole object storage")
        .map(|r| r.id)
        .next()
}

#[no_mangle]
pub unsafe extern "C" fn part1(text: *const c_char) -> usize {
    let text = CStr::from_ptr(text).to_str().unwrap();
    sum_ids(text) as usize
}

#[no_mangle]
pub unsafe extern "C" fn part2(text: *const c_char) -> usize {
    let text = CStr::from_ptr(text).to_str().unwrap();
    find_id_of_northpole_object_storage(text).unwrap_or(!0) as usize
}
