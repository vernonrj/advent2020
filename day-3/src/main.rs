use std::error::Error;
use std::io::{Read, BufRead, BufReader};
use std::fs::File;
use std::ops::Index;
use std::str::FromStr;

use clap::{App, Arg};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("day-3")
        .arg(Arg::with_name("input")
            .required(true)
            .takes_value(true)
            .help("input file for the day"))
        .arg(Arg::with_name("check-all")
            .short("c")
            .long("check-all")
            .help("check slopes of 1, 1/3, 1/5, 1/7, and 2"))
        .get_matches();

    let f = File::open(matches.value_of("input").unwrap())?;
    let field = Field::from_reader(f)?;

    if matches.is_present("check-all") {
        // check a series of slopes for trees. The input data is listed below as an array tuples of (right, down).
        let slopes = [
            (1, 1),
            (3, 1),
            (5, 1),
            (7, 1),
            (1, 2),
        ];
        let nums_trees: Vec<u64> = slopes.iter()
            .map(|(right, down)| count_trees(&field, *right, *down))
            .collect();
            
        println!("trees encountered: {:?}", nums_trees);
        println!("product: {}", nums_trees.into_iter().product::<u64>());
    } else {
        // just check a slope of right 3, down 1
        let num_trees = count_trees(&field, 3, 1);

        println!("number of trees encountered: {}", num_trees);
    }
    
    Ok(())
}

/**
 * Counts the number of trees encountered while toboggoning down a field
 */
pub fn count_trees(field: &Field, slope_right: usize, slope_down: usize) -> u64 {
    let mut row = 0;
    let mut trees = 0;
    for column in field.into_iter().step_by(slope_down) {
        match column[row] {
            Coordinate::Tree => trees += 1,
            _ => (),
        }
        row += slope_right;
    }
    trees
}

/**
 * Implementation of a field with trees and such
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    rows: Vec<Row>,
}

impl Field {
    /// Construct a field from a reader. Each line is another row in the field.
    pub fn from_reader(r: impl Read) -> Result<Self, Box<dyn Error>> {
        let mut rows = Vec::new();
        for line in BufReader::new(r).lines() {
            let line: String = line?;
            let col = Row::from_str(&line)?;
            rows.push(col);
        }
        Ok(Self { rows: rows })
    }
    /// Returns the number of 
    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

impl Index<usize> for Field {
    type Output = Row;
    fn index(&self, idx: usize) -> &Self::Output {
        self.rows.index(idx)
    }
}

impl IntoIterator for Field {
    type Item = Row;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}

impl<'a> IntoIterator for &'a Field {
    type Item = &'a Row;
    type IntoIter = std::slice::Iter<'a, Row>;

    fn into_iter(self) -> Self::IntoIter
        where Self: 'a
    {
        self.rows.iter()
    }
}

/**
 * A Row in the Field. Rows are the vertical coordinates of the field.
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    /// the columns in this row
    columns: Vec<Coordinate>,
}

impl FromStr for Row {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse the line given via Coordinate's construction function
        let columns: Result<Vec<Coordinate>, _> = s.chars().map(Coordinate::from_char).collect();
        let columns = columns?;
        Ok(Row {
            columns: columns
        })
    }
}

impl Index<usize> for Row {
    type Output = Coordinate;
    fn index(&self, idx: usize) -> &Self::Output {
        // row's indexing wraps if the index is larger than the number of columns available.
        let idx_mod = idx % self.columns.len();
        &self.columns[idx_mod]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Coordinate {
    /// An open space
    Open,
    /// A tree. Wouldn't want to hit that
    Tree,
}

impl Coordinate {
    /**
     * Construct a Coordinate from a character.
     */
    pub fn from_char(c: char) -> Result<Self, Box<dyn Error>> {
        match c {
            '.' => Ok(Self::Open),
            '#' => Ok(Self::Tree),
            e => Err(format!("invalid character for coordinate: '{}'", e).into())
        }
    }
}

#[test]
fn test_part_1() {
    use std::io::Cursor;
    let input = include_str!("../input.txt");
    let field = Field::from_reader(Cursor::new(input)).unwrap();

    let num_trees = count_trees(&field, 3, 1);
    assert_eq!(num_trees, 162);
}

#[test]
fn test_part_2() {
    use std::io::Cursor;
    let input = include_str!("../input.txt");
    let field = Field::from_reader(Cursor::new(input)).unwrap();
    let slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    let nums_trees: Vec<u64> = slopes.iter()
        .map(|(right, down)| count_trees(&field, *right, *down))
        .collect();
    assert_eq!(nums_trees, vec![80, 162, 77, 83, 37]);
    assert_eq!(nums_trees.iter().copied().product::<u64>(), 3064612320);
}


#[test]
fn test_coordinate_from_char() {
    assert_eq!(Coordinate::from_char('.').unwrap(), Coordinate::Open);
    assert_eq!(Coordinate::from_char('#').unwrap(), Coordinate::Tree);
    assert!(Coordinate::from_char('n').is_err());
}

#[test]
fn test_row_from_str() {
    assert_eq!(Row::from_str(".##.").unwrap(), Row { columns: vec![Coordinate::Open, Coordinate::Tree, Coordinate::Tree, Coordinate::Open]});
}

#[test]
fn test_field_construct() {
    use std::io::Cursor;
    let input = Cursor::new(
        "#..#\n####\n.#..\n"
    );
    let field = Field::from_reader(input).unwrap();
    assert_eq!(field, Field {
        rows: vec![
            Row { columns: vec![Coordinate::Tree, Coordinate::Open, Coordinate::Open, Coordinate::Tree], },
            Row { columns: vec![Coordinate::Tree, Coordinate::Tree, Coordinate::Tree, Coordinate::Tree], },
            Row { columns: vec![Coordinate::Open, Coordinate::Tree, Coordinate::Open, Coordinate::Open], },
        ]
    })
}

#[test]
fn test_field_index() {
    use std::io::Cursor;
    let input = Cursor::new(
        "#..#\n####\n.#..\n"
    );
    let field = Field::from_reader(input).unwrap();
    assert_eq!(field[0][0], Coordinate::Tree);
}

#[test]
fn test_column_index() {
    let row = Row { columns: vec![Coordinate::Tree, Coordinate::Open, Coordinate::Open, Coordinate::Open ]};
    assert_eq!(row[0], Coordinate::Tree);
    assert_eq!(row[1], Coordinate::Open);
    assert_eq!(row[2], Coordinate::Open);
    assert_eq!(row[3], Coordinate::Open);

    assert_eq!(row[4], Coordinate::Tree);
    assert_eq!(row[5], Coordinate::Open);
}

#[test]
fn test_count_trees() {
    use std::io::Cursor;
    let input = Cursor::new(
        vec!["..##.......",
             "#...#...#..",
             ".#....#..#.",
             "..#.#...#.#",
             ".#...##..#.",
             "..#.##.....",
             ".#.#.#....#",
             ".#........#",
             "#.##...#...",
             "#...##....#",
             ".#..#...#.#"].join("\n")
    );
    let field = Field::from_reader(input).unwrap();
    assert_eq!(count_trees(&field, 3, 1), 7);
}

#[test]
fn test_count_trees_vary_slope() {
    use std::io::Cursor;
    let input = Cursor::new(
        vec!["..##.......",
             "#...#...#..",
             ".#....#..#.",
             "..#.#...#.#",
             ".#...##..#.",
             "..#.##.....",
             ".#.#.#....#",
             ".#........#",
             "#.##...#...",
             "#...##....#",
             ".#..#...#.#"].join("\n")
    );
    let field = Field::from_reader(input).unwrap();
    let slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    let nums_trees: Vec<u64> = slopes.iter()
        .map(|(right, down)| count_trees(&field, *right, *down))
        .collect();
    assert_eq!(nums_trees, vec![2, 7, 3, 4, 2]);
    assert_eq!(nums_trees.iter().copied().product::<u64>(), 336);
}