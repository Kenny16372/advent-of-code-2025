use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug)]
struct BatteryBank {
    batteries: Vec<i8>,
}

impl From<&str> for BatteryBank {
    fn from(value: &str) -> Self {
        Self {
            batteries: value
                .chars()
                .map(|c| c.to_digit(10).expect("only numbers") as i8)
                .collect(),
        }
    }
}

impl BatteryBank {
    fn find_first_digit(&self) -> (i8, usize) {
        self.batteries
            .iter()
            .enumerate()
            .rev()
            .skip(1)
            .map(|(idx, &val)| (val, idx))
            .max_by_key(|&(val, _)| val)
            .expect("shouldn't be empty")
    }

    fn find_second_digit(&self, start_idx: usize) -> i8 {
        *self.batteries[(start_idx + 1)..]
            .iter()
            .max()
            .expect("shouldn't be empty")
    }

    fn star_1(&self) -> i64 {
        let (first, idx) = self.find_first_digit();
        let second = self.find_second_digit(idx);
        println!("{first} {second} {:?}", self);
        first as i64 * 10 + second as i64
    }

    fn star_2(&self) -> i64 {
        let mut result = Vec::with_capacity(12);
        let mut start_idx = 0;
        let mut skip_max = self.batteries.len() - 12;
        while result.len() != result.capacity() {
            let (start_idx_new, skip_max_new, value) = self.next_idx_skip_max(start_idx, skip_max);
            start_idx = start_idx_new;
            skip_max = skip_max_new;
            result.push(value as i64);
        }
        println!("result vec: {:?}", result);

        result.iter().fold(0, |a, b| a * 10 + b)
    }

    fn next_idx_skip_max(&self, start_idx: usize, skip_max: usize) -> (usize, usize, i8) {
        // println!("{start_idx} {skip_max} {:?}", self.batteries);
        self.batteries[start_idx..=(start_idx + skip_max)]
            .iter()
            .enumerate()
            .rev()
            .map(|(idx, &val)| (start_idx + idx + 1, skip_max - idx, val))
            .max_by_key(|&(_, _, val)| val)
            .expect("shouldn't be empty")
    }
}

fn main() {
    let content = std::fs::read_to_string("data/input.txt").expect("should be able to read file");
    let banks: Vec<BatteryBank> = content
        .trim()
        .split_ascii_whitespace()
        .map(|l| l.into())
        .collect();
    let star_1_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let star_1: i64 = banks.iter().map(|b| b.star_1()).sum();
    let star_1_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{star_1} {:?}", star_1_end - star_1_start);
    let star_2_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let star_2: i64 = banks.iter().map(|b| b.star_2()).sum();
    let star_2_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{star_2} {:?}", star_2_end - star_2_start);
}
