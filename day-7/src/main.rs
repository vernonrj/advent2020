#[macro_use] extern crate lazy_static;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};

use clap::{App, Arg};
use regex::Regex;


fn main() {
    let matches = App::new("day-7")
        .arg(Arg::with_name("input")
            .takes_value(true)
            .help("the input for the day"))
        .get_matches();

    let filename = matches.value_of("input").unwrap();
    let has_shiny_gold_bag = part_1(File::open(filename).unwrap()).unwrap();
    println!("{} bags can contain it", has_shiny_gold_bag);

    let bags_in_shiny_gold_bag = part_2(File::open(filename).unwrap(), "shiny gold").unwrap();
    println!("{} bags within a shiny gold bag", bags_in_shiny_gold_bag);
}

/**
 * How many colors can eventually contain at least one shiny gold bag?
 */
pub fn part_1(input: impl Read) -> Result<usize, Box<dyn Error>> {
    let lines: Result<Vec<String>, _> = BufReader::new(input)
        .lines()
        .collect();
    let lines = lines?;

    let mut bags = Bags::new();
    for each_line in lines {
        bags.insert_by_line(&each_line);
    }

    let found: BTreeSet<String> = bags.contains("shiny gold").collect();
    Ok(found.len())
}

pub fn part_2(input: impl Read, bag_type: &str) -> Result<u32, Box<dyn Error>> {
    let lines: Result<Vec<String>, _> = BufReader::new(input)
    .lines()
    .collect();
    let lines = lines?;

    let mut bags = Bags::new();
    for each_line in lines {
        bags.insert_by_line(&each_line);
    }

    let it = match bags.contents_recursive(bag_type) {
        Some(bags) => bags,
        None => return Err(format!("no bags found").into()),
    };

    let map = it.fold(BTreeMap::new(), |mut m, (key, val)| {
        *m.entry(key).or_insert(0u32) += val;
        m
    });
    Ok(map.values().sum())
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Bags {
    rules: BTreeMap<String, BTreeMap<String, u32>>,
}

impl Bags {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert_by_line(&mut self, line: &str) {
        lazy_static! {
            static ref SPLITTER: Regex = Regex::new(r"(?P<bigbag>\w+\s+\w+)\s+bags\s+contain\s+(?P<rest>.*)\.").unwrap();
            static ref CONTAINS: Regex = Regex::new(r"(?P<num>\d+)\s+(?P<bag>\w+\s+\w+)\s+bag(s)?,?").unwrap();
        }
        let split = SPLITTER.captures(line).unwrap();
        let bigbag = &split["bigbag"];
        let rest = &split["rest"];
        let mut bag_contents = BTreeMap::new();
        for cap in CONTAINS.captures_iter(rest) {
            bag_contents.insert(cap["bag"].to_string(), cap["num"].parse().unwrap());
        }
        self.rules.insert(bigbag.to_string(), bag_contents);
    }
    fn contains_1<'a>(&'a self, smaller_bag: &str) -> impl 'a + Iterator<Item=String> {
        let smaller_bag = String::from(smaller_bag);
        self.rules.iter()
            .filter(move |&(_, value)| value.contains_key(&smaller_bag))
            .map(|(key, _)| key.clone())
    }
    pub fn contains(&self, smaller_bag: &str) -> impl Iterator<Item=String> {
        let mut found: BTreeSet<String> = self.contains_1(smaller_bag).collect();
        let mut current_len = found.len();
        let mut last_len = 0;

        while current_len != last_len {
            last_len = current_len;
            let next_found: Vec<_> = found.iter().flat_map(|s| self.contains_1(&s)).collect();
            found.extend(next_found.into_iter());
            current_len = found.len();
        }
        
        found.into_iter()
    }
    pub fn contents(&self, key: &str) -> Option<impl Iterator<Item=(String, u32)>> {
        self.rules.get(key).map(|hm| hm.clone().into_iter())
    }
    pub fn contents_recursive(&self, key: &str) -> Option<impl Iterator<Item=(String, u32)>> {
        let mut output: Vec<(String, u32)> = Vec::new();
        let found = self.contents(key)?;
        for (key, num) in found {
            output.push((key.clone(), num));
            if let Some(it) = self.contents_recursive(&key) {
                let it_mult = it.map(|(key, oldnum)| (key, num*oldnum));
                output.extend(it_mult);
            }
        }
        Some(output.into_iter())
    }
}

#[test]
fn test_bag_insert() {
    let mut b = Bags::new();
    b.insert_by_line("light red bags contain 1 bright white bag, 2 muted yellow bags.");
    assert_eq!(b.rules.get("light red").unwrap(), &vec![("bright white".to_string(), 1u32), ("muted yellow".to_string(), 2u32)].into_iter().collect::<BTreeMap<_, _>>());
}


#[test]
fn test_contents() {
    let mut b = Bags::new();
    b.insert_by_line("light red bags contain 1 bright white bag, 2 muted yellow bags.");

    assert_eq!(b.contents("light red").unwrap().collect::<Vec<_>>(), vec![("bright white".to_string(), 1u32), ("muted yellow".to_string(), 2u32)]);
}

#[test]
fn test_contents_recursive() {
    let rules = [
        "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
        "bright white bags contain 1 shiny gold bag.",
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
        "faded blue bags contain no other bags.",
        "dotted black bags contain no other bags.",
    ];
    let mut b = Bags::new();
    for each in rules.iter() {
        b.insert_by_line(*each);
    }

    /* bright white bags can hold:
     * - shiny gold
     *   - dark olive
     *     - faded blue
     *     - dotted black
     *   - vibrant plum
     *     - faded blue
     *     - dotted black
     */
    assert_eq!(b.contents_recursive("bright white").unwrap().collect::<Vec<_>>(),
               vec![("shiny gold".to_string(), 1),
                    ("dark olive".to_string(), 1u32), ("dotted black".to_string(), 4), ("faded blue".to_string(), 3),
                    ("vibrant plum".to_string(), 2), ("dotted black".to_string(), 6), ("faded blue".to_string(), 5)]);
}

#[test]
fn test_part_2() {
    use std::io::Cursor;
    let rules = vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
        "bright white bags contain 1 shiny gold bag.",
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
        "faded blue bags contain no other bags.",
        "dotted black bags contain no other bags.",
    ];
    assert_eq!(part_2(Cursor::new(rules.join("\n")), "faded blue").unwrap(), 0);
    assert_eq!(part_2(Cursor::new(rules.join("\n")), "dotted black").unwrap(), 0);
    assert_eq!(part_2(Cursor::new(rules.join("\n")), "vibrant plum").unwrap(), 11);
    assert_eq!(part_2(Cursor::new(rules.join("\n")), "dark olive").unwrap(), 7);
    assert_eq!(part_2(Cursor::new(rules.join("\n")), "shiny gold").unwrap(), 32);
}