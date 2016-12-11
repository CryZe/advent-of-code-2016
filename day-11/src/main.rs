// #![no_main]
extern crate libc;
#[macro_use]
extern crate lazy_static;
extern crate arrayvec;
extern crate seahash;

use libc::c_char;
use std::ffi::CStr;
use std::str;
use std::sync::Mutex;
use std::fmt::Write;
use arrayvec::ArrayVec;
use std::cmp::{PartialEq, Eq, min};
use std::collections::HashSet;
use seahash::SeaHasher;
use std::hash::{Hash, Hasher, BuildHasherDefault};

type Error = &'static str;
type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct InternedStr(u8);

struct StrPool<'a> {
    elements: ArrayVec<[&'a str; 14]>,
}

impl<'a> StrPool<'a> {
    fn new() -> Self {
        StrPool { elements: Default::default() }
    }

    fn add(&mut self, slice: &'a str) -> Result<InternedStr> {
        if let Some((index, _)) = self.elements.iter().enumerate().find(|&(_, &s)| s == slice) {
            Ok(InternedStr(index as u8))
        } else {
            let len = self.elements.len();
            if len < 14 {
                self.elements.push(slice);
                Ok(InternedStr(len as u8))
            } else {
                Err("Too many elements")
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
enum Component {
    Microchip(InternedStr),
    Generator(InternedStr),
}

impl Component {
    fn kind(&self) -> u8 {
        match *self {
            Microchip(_) => 0,
            Generator(_) => 1,
        }
    }
}

use self::Component::*;

#[derive(Debug)]
struct Facility {
    elevator: usize,
    steps: usize,
    floors: [ArrayVec<[Component; 14]>; 4],
}

impl PartialEq for Facility {
    fn eq(&self, other: &Self) -> bool {
        if self.elevator != other.elevator || self.steps != other.steps {
            return false;
        }
        for (mine, other) in self.floors.iter().zip(other.floors.iter()) {
            if !mine.iter().map(|c| c.kind()).eq(other.iter().map(|c| c.kind())) {
                return false;
            }
        }
        true
    }
}

impl Eq for Facility {}

impl Hash for Facility {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.elevator);
        state.write_usize(self.steps);
        for floor in &self.floors {
            state.write_usize(floor.len());
            for component in floor {
                state.write_u8(component.kind());
            }
        }
    }
}

impl Clone for Facility {
    fn clone(&self) -> Self {
        Facility {
            elevator: self.elevator,
            steps: self.steps,
            floors: [self.floors[0].clone(),
                     self.floors[1].clone(),
                     self.floors[2].clone(),
                     self.floors[3].clone()],
        }
    }
}

impl Facility {
    fn min_steps_to_complete(&self) -> usize {
        let mut floor_count = 0;
        let mut steps = self.steps;

        for floor in &self.floors[..3] {
            floor_count += floor.len();
            steps += floor_count >> 1;
        }

        steps += self.floors[..3]
            .iter()
            .enumerate()
            .find(|&(_, f)| !f.is_empty())
            .map_or(0, |(i, _)| self.elevator.saturating_sub(i));

        steps
    }

    fn with(&self, source_floor: usize, target_floor: usize, component_index: usize) -> Self {
        let mut facility = self.clone();

        let component = facility.floors[source_floor].remove(component_index).unwrap();
        facility.floors[target_floor].push(component);
        facility.floors[target_floor].sort();

        facility
    }

    fn is_done(&self) -> bool {
        self.floors.iter().take(3).all(|f| f.is_empty())
    }

    fn is_valid(&self) -> bool {
        for floor in &self.floors {
            let (mut has_generators, mut disconnected_microchips) = (false, false);
            for component in floor {
                match *component {
                    Microchip(element) => {
                        if !floor.contains(&Generator(element)) {
                            disconnected_microchips = true;
                        }
                    }
                    Generator(_) => {
                        has_generators = true;
                    }
                }
                if disconnected_microchips && has_generators {
                    return false;
                }
            }
        }
        true
    }
}

struct Heap {
    min: usize,
    sets: Vec<HashSet<Facility, BuildHasherDefault<SeaHasher>>>,
}

impl Heap {
    fn new() -> Self {
        Heap {
            min: 0,
            sets: Default::default(),
        }
    }

    fn push(&mut self, facility: Facility) {
        let index = facility.min_steps_to_complete();
        if index >= self.sets.len() {
            self.sets.resize(index + 1, Default::default());
        }
        self.sets[index].insert(facility);
    }

    fn pop(&mut self) -> Option<Facility> {
        loop {
            if self.min >= self.sets.len() {
                return None;
            }
            if let Some(element) = self.sets[self.min].iter().cloned().next() {
                self.sets[self.min].remove(&element);
                return Some(element);
            } else {
                self.sets[self.min] = Default::default();
                self.min += 1;
            }
        }
    }
}

fn parse_floor<'a>(pool: &mut StrPool<'a>, text: &'a str) -> Result<ArrayVec<[Component; 14]>> {
    let mut splits = text.split("floor contains");
    splits.next().ok_or("Expected 'The _ floor contains'")?;
    let component_list = splits.next().ok_or("Expected Component list")?.trim();
    if component_list == "nothing relevant." {
        Ok(ArrayVec::new())
    } else {
        let mut components = ArrayVec::<[_; 14]>::new();
        for split in component_list.split("a ").skip(1) {
            let component = if split.ends_with(", ") {
                &split[..split.len() - 2]
            } else if split.ends_with(", and ") {
                &split[..split.len() - 6]
            } else if split.ends_with(" and ") {
                &split[..split.len() - 5]
            } else if split.ends_with('.') {
                &split[..split.len() - 1]
            } else {
                return Err("Component list is not properly enumerated");
            };

            let component = if component.ends_with(" generator") {
                Generator(pool.add(&component[..component.len() - 10])?)
            } else if component.ends_with("-compatible microchip") {
                Microchip(pool.add(&component[..component.len() - 21])?)
            } else {
                return Err("Component is neither a generator nor a microchip");
            };

            components.push(component);
        }
        components.sort();
        Ok(components)
    }
}

fn parse_facility<'a>(pool: &mut StrPool<'a>, text: &'a str) -> Result<Facility> {
    let mut lines = text.lines();

    let first = parse_floor(pool, lines.next().ok_or("Expected first floor")?)?;
    let second = parse_floor(pool, lines.next().ok_or("Expected second floor")?)?;
    let third = parse_floor(pool, lines.next().ok_or("Expected third floor")?)?;
    let fourth = parse_floor(pool, lines.next().ok_or("Expected fourth floor")?)?;

    let facility = Facility {
        elevator: 0,
        steps: 0,
        floors: [first, second, third, fourth],
    };

    if facility.is_valid() {
        Ok(facility)
    } else {
        Err("Invalid initial facility")
    }
}

fn add_part2_elements(pool: &mut StrPool, mut facility: Facility) -> Result<Facility> {
    let elerium = pool.add("elerium")?;
    let dilithium = pool.add("dilithium")?;

    facility.floors[0].push(Generator(elerium));
    facility.floors[0].push(Microchip(elerium));
    facility.floors[0].push(Generator(dilithium));
    facility.floors[0].push(Microchip(dilithium));

    facility.floors[0].sort();

    Ok(facility)
}

fn find_min_steps_to_complete(facility: Facility) -> Result<usize> {
    let mut heap = Heap::new();
    heap.push(facility);

    while let Some(facility) = heap.pop() {
        if facility.is_done() {
            return Ok(facility.steps);
        }

        let current_floor = facility.elevator;
        let original_facility = facility;
        for target_floor in current_floor.saturating_sub(1)..min(current_floor + 2, 4) {
            if target_floor != current_floor {
                for index in 0..original_facility.floors[current_floor].len() {
                    let mut facility = original_facility.with(current_floor, target_floor, index);
                    facility.elevator = target_floor;
                    facility.steps += 1;

                    for index in index..facility.floors[current_floor].len() {
                        let facility = facility.with(current_floor, target_floor, index);
                        if facility.is_valid() {
                            heap.push(facility);
                        }
                    }

                    if facility.is_valid() {
                        heap.push(facility);
                    }
                }
            }
        }
    }

    Err("Can't finish this facility")
}

fn main() {
    let text = "The first floor contains a polonium generator, a thulium generator, a \
                thulium-compatible microchip, a promethium generator, a ruthenium generator, a \
                ruthenium-compatible microchip, a cobalt generator, and a cobalt-compatible \
                microchip.
The second floor contains a polonium-compatible microchip and a \
                promethium-compatible microchip.
The third floor contains nothing relevant.
The \
                fourth floor contains nothing relevant.";

    let mut pool = StrPool::new();
    let result = parse_facility(&mut pool, text)
        .and_then(|f| add_part2_elements(&mut pool, f))
        .and_then(find_min_steps_to_complete);
    match result {
        Ok(steps) => println!("Steps to complete: {}", steps),
        Err(e) => println!("{}", e),
    }
}

lazy_static! {
    static ref OUTPUT: Mutex<String> = Mutex::new(String::new());
}

#[no_mangle]
pub unsafe extern "C" fn part1(text: *const c_char) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();
    output.clear();

    let mut pool = StrPool::new();
    let result = parse_facility(&mut pool, text).and_then(find_min_steps_to_complete);
    match result {
        Ok(steps) => writeln!(output, "Steps to complete: {}", steps).unwrap(),
        Err(e) => output.push_str(e),
    }

    output.push('\0');
    output.as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn part2(text: *const c_char) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();
    output.clear();

    let mut pool = StrPool::new();
    let result = parse_facility(&mut pool, text)
        .and_then(|f| add_part2_elements(&mut pool, f))
        .and_then(find_min_steps_to_complete);
    match result {
        Ok(steps) => writeln!(output, "Steps to complete: {}", steps).unwrap(),
        Err(e) => output.push_str(e),
    }

    output.push('\0');
    output.as_ptr() as *const c_char
}
