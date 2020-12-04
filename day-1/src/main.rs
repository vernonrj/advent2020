extern crate clap;

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::{App, Arg};

mod tree;

use tree::SumTree;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("day-1")
        .arg(Arg::with_name("input")
            .required(true)
            .help("the input to the program"))
        .arg(Arg::with_name("numbers")
            .short("n")
            .long("num")
            .default_value("2")
            .takes_value(true)
            .help("number of nums to use in the sum"))
        .get_matches();

    let to_sum: i64 = matches.value_of("numbers").unwrap().parse().unwrap();
    let input = get_input(matches.value_of("input").unwrap())?;

    match get_sum_tree(2020, to_sum, &input) {
        Some(nums) => println!("found numbers {:?}, which multiply to {}", nums, nums.iter().copied().product::<i64>()),
        None => eprintln!("No matching numbers found"),
    }
    Ok(())
}

/**
 * slurps data from a file into a vec of integers. One integer per line.
 */
pub fn get_input(filename: &str) -> io::Result<Vec<i64>> {
    let input_file = File::open(filename)?;

    let mut output = Vec::new();
    for line in BufReader::new(input_file).lines() {
        let line: String = line?;
        match line.parse::<i64>() {
            Ok(n) => output.push(n),
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "bad data")),
        }
    }

    Ok(output)
}

/**
 * Returns a list of elements from `input` of length `to_sum` that sum up to `n`
 */
pub fn get_sum_to(n: i64, to_sum: i64, input: &[i64]) -> Option<Vec<&i64>> {
    Permutations::new(input, to_sum as usize)
        .filter(|perm| perm.into_iter().copied().sum::<i64>() == n)
        .next()
}

pub fn get_sum_tree(n: i64, to_sum: i64, input: &[i64]) -> Option<Vec<i64>> {
    let mut t = SumTree::new(to_sum);
    for each in input {
        t.insert(*each);
    }
    t.find(n)
}

/**
 * Calculates permutations of a list of elements, with an arbitrary number of elements to permute
 * 
 * ```rust
 * assert_eq!(Permutations::new(&[1, 2, 3], 2).collect::<Vec<i32>>(),
 *            vec![vec![1, 2], vec![1, 3], vec![2, 3]]);
 * ```
 */
pub struct Permutations<'a, T> {
    elements: &'a [T],
    indexes: Vec<usize>,
}

impl<'a, T> Permutations<'a, T> {
    pub fn new(elems: &'a [T], num_indexes: usize) -> Self {
        let mut indexes: Vec<usize> = (0..).take(num_indexes).collect();
        indexes.reverse();
        Permutations {
            elements: elems,
            indexes: indexes,
        }
    }
    /**
     * increments an index of idx `idx` (in the simple case, `self.indexes[idx]++`). Recurses on carry.
     */
    fn inc_index(&mut self, idx: usize) -> Option<usize> {
        if idx >= self.indexes.len() {
            return None;
        }
        self.indexes[idx] += 1;
        if self.indexes[idx] >= self.elements.len() {
            match self.inc_index(idx + 1) {
                Some(new_idx) if new_idx < self.elements.len() => {
                    // we have a valid index to pull from
                    self.indexes[idx] = new_idx;
                    return Some(new_idx + 1);
                },
                Some(_) => {
                    // we need to increment further up the chain
                    let elem_len = self.elements.len();
                    self.indexes.get_mut(idx + 1).map(|next_idx| *next_idx = elem_len);
                    return self.inc_index(idx);
                },
                None => return None,
            }
        }
        Some(self.indexes[idx] + 1)
    }
}

impl<'a, T> Iterator for Permutations<'a, T> {
    type Item = Vec<&'a T>;
    fn next(&mut self) -> Option<Self::Item> {
        // first, map our list of indexes into elements. If any indexes are out of range, bail.
        let elems: Vec<Option<&T>> = self.indexes.iter()
            .rev()
            .map(|idx| self.elements.get(*idx))
            .collect();
        if elems.iter().any(Option::is_none) {
            return None;
        }
        // now increment the indexes. We don't care if this goes out of range... yet!
        self.inc_index(0);
        // finally take the list of elems we got before, and flatten them to pull the values out of their options.
        Some(elems.into_iter().flatten().collect())
    }
}

#[test]
fn test_permutator_inc() {
    let mut p = Permutations::new(&[0, 0, 0], 2);
    assert_eq!(*&p.indexes, &[1, 0]);
    p.inc_index(0).unwrap();
    assert_eq!(*&p.indexes, &[2, 0]);
    p.inc_index(0).unwrap();
    assert_eq!(*&p.indexes, &[2, 1]);
}

#[test]
fn test_permutations_2() {
    assert_eq!(Permutations::new(&[1, 2, 3], 2).collect::<Vec<_>>(), vec![vec![&1, &2], vec!(&1, &3), vec!(&2, &3)]);
    assert!(Permutations::new(&[1], 2).collect::<Vec<_>>().is_empty());
    assert_eq!(Permutations::new(&[1, 2], 2).collect::<Vec<_>>(), vec![vec!(&1, &2)]);
}

#[test]
fn test_permutations_3() {
    assert_eq!(Permutations::new(&[1, 2, 3, 4], 3).collect::<Vec<_>>(),
                vec![vec![&1, &2, &3], vec![&1, &2, &4], vec![&1, &3, &4], vec![&2, &3, &4]]);
    assert!(Permutations::new(&[1], 3).collect::<Vec<_>>().is_empty());
    assert!(Permutations::new(&[1, 2], 3).collect::<Vec<_>>().is_empty());
    assert_eq!(Permutations::new(&[1, 2, 3], 3).collect::<Vec<_>>(), vec![vec!(&1, &2, &3)]);
}

#[test]
fn test_get_sum_to() {
    assert_eq!(get_sum_to(5, 2, &[1, 2, 3]), Some(vec!(&2, &3)));
    assert_eq!(get_sum_to(5, 2, &[1, 2, 3, 4]), Some(vec!(&1, &4)));
    assert_eq!(get_sum_to(5, 2, &[1, 2, 4]), Some(vec!(&1, &4)));
    assert_eq!(get_sum_to(10, 2, &[1, 2, 4]), None);

    assert_eq!(get_sum_to(20, 3, &[1, 2, 4, 6, 10]), Some(vec![&4, &6, &10]));
}

#[test]
fn test_get_sum_tree() {
    assert_eq!(get_sum_tree(5, 2, &[1, 2, 3]), Some(vec!(3, 2)));
    assert_eq!(get_sum_tree(5, 2, &[1, 2, 3, 4]), Some(vec!(4, 1)));
    assert_eq!(get_sum_tree(5, 2, &[1, 2, 4]), Some(vec!(4, 1)));
    assert_eq!(get_sum_tree(10, 2, &[1, 2, 4]), None);

    assert_eq!(get_sum_tree(20, 3, &[1, 2, 4, 6, 10]), Some(vec![10, 6, 4]));
}