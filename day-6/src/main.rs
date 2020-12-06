use std::collections::HashSet;
use std::error::Error;
use std::io::{Read, BufRead, BufReader};
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
    let sum = part_1(File::open(filename)?)?;
    println!("sum = {}", sum);

    Ok(())
}

pub fn part_1(input: impl Read) -> Result<usize, Box<dyn Error>> {
    let data: Vec<String> = parse_answers(input)?;
    Ok(data.into_iter()
        .map(get_answers)
        .map(|set| set.len())
        .sum())
}

/**
 * Take the input data and parse it into a list of strings, one string for each family
 */
pub fn parse_answers(input: impl Read) -> Result<Vec<String>, Box<dyn Error>> {
    let lines: Result<Vec<String>, _> = BufReader::new(input)
        .lines()
        .collect();
    let lines = lines?;
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
    Ok(grouped.into_iter()
        .map(|group| group.join(""))
        .collect())
}

/**
 * Take a string, pull it apart into chars, and throw the chars into a set
 */
pub fn get_answers(mut line: String) -> HashSet<char> {
    line.retain(|c| !c.is_whitespace());
    line.chars().collect()
}

#[test]
fn test_get_answers() {
    use std::iter::FromIterator;
    assert_eq!(get_answers("abcx\nabcy\nabcz".to_string()), HashSet::from_iter(vec!['a', 'b', 'c', 'x', 'y', 'z'].into_iter()));
}

#[test]
fn test_parse() {
    use std::io::Cursor;
    assert_eq!(parse_answers(Cursor::new("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb")).unwrap(), vec![
        "abc".to_string(),
        "abc".to_string(),
        "abac".to_string(),
        "aaaa".to_string(),
        "b".to_string(),
    ]);
}

#[test]
fn test_part_1() {
    use std::io::Cursor;
    assert_eq!(part_1(Cursor::new("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb")).unwrap(), 11);
}