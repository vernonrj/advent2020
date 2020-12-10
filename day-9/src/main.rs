use std::collections::{VecDeque, HashMap};
use std::error::Error;

fn main() {
    let input: Vec<i64> = include_str!("../input.txt").lines().map(|line| line.parse().unwrap()).collect();
    let invalid_number = find_invalid_number(25, &input).unwrap();
    println!("couldn't insert value {}", invalid_number);
    let cypher_break = find_contiguous_sum(invalid_number, &input).unwrap();
    println!("cypher weakness: {}", cypher_break);
}

pub fn find_invalid_number(window_size: usize, input: &[i64]) -> Result<i64, Box<dyn Error>> {
    let mut window = CipherWindow::new(window_size);
    for value in input.iter() {
        match window.insert(*value) {
            Ok(_) => (),
            Err(_) => {
                return Ok(*value);
            }
        }
    }
    Err(format!("All numbers were valid").into())
}

pub fn find_contiguous_sum(value_to_find: i64, input: &[i64]) -> Result<i64, Box<dyn Error>> {
    for winsize in 2..input.len() {
        let found = input.windows(winsize)
            .filter(|window| (*window).iter().sum::<i64>() == value_to_find)
            .next();
        
        match found {
            Some(n) => {
                let mut v = Vec::from(n);
                v.sort();
                return Ok(v.first().unwrap() + v.last().unwrap());
            }
            None => (),
        }
    }
    Err(format!("couldn't find it").into())
}

pub struct CipherWindow {
    window: VecDeque<i64>,
    window_size: usize,
}

pub enum InsertError {
    NotASum,
}

impl CipherWindow {
    pub fn new(window_size: usize) -> Self {
        Self {
            window: VecDeque::new(),
            window_size
        }
    }
    pub fn insert(&mut self, value: i64) -> Result<(), InsertError> {
        if self.window.len() < self.window_size {
            self.window.push_back(value);
            return Ok(());
        }
        let set = self.window.iter()
            .fold(HashMap::new(), |mut m, elem| {
                let e = m.entry(*elem).or_insert(0);
                *e += 1;
                m
            });
        let mut found = false;
        for each_value in self.window.iter() {
            let supposed_second_value = value - *each_value;
            let is_match = match set.get(&supposed_second_value) {
                Some(v) if value == *each_value => *v > 1,
                Some(_) => true,
                None => false,
            };
            if is_match {
                found = true;
                break;
            }
        }
        if found {
            self.window.push_back(value);
            self.window.pop_front();
            Ok(())
        } else {
            Err(InsertError::NotASum)
        }
    }
}