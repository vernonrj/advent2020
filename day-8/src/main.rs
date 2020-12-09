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

    Ok(())
}

fn get_input(input: impl Read) -> Result<Vec<String>, io::Error> {
    BufReader::new(input)
    .lines()
    .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    NoOperation,
    Accumulate(i64),
    Jump(i64),
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
            ("nop", _) => Ok(Self::NoOperation),
            ("acc", Some(arg)) => Ok(Self::Accumulate(arg.parse()?)),
            ("jmp", Some(arg)) => Ok(Self::Jump(arg.parse()?)),
            ("acc", None) | ("jmp", None) => Err(format!("{op}: Expected argument, got none", op=op).into()),
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
            Instruction::NoOperation => (),
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
    assert_eq!(Instruction::from_str("nop +0").unwrap(), Instruction::NoOperation);
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