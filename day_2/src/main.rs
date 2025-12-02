use std::{
    iter::repeat,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug)]
struct Range(std::ops::RangeInclusive<i64>);

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let (s, e) = value.split_once('-').expect("should contain dash");
        Self(s.parse().expect("start should be valid")..=e.parse().expect("end should be valid"))
    }
}

fn is_valid(value: i64) -> bool {
    let log10 = (value as f32).log10() as u32;
    let num_digits = log10 + 1;
    // println!("{value} {num_digits}");
    if num_digits % 2 != 0 {
        // println!("valid");
        return true;
    }
    let upper = value / 10i64.pow(num_digits / 2);
    let lower = value % 10i64.pow(num_digits / 2);
    let result = upper != lower;
    // println!("{upper} {lower} {result}");
    result
}

fn is_valid2(value: i64) -> bool {
    let log10 = (value as f32).log10() as u32;
    let num_digits = log10 + 1;
    // println!("{value} {num_digits}");
    let valid = !(1..=(num_digits / 2)).any(|repeat_length| {
        let divisor = 10i64.pow(repeat_length);
        let lower = value % divisor;
        if lower * 10 < divisor {
            return false;
        }
        let mut v = value;
        while v != 0 {
            let low = v % divisor;
            // println!("{lower} {low} {v} {value}");
            if low != lower {
                return false;
            }
            v = v / divisor
        }
        // println!("{value} {divisor} {lower}");
        true
    });
    // if !valid {
    //     println!("{value}");
    // }
    valid
}

fn main() {
    let content = std::fs::read_to_string("data/input.txt").expect("should be able to read file");
    let ranges: Vec<Range> = content.trim().split(',').map(|s| s.into()).collect();
    println!("{:?}", ranges);
    let star_1_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let star_1: i64 = ranges
        .iter()
        .flat_map(|range| range.0.clone().filter(|&v| !is_valid(v)))
        .sum();
    let star_1_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{star_1} {:?}", star_1_end - star_1_start);
    let star_2_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let star_2: i128 = ranges
        .iter()
        .flat_map(|range| range.0.clone().filter(|&v| !is_valid2(v)))
        .map(|v| v as i128)
        .sum();
    let star_2_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{star_2} {:?}", star_2_end - star_2_start);
}
