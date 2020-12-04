#[macro_use] extern crate lazy_static;

use std::{error::Error, ops::{RangeInclusive}};
use std::io::{self, Read, BufRead, BufReader};
use std::fs::File;
use std::str::FromStr;

use clap::{App, Arg};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("day-2")
            .arg(Arg::with_name("input")
                .required(true)
                .help("the input to the program"))
            .arg(Arg::with_name("policy")
                .short("p")
                .long("policy")
                .takes_value(true)
                .possible_values(&["range", "xor"])
                .help("policy to use"))
            .get_matches();
    let variant = match matches.value_of("policy") {
        Some("range") | None => PolicyVariant::OccurrenceRange,
        Some("xor") => PolicyVariant::XorPosition,
        Some(bad) => panic!("invalid value to --policy: {}", bad),
    };
    let f = File::open(matches.value_of("input").unwrap())?;
    let input: Vec<String> = get_input(f)?;
    let passwords: Vec<(String, Policy)> = parse_input(input).unwrap();
    let valid: Vec<(String, Policy)> = passwords.into_iter().filter(|(pass, policy)| {
        policy.is_valid(&pass, variant)
    }).collect();
    
    println!("number of valid passwords: {}", valid.len());
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Policy {
    pub letter: String,
    pub occurrences: RangeInclusive<i64>,
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PolicyVariant {
    /// letters must occur within the range given
    OccurrenceRange,
    /// a letter must occur in either the start or end position, not both
    XorPosition,
}

impl Policy {
    pub fn is_valid(&self, password: &str, variant: PolicyVariant) -> bool {
        match variant {
            PolicyVariant::OccurrenceRange => {
                let policy_occs = password.matches(&self.letter).count();
                self.occurrences.contains(&(policy_occs as i64))
            },
            PolicyVariant::XorPosition => {
                let chars: Vec<char> = password.chars().collect();
                let start = match ((*self.occurrences.start()) as usize).checked_sub(1) {
                    Some(n) => n,
                    None => return false,
                };
                let end = match ((*self.occurrences.end()) as usize).checked_sub(1) {
                    Some(n) => n,
                    None => return false,
                };
                match (chars.get(start), chars.get(end)) {
                    (Some(c), Some(d)) if c == d => false,
                    (Some(c), _) if c.to_string() == self.letter => true,
                    (_, Some(c)) if c.to_string() == self.letter => true,
                    (_, _) => false,
                }
            }
        }
    }
}

impl FromStr for Policy {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<low>\d+)-(?P<high>\d+)\s+(?P<chars>\w+):\s+(?P<password>\w+)").unwrap();
        }
        let m = match RE.captures(s) {
            Some(cap) => cap,
            None => return Err(format!("invalid input: `{}`", s).into()),
        };
        let low = m.name("low").and_then(|l| l.as_str().parse::<i64>().ok());
        let high = m.name("high").and_then(|h| h.as_str().parse::<i64>().ok());
        let chars = m.name("chars").map(|c| c.as_str());
        match (low, high, chars) {
            (Some(l), Some(h), Some(c)) => Ok(Policy {
                letter: c.to_string(),
                occurrences: l..=h,
            }),
            _ => Err(format!("failed to parse into a policy").into())
        }
    }
}

pub fn get_input(reader: impl Read) -> io::Result<Vec<String>> {
    let data: Result<Vec<String>, _> = BufReader::new(reader)
        .lines()
        .collect();
    data
}

pub fn parse_input(input: impl IntoIterator<Item=String>) -> Result<Vec<(String, Policy)>, Box<dyn Error>> {
    let mut output = Vec::new();
    for line in input {
        let password = get_password_from_line(&line)?;
        let policy = Policy::from_str(&line)?;
        output.push((password.to_string(), policy));
    }
    Ok(output)
}

pub fn get_password_from_line(line: &str) -> Result<&str, Box<dyn Error>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".*:\s+(?P<password>\w+)").unwrap();
    }
    let maybe_password = RE.captures(line).and_then(|cap| cap.name("password"));
    match maybe_password {
        Some(m) => Ok(m.as_str()),
        None => Err(format!("failed to get password from line `{}`", line).into()),
    }
}


#[test]
fn test_part_1() {
    use std::io::Cursor;
    let data = include_str!("../input.txt");
    let data = get_input(Cursor::new(data)).unwrap();
    let passwords: Vec<(String, Policy)> = parse_input(data).unwrap();
    let valid: Vec<(String, Policy)> = passwords.into_iter().filter(|(pass, policy)| {
        policy.is_valid(&pass, PolicyVariant::OccurrenceRange)
    }).collect();
    assert_eq!(valid.len(), 638);
}

#[test]
fn test_part_2() {
    use std::io::Cursor;
    let data = include_str!("../input.txt");
    let data = get_input(Cursor::new(data)).unwrap();
    let passwords: Vec<(String, Policy)> = parse_input(data).unwrap();
    let valid: Vec<(String, Policy)> = passwords.into_iter().filter(|(pass, policy)| {
        policy.is_valid(&pass, PolicyVariant::XorPosition)
    }).collect();
    assert_eq!(valid.len(), 699);
}

#[test]
fn test_get_password_from_line() {
    assert_eq!(get_password_from_line("1-3 a: abcde").unwrap(), "abcde");
    assert_eq!(get_password_from_line("1-3 b: cdefg").unwrap(), "cdefg");
    assert_eq!(get_password_from_line("2-9 c: ccccccccc").unwrap(), "ccccccccc");
}

#[test]
fn test_policy_from_str() {
    assert_eq!(Policy::from_str("1-3 a: abcde").unwrap(), Policy { letter: "a".to_string(), occurrences: 1..=3 });
    assert_eq!(Policy::from_str("1-3 b: cdefg").unwrap(), Policy { letter: "b".to_string(), occurrences: 1..=3 });
    assert_eq!(Policy::from_str("2-9 c: ccccccccc").unwrap(), Policy { letter: "c".to_string(), occurrences: 2..=9 });
}

#[test]
fn test_parse_input() {
    let input = vec![
        "1-3 a: abcde".to_string(),
        "1-3 b: cdefg".to_string(),
        "2-9 c: ccccccccc".to_string(),
    ];
    assert_eq!(parse_input(input).unwrap(), vec![
        ("abcde".to_string(), Policy { letter: "a".to_string(), occurrences: 1..=3 }),
        ("cdefg".to_string(), Policy { letter: "b".to_string(), occurrences: 1..=3 }),
        ("ccccccccc".to_string(), Policy { letter: "c".to_string(), occurrences: 2..=9 }),
    ])
}

#[test]
fn test_is_policy_valid() {
    assert!(Policy { letter: "a".to_string(), occurrences: 1..=3 }.is_valid("abcde", PolicyVariant::OccurrenceRange));
    assert!(Policy { letter: "a".to_string(), occurrences: 1..=3 }.is_valid("aaabcde", PolicyVariant::OccurrenceRange));
    assert!(! Policy { letter: "b".to_string(), occurrences: 1..=3 }.is_valid("cdefg", PolicyVariant::OccurrenceRange));
    assert!(Policy { letter: "c".to_string(), occurrences: 2..=9 }.is_valid("ccccccccc", PolicyVariant::OccurrenceRange));

    assert!(Policy { letter: "a".to_string(), occurrences: 1..=3 }.is_valid("abcde", PolicyVariant::XorPosition));
    assert!(! Policy { letter: "b".to_string(), occurrences: 1..=3 }.is_valid("cdefg", PolicyVariant::XorPosition));
    assert!(! Policy { letter: "c".to_string(), occurrences: 2..=9 }.is_valid("ccccccccc", PolicyVariant::XorPosition));
}