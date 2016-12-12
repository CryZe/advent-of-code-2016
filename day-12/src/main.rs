#![no_main]
extern crate libc;
#[macro_use]
extern crate lazy_static;

use libc::c_char;
use std::ffi::CStr;
use std::str;
use std::sync::Mutex;
use std::fmt::Write;

mod grammar;

pub type Register = usize;
pub type Cpu = [RegSize; 4];
pub type RegSize = i32;

#[derive(Copy, Clone)]
pub enum Value {
    Immediate(RegSize),
    Reg(Register),
}

pub enum Instruction {
    Cpy(Value, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Value, Value),
}

use self::Value::*;
use self::Instruction::*;

fn cpu_part1() -> Cpu {
    [0; 4]
}

fn cpu_part2() -> Cpu {
    [0, 0, 1, 0]
}

fn evaluate(cpu: &Cpu, v: Value) -> RegSize {
    match v {
        Immediate(v) => v,
        Reg(r) => cpu[r],
    }
}

fn execute(cpu: &mut Cpu, code: &[Instruction]) {
    let mut instruction_ptr = code.iter().enumerate().skip(0);

    while let Some((i, instruction)) = instruction_ptr.next() {
        match *instruction {
            Cpy(src, dst) => {
                let src = evaluate(cpu, src);
                cpu[dst] = src;
            }
            Inc(r) => cpu[r] = cpu[r].wrapping_add(1),
            Dec(r) => cpu[r] = cpu[r].wrapping_sub(1),
            Jnz(cond, offset) => {
                let cond = evaluate(cpu, cond);
                if cond != 0 {
                    let offset = evaluate(cpu, offset);
                    let i = i as isize + offset as isize;
                    if i < 0 || i as usize >= code.len() {
                        break;
                    } else {
                        instruction_ptr = code.iter().enumerate().skip(i as usize);
                    }
                }
            }
        }
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

    let mut cpu = cpu_part1();
    let result = grammar::parse_instructions(text).map(|c| execute(&mut cpu, &c));

    match result {
        Ok(()) => writeln!(output, "CPU: {:?}", cpu).unwrap(),
        Err(e) => writeln!(output, "Error: {:?}", e).unwrap(),
    }

    output.push('\0');
    output.as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn part2(text: *const c_char) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();
    output.clear();

    let mut cpu = cpu_part2();
    let result = grammar::parse_instructions(text).map(|c| execute(&mut cpu, &c));

    match result {
        Ok(()) => writeln!(output, "CPU: {:?}", cpu).unwrap(),
        Err(e) => writeln!(output, "Error: {:?}", e).unwrap(),
    }

    output.push('\0');
    output.as_ptr() as *const c_char
}
