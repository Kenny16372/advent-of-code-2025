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

fn main() {
    let content = std::fs::read_to_string("data/input.txt").expect("should be able to read file");
    let ranges: Vec<Range> = content.trim().split(',').map(|s| s.into()).collect();
    println!("{:?}", ranges);
    let star_1: i64 = ranges
        .into_iter()
        .flat_map(|range| range.0.filter(|&v| !is_valid(v)))
        .sum();
    println!("{star_1}");
}
