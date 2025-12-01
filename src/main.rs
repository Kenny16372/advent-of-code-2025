use std::fmt::Debug;

#[derive(Debug)]
enum Rotation {
    Left,
    Right,
}

impl From<&str> for Rotation {
    fn from(value: &str) -> Self {
        match value {
            "L" => Rotation::Left,
            "R" => Rotation::Right,
            _ => unreachable!("invalid rotation"),
        }
    }
}

#[derive(Debug)]
struct Step {
    rotation: Rotation,
    distance: i32,
}

impl From<&str> for Step {
    fn from(value: &str) -> Self {
        let (r, v) = value.split_at(1);
        Step {
            rotation: r.into(),
            distance: v.parse::<i32>().expect("Should be a number"),
        }
    }
}

fn star_1(steps: Vec<Step>) -> i32 {
    fn helper(steps: &[Step], acc: (i32, i32)) -> i32 {
        match steps.get(0) {
            None => acc.1,
            Some(step) => {
                let sign = match step.rotation {
                    Rotation::Left => -1,
                    Rotation::Right => 1,
                };
                let distance = step.distance * sign;
                let value_new = (acc.0 + distance) % 100;
                helper(
                    &steps[1..],
                    (value_new, acc.1 + if value_new == 0 { 1 } else { 0 }),
                )
            }
        }
    }
    helper(&steps, (50, 0))
}

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").expect("Should be able to read input");
    let steps: Vec<Step> = contents
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.into())
        .collect();
    println!("steps: {:?}", steps);
    println!("star_1: {:?}", star_1(steps));
}
