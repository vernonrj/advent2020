use std::collections::HashSet;
use std::error::Error;
use std::io::{Read, BufRead, BufReader};
use std::iter::FromIterator;
use std::fs::File;

use clap::{App, Arg};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("day-6")
        .arg(Arg::with_name("input")
             .required(true)
             .takes_value(true)
             .help("input to the function"))
        .get_matches();
    let filename = matches.value_of("input").unwrap();
    let part1_sum = part_1(File::open(filename)?)?;
    println!("part 1 sum = {}", part1_sum);

    let part2_sum = part_2(File::open(filename)?)?;
    println!("part 2 sum = {}", part2_sum);
    Ok(())
}

pub fn part_1(input: impl Read) -> Result<usize, Box<dyn Error>> {
    let data: Vec<Vec<String>> = parse_answers(input)?;
    Ok(data.into_iter()
        .map(|family| get_answers(family, CombineMode::AnyoneAnsweredYes))
        .map(|set| set.len())
        .sum())
}

pub fn part_2(input: impl Read) -> Result<usize, Box<dyn Error>> {
    let data: Vec<Vec<String>> = parse_answers(input)?;
    Ok(data.into_iter()
        .map(|family| get_answers(family, CombineMode::EveryoneAnsweredYes))
        .map(|set| set.len())
        .sum())
}

/**
 * Take the input data and parse it into a list of strings, one vec for each family
 */
pub fn parse_answers(input: impl Read) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let lines: Result<Vec<String>, _> = BufReader::new(input)
        .lines()
        .collect();
    let lines = lines?;
    // perform an unflatten. Content between empty lines is grouped into a vector.
    let (mut grouped, last): (Vec<Vec<String>>, Vec<String>) = lines.into_iter()
        .fold((Vec::new(), Vec::new()), |(mut groups, mut current), line| {
            if line.is_empty() {
                groups.push(current);
                current = Vec::new();
            } else {
                current.push(line);
            }
            (groups, current)
        });
    if ! last.is_empty() {
        grouped.push(last);
    }
    Ok(grouped)
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CombineMode {
    AnyoneAnsweredYes,
    EveryoneAnsweredYes,
}

/**
 * Take a list of strings, pull each string apart into chars, throw the chars into sets, and combine the sets based on the combine mode
 */
pub fn get_answers(family_answers: Vec<String>, combine: CombineMode) -> HashSet<char> {
    let mut first = true;
    family_answers.into_iter()
        .map(|person_answer| HashSet::from_iter(person_answer.chars()))
        // fold_first would work better, but it's not on stable rust yet
        .fold(HashSet::new(), |answers, next_member_answer| {
            if first {
                first = false;
                next_member_answer
            } else {
                match combine {
                    CombineMode::AnyoneAnsweredYes => { answers.union(&next_member_answer).copied().collect() }
                    CombineMode::EveryoneAnsweredYes => { answers.intersection(&next_member_answer).copied().collect() }
                }
            }
        })
}

#[test]
fn test_get_answers() {
    use std::iter::FromIterator;
    let answers = vec!["abcx".to_string(), "abcy".to_string(), "abcz".to_string()];
    assert_eq!(get_answers(answers, CombineMode::AnyoneAnsweredYes),
               HashSet::from_iter(vec!['a', 'b', 'c', 'x', 'y', 'z'].into_iter()));
}

#[test]
fn test_parse() {
    use std::io::Cursor;
    let answers = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
    assert_eq!(parse_answers(Cursor::new(answers)).unwrap(), vec![
        vec!["abc".to_string()],
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
        vec!["ab".to_string(), "ac".to_string()],
        vec!["a".to_string(), "a".to_string(), "a".to_string(), "a".to_string()],
        vec!["b".to_string()],
    ]);
}

#[test]
fn test_part_1() {
    use std::io::Cursor;
    let answers = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
    assert_eq!(part_1(Cursor::new(answers)).unwrap(), 11);
}

#[test]
fn test_part_2() {
    use std::io::Cursor;
    let answers = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
    assert_eq!(part_2(Cursor::new(answers)).unwrap(), 6);
}