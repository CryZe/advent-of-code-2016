#![no_main]
extern crate libc;
#[macro_use]
extern crate lazy_static;
extern crate arrayvec;

use libc::c_char;
use std::ffi::CStr;
use std::str;
use std::sync::Mutex;
use std::fmt::Write;
use arrayvec::ArrayVec;
use std::collections::VecDeque;
use std::cmp::max;
use std::cell::RefCell;

type Value = u32;
type Error = &'static str;

#[derive(Debug)]
enum Target {
    Bot(usize),
    Output(usize),
}

#[derive(Debug)]
struct SplitInstruction {
    low: Target,
    high: Target,
}

enum Instruction {
    Split(usize, SplitInstruction),
    Initial(Value, usize),
}

#[derive(Default, Clone, Debug)]
struct Bot<'a> {
    values: ArrayVec<[Value; 2]>,
    instruction: Option<&'a SplitInstruction>,
}

#[derive(Debug)]
struct Factory<'a> {
    bots: Vec<RefCell<Bot<'a>>>,
    outputs: Vec<Value>,
    active_bots: VecDeque<usize>,
}

fn parse_target(text: &str) -> Result<Target, Error> {
    let mut splits = text.split_whitespace();
    let typ = splits.next().ok_or("Target type missing")?;
    let id =
        splits.next().ok_or("Target id missing")?.parse().map_err(|_| "Can't parse target id")?;

    match typ {
        "bot" => Ok(Target::Bot(id)),
        "output" => Ok(Target::Output(id)),
        _ => Err("Unknown target type"),
    }
}

fn parse_instruction(text: &str) -> Result<Instruction, Error> {
    let keyword = text.split_whitespace().next().ok_or("Empty Instruction")?;
    let text = text[keyword.len()..].trim_left();

    match keyword {
        "value" => {
            let mut splits = text.split("goes to bot");
            let value = splits.next()
                .ok_or("Value missing")?
                .trim_right()
                .parse()
                .map_err(|_| "Can't parse value")?;
            let bot_id = splits.next()
                .ok_or("Bot id missing")?
                .trim_left()
                .parse()
                .map_err(|_| "Can't parse bot id")?;
            Ok(Instruction::Initial(value, bot_id))
        }
        "bot" => {
            let mut splits = text.split("gives low to");
            let bot_id = splits.next()
                .ok_or("Bot id missing")?
                .trim_right()
                .parse()
                .map_err(|_| "Can't parse bot id")?;
            let text = splits.next().ok_or("'gives low to' missing")?.trim_left();
            let mut splits = text.split("and high to");
            let low = splits.next().ok_or("Low target missing")?.trim_right();
            let low = parse_target(low)?;
            let high = splits.next().ok_or("High target missing")?.trim_left();
            let high = parse_target(high)?;
            Ok(Instruction::Split(bot_id,
                                  SplitInstruction {
                                      low: low,
                                      high: high,
                                  }))
        }
        _ => Err("Unknown Instruction type"),
    }
}

fn parse(text: &str) -> Result<Vec<(Instruction)>, Error> {
    text.lines().map(|l| parse_instruction(l.trim())).collect()
}

fn count_bots_and_outputs(instructions: &[Instruction]) -> (usize, usize) {
    let (mut bots, mut outputs) = (0, 0);
    for instruction in instructions {
        match *instruction {
            Instruction::Initial(_, id) => bots = max(id, bots),
            Instruction::Split(id, SplitInstruction { ref low, ref high }) => {
                bots = max(id, bots);
                for &target in &[low, high] {
                    match *target {
                        Target::Bot(id) => bots = max(id, bots),
                        Target::Output(id) => outputs = max(id, outputs),
                    }
                }
            }
        }
    }

    (bots + 1, outputs + 1)
}

fn make_factory(instructions: &[Instruction]) -> Result<Factory, Error> {
    let (bot_count, output_count) = count_bots_and_outputs(instructions);

    let mut bots = Vec::new();
    bots.resize(bot_count, RefCell::<Bot>::default());

    let mut outputs = Vec::new();
    outputs.resize(output_count, 0);

    let mut active_bots = VecDeque::new();

    for instruction in instructions {
        match *instruction {
            Instruction::Initial(value, bot_id) => {
                let mut bot = bots[bot_id].borrow_mut();
                let values = &mut bot.values;
                if values.len() < 2 {
                    values.push(value);
                } else {
                    return Err("Bot has more than 2 values initially");
                }
                if values.len() == 2 {
                    active_bots.push_back(bot_id);
                }
            }
            Instruction::Split(bot_id, ref instruction) => {
                let mut bot = bots[bot_id].borrow_mut();
                let slot = &mut bot.instruction;
                if slot.is_some() {
                    return Err("Bot already has an instruction");
                } else {
                    *slot = Some(instruction);
                }
            }
        }
    }

    Ok(Factory {
        bots: bots,
        outputs: outputs,
        active_bots: active_bots,
    })
}

fn execute_factory<F, T>(instructions: &[Instruction],
                         mut listener: F)
                         -> Result<Result<T, Factory>, Error>
    where F: FnMut(&Bot, usize) -> Option<T>
{
    let mut factory = make_factory(instructions)?;

    while let Some(bot_id) = factory.active_bots.pop_front() {
        let mut bot = factory.bots[bot_id].borrow_mut();
        if let Some(value) = listener(&bot, bot_id) {
            return Ok(Ok(value));
        }

        if bot.values.len() == 2 {
            let instruction = bot.instruction.ok_or("Bot has no instruction")?;
            let a = bot.values[0];
            let b = bot.values[1];
            let (high, low) = if a > b { (a, b) } else { (b, a) };
            bot.values.clear();

            for &(target, value) in &[(&instruction.low, low), (&instruction.high, high)] {
                match *target {
                    Target::Output(id) => {
                        factory.outputs[id] = value;
                    }
                    Target::Bot(id) => {
                        let mut other = factory.bots[id].try_borrow_mut()
                            .map_err(|_| "Can't give value to yourself")?;
                        let values = &mut other.values;
                        if values.len() <= 1 {
                            values.push(value);
                        } else {
                            return Err("Other bot already has 2 values");
                        }
                        if values.len() == 2 {
                            factory.active_bots.push_back(id);
                        }
                    }
                }
            }
        }
    }

    Ok(Err(factory))
}

fn find(bot: &Bot, id: usize) -> Option<usize> {
    if bot.values.contains(&61) && bot.values.contains(&17) {
        Some(id)
    } else {
        None
    }
}

fn ignore(_: &Bot, _: usize) -> Option<()> {
    None
}

lazy_static! {
    static ref OUTPUT: Mutex<String> = Mutex::new(String::new());
}

#[no_mangle]
pub unsafe extern "C" fn part1(text: *const c_char) -> *const c_char {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let mut output = OUTPUT.lock().unwrap();
    output.clear();

    match parse(text) {
        Ok(instructions) => {
            match execute_factory(&instructions, find) {
                Ok(Ok(id)) => writeln!(output, "Bot ID is: {}", id).unwrap(),
                Ok(Err(_)) => writeln!(output, "Not found").unwrap(),
                Err(e) => output.push_str(e),
            }
        }
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

    match parse(text) {
        Ok(instructions) => {
            match execute_factory(&instructions, ignore) {
                Ok(Err(factory)) => {
                    if factory.outputs.len() < 3 {
                        output.push_str("Factory doesn't have 3 outputs");
                    } else {
                        let result = factory.outputs[0] * factory.outputs[1] * factory.outputs[2];
                        writeln!(output, "Outputs multiplied: {}", result).unwrap();
                    }
                }
                Err(e) => output.push_str(e),
                _ => unreachable!(),
            }
        }
        Err(e) => output.push_str(e),
    }

    output.push('\0');
    output.as_ptr() as *const c_char
}
