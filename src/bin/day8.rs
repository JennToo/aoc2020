use aoc2020::advent_main;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

advent_main!(day8, "data/day8_input.txt");

fn day8(lines: &[String]) -> (i64, i64) {
    let emulator = Emulator::from_lines(lines);
    let (part1, _) = run_until_repeat_or_end(&mut (emulator.clone()));
    let part2 = find_and_replace_bad_instruction(&emulator);

    (part1, part2)
}

fn find_and_replace_bad_instruction(emulator: &Emulator) -> Word {
    for (instruction_index, instruction) in emulator.instruction_memory.iter().enumerate() {
        let mut clone = emulator.clone();
        match instruction {
            Instruction::Nop(data) => {
                clone.instruction_memory[instruction_index] = Instruction::Jmp(*data);
                if run_until_repeat_or_end(&mut clone).1 == RepeatOrEnd::End {
                    return clone.accumulator;
                }
            }
            Instruction::Jmp(data) => {
                clone.instruction_memory[instruction_index] = Instruction::Nop(*data);
                if run_until_repeat_or_end(&mut clone).1 == RepeatOrEnd::End {
                    return clone.accumulator;
                }
            }
            Instruction::Acc(_) => {}
        }
    }
    unreachable!();
}

fn run_until_repeat_or_end(emulator: &mut Emulator) -> (Word, RepeatOrEnd) {
    let mut visited_pcs = HashSet::new();

    while !visited_pcs.contains(&emulator.pc)
        && (emulator.pc as usize) < emulator.instruction_memory.len()
    {
        visited_pcs.insert(emulator.pc);
        emulator.run_cycle();
    }
    let stop_condition = if visited_pcs.contains(&emulator.pc) {
        RepeatOrEnd::Repeat
    } else {
        RepeatOrEnd::End
    };

    (emulator.accumulator, stop_condition)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RepeatOrEnd {
    Repeat,
    End,
}

type Word = i64;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Acc(Word),
    Jmp(Word),
    Nop(Word),
}

#[derive(Debug, Clone)]
struct Emulator {
    pc: Word,
    instruction_memory: Vec<Instruction>,
    accumulator: Word,
}

lazy_static! {
    static ref INSTRUCTION_REGEX: Regex = Regex::new(r"(\w+) ([+-]\d+)").unwrap();
}

impl Emulator {
    fn from_lines(lines: &[String]) -> Emulator {
        let instruction_memory = lines
            .iter()
            .map(String::as_str)
            .map(Instruction::from_line)
            .collect();

        Emulator {
            pc: 0,
            accumulator: 0,
            instruction_memory,
        }
    }

    fn run_cycle(&mut self) {
        if self.pc >= self.instruction_memory.len() as i64 || self.pc < 0 {
            panic!("Invalid PC: {}", self.pc);
        }

        match self.instruction_memory[self.pc as usize] {
            Instruction::Nop(_) => {
                self.pc += 1;
            }
            Instruction::Jmp(data) => {
                self.pc += data;
            }
            Instruction::Acc(data) => {
                self.accumulator += data;
                self.pc += 1;
            }
        }
    }
}

impl Instruction {
    fn from_line(line: &str) -> Instruction {
        let captures = INSTRUCTION_REGEX.captures(line).unwrap();
        let data = captures[2].parse::<Word>().unwrap();

        match &captures[1] {
            "nop" => Instruction::Nop(data),
            "acc" => Instruction::Acc(data),
            "jmp" => Instruction::Jmp(data),
            _ => unimplemented!(),
        }
    }
}
