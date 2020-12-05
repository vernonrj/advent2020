use std::error::Error;
use std::io::{Read, BufRead, BufReader};
use std::fs::File;

use clap::{App, Arg};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("day-5")
        .arg(Arg::with_name("input")
             .takes_value(true)
             .help("input file to use"))
        .arg(Arg::with_name("max")
             .short("m")
             .long("max")
             .help("returns the max seat ID (part 1)"))
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let f = File::open(input_file)?;

    if matches.is_present("max") {
        let max_code = part_1(f)?;
        println!("highest seat ID is {}", max_code);
    } else {
        let missing = part_2(f)?;
        println!("your seat ID: {}", missing);
    }

    Ok(())
}


/**
 * Returns the highest seat ID
 */
fn part_1(input: impl Read) -> Result<u32, Box<dyn Error>> {
    let data: Result<Vec<String>, _> = BufReader::new(input).lines().collect();
    let data = data?;
    let m = data.into_iter()
        .map(|code| seat_to_rows(&code).unwrap())
        .map(|(seat, row)| seat_id(seat, row))
        .max()
        .unwrap_or_default();
    
    Ok(m)
}


/**
 * Returns the seat ID that is missing from the middle of the list
 */
fn part_2(input: impl Read) -> Result<u32, Box<dyn Error>> {
    let data: Result<Vec<String>, _> = BufReader::new(input).lines().collect();
    let data = data?;

    // first get the list of seat IDs
    let mut seats: Vec<u32> = data.into_iter()
        .map(|code| seat_to_rows(&code).unwrap())
        .map(|(seat, row)| seat_id(seat, row))
        .collect();
    
    // now put them in order
    seats.sort();

    // now we window the data, and just look for the only occurrence where (a + 1 != b)
    let noncontig: Vec<u32> = seats.windows(2)
        .filter(|&window| window[0] + 1 != window[1])
        .map(|window| window[0] + 1)
        .collect();

    // finally we grab the first missing seat ID from the list. There should only be one
    noncontig.get(0).copied().ok_or_else(|| format!("empty list!").into())
}


/**
 * Takes a seating specifier and converts it to row and column
 *
 * ```
 * assert_eq!(seat_to_rows("FBFBBFFRLR").unwrap(), (44, 5));
 * ```
 */
pub fn seat_to_rows(seatcode: &str) -> Result<(u8, u8), Box<dyn Error>> {
    if seatcode.len() != 10 {
        return Err(format!("wrong length for seat specifier: `{}`", seatcode).into());
    }
    let mask: u32 = seatcode.chars().fold(0, |bits, ch| {
        let mask = match ch {
            'B' | 'R' => 1,
            _ => 0,
        };
        (bits << 1) | mask
    });
    let row = ((mask >> 3) & 0xff) as u8;
    let seat = (mask & 0x7) as u8;
    Ok((row, seat))
}


/**
 * convert row/seat to a seat ID
 */
pub fn seat_id(row: u8, seat: u8) -> u32 {
    (row as u32) * 8 + (seat as u32)
}


#[test]
fn test_seat_to_rows() {
    assert_eq!(seat_to_rows("FBFBBFFRLR").unwrap(), (44, 5));
    assert_eq!(seat_to_rows("BFFFBBFRRR").unwrap(), (70, 7));
    assert_eq!(seat_to_rows("FFFBBBFRRR").unwrap(), (14, 7));
    assert_eq!(seat_to_rows("BBFFBBFRLL").unwrap(), (102, 4));
}


#[test]
fn test_seat_id() {
    assert_eq!(seat_id(70, 7), 567);
    assert_eq!(seat_id(14, 7), 119);
    assert_eq!(seat_id(102, 4), 820);
}


#[test]
fn test_part_1() {
    use std::io::Cursor;
    let text = include_str!("../input.txt");
    let max = part_1(Cursor::new(text)).unwrap();
    assert_eq!(max, 896)
}


#[test]
fn test_part_2() {
    use std::io::Cursor;
    let text = include_str!("../input.txt");
    assert_eq!(part_2(Cursor::new(text)).unwrap(), 659);
}