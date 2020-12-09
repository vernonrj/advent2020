use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, BufRead, BufReader};
use std::fs::File;
use std::str::FromStr;

use clap::{App, Arg};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("day-8")
        .arg(Arg::with_name("input")
             .takes_value(true)
             .help("the input to the program")
             .required(true))
        .get_matches();
    
    let filename = matches.value_of("input").unwrap();
    let instructions: Vec<Instruction> = get_input(File::open(filename)?)?
        .into_iter()
        .map(|i| Instruction::from_str(&i).unwrap())
        .collect();
    
    let mut e = Emulator::new();
    let mut seen: HashSet<usize> = HashSet::new();
    while seen.insert(e.pc) {
        e.execute(instructions[e.pc]);
    }
    println!("first loop at PC {pc}. Accumulator = {acc}", pc=e.pc, acc=e.accumulator);

    match run_program(Emulator::new(), &instructions, 1, HashSet::new()) {
        TerminationCriteria::Terminated(emulator) => {
            println!("successfully terminated. Accumulator value = {}", emulator.accumulator);
        }
        TerminationCriteria::InfiniteLoop => {
            println!("oh noes no paths to termination found.");
        }
    }
    Ok(())
}


/**
 * Attempt to run the program from the current emulator state.
 * 
 * Execution runs either until the PC goes off the end of the instruction set, or an infinite loop is detected.
 * NOP or JUMP can be mutated to JUMP or NOP, respectively, but only `instructions_that_can_change` times.
 * Infinite loop detection is done by pushing the current PC into a set. If that specific PC has already been seen, an infinite loop has occurred.
 */
pub fn run_program(mut emulator: Emulator,
                   instructions: &[Instruction],
                   instructions_that_can_change: i32,
                   mut instructions_hit: HashSet<usize>) -> TerminationCriteria {
    // loop until we encounter an infinite loop (detected by inserting PC into a set and checking to see if we've that PC value before)
    while instructions_hit.insert(emulator.pc) {
        // try to get the next instruction. If the PC has gone off the end, then the program has terminated.
        let next_instruction = match instructions.get(emulator.pc) {
            None => return TerminationCriteria::Terminated(emulator), // WE DID IT, WE SAVED THE DAY!
            Some(i) => i,
        };
        // try to mutate the instruction JUMP{value} <=> NOP{value}; but only if we're still allowed to change instructions!
        // when a mutation is available, try to run the program to completion with that mutation.
        // If we're successful, then return with the end state of that emulator. Otherwise, continue executing from here.
        match next_instruction.try_mutate() {
            Some(mutated) if instructions_that_can_change > 0 => {
                // mutation is allowed! Try to run the program to completion with this mutation.
                let mut alternate_reality = emulator.clone();
                alternate_reality.execute(mutated); // have to execute the mutated instruction separately, since it's not part of the instruction slice
                let terminated_how = run_program(alternate_reality, instructions, instructions_that_can_change - 1, instructions_hit.clone());

                if let done @ TerminationCriteria::Terminated(_) = terminated_how {
                    // program terminated successfully. Pass that up the stack.
                    return done;
                }
            },
            // mutation is not allowed. Pass-through.
            Some(_) | None => (),
        }
        // okay, execute this instruction. That'll modify the accumulator and the PC. Later, we'll see if we've hit our end conditions.
        emulator.execute(*next_instruction);
    }
    // Infinite loop occurred. Bail.
    TerminationCriteria::InfiniteLoop
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminationCriteria {
    InfiniteLoop,
    Terminated(Emulator),
}

fn get_input(input: impl Read) -> Result<Vec<String>, io::Error> {
    BufReader::new(input)
    .lines()
    .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    NoOperation(i64),
    Accumulate(i64),
    Jump(i64),
}

impl Instruction {
    pub fn try_mutate(self) -> Option<Instruction> {
        match self {
            Self::NoOperation(value) => Some(Self::Jump(value)),
            Self::Accumulate(_) => None,
            Self::Jump(value) => Some(Self::NoOperation(value)),
        }
    }
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.splitn(2, char::is_whitespace);
        let op = match split.next() {
            Some(o) => o,
            None => return Err(format!("failed to find operation in `{}`", s).into()),
        };
        let arg = split.next();
        match (op, arg) {
            ("nop", Some(arg)) => Ok(Self::NoOperation(arg.parse()?)),
            ("acc", Some(arg)) => Ok(Self::Accumulate(arg.parse()?)),
            ("jmp", Some(arg)) => Ok(Self::Jump(arg.parse()?)),
            ("nop", None) | ("acc", None) | ("jmp", None) => Err(format!("{op}: Expected argument, got none", op=op).into()),
            (bad_op, _) => Err(format!("Unexpected Operation `{op}`", op=bad_op).into()),
        }
    }
}


#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Emulator {
    pub accumulator: i64,
    pub pc: usize,
}

impl Emulator {
    pub fn new() -> Self { Self::default() }
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::NoOperation(_) => (),
            Instruction::Accumulate(value) => {
                self.accumulator += value;
            },
            Instruction::Jump(rel) => {
                let new_pc = self.pc as i64 + rel - 1;
                if new_pc < 0 {
                    panic!("Program Counter Underflow on instruction {:?}", instruction);
                }
                self.pc = new_pc as usize;
            }
        }
        self.pc += 1;
    }
}

#[test]
fn test_instruction_from_str() {
    assert_eq!(Instruction::from_str("nop +0").unwrap(), Instruction::NoOperation(0));
    assert_eq!(Instruction::from_str("acc +1").unwrap(), Instruction::Accumulate(1));
    assert_eq!(Instruction::from_str("acc -99").unwrap(), Instruction::Accumulate(-99));
    assert_eq!(Instruction::from_str("jmp -4").unwrap(), Instruction::Jump(-4));
}

#[test]
fn test_emulator() {
    let mut e = Emulator::new();
    let instrs = [
        Instruction::from_str("nop +0").unwrap(),
        Instruction::from_str("acc +1").unwrap(),
        Instruction::from_str("jmp +4").unwrap(),
        Instruction::from_str("acc +3").unwrap(),
        Instruction::from_str("jmp -3").unwrap(),
        Instruction::from_str("acc -99").unwrap(),
        Instruction::from_str("acc +1").unwrap(),
        Instruction::from_str("jmp -4").unwrap(),
        Instruction::from_str("acc +6").unwrap(),
    ];
    assert_eq!(e.pc, 0);
    e.execute(instrs[e.pc]); // nop +0
    assert_eq!(e.pc, 1);
    e.execute(instrs[e.pc]); // acc +1
    assert_eq!(e.pc, 2);
    e.execute(instrs[e.pc]); // jmp +4
    assert_eq!(e.pc, 6);
    e.execute(instrs[e.pc]); // acc +1
    assert_eq!(e.pc, 7);
    e.execute(instrs[e.pc]); // jmp -4
    assert_eq!(e.pc, 3);
    e.execute(instrs[e.pc]); // acc +3
    assert_eq!(e.pc, 4);
    e.execute(instrs[e.pc]); // jmp -3
    assert_eq!(e.pc, 1);
}