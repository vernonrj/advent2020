use std::collections::{VecDeque, HashMap};

fn main() {
    let input: Vec<i64> = include_str!("../input.txt").lines().map(|line| line.parse().unwrap()).collect();
    let mut window = CipherWindow::new(25);
    for value in input.iter() {
        match window.insert(*value) {
            Ok(_) => (),
            Err(_) => println!("couldn't insert value {}", value),
        }
    }
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