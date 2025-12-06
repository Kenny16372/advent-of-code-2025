use std::time::SystemTime;

// Source - https://stackoverflow.com/a/64499219
// Posted by Netwave, modified by community. See post 'Timeline' for change history
// Retrieved 2025-12-06, License - CC BY-SA 4.0

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[derive(Debug, Clone)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug, Clone)]
struct Question {
    op: Op,
    numbers: Vec<i64>,
}

impl Question {
    fn solve(&self) -> i64 {
        match self.op {
            Op::Add => self.numbers.iter().fold(0, |a, &b| a + b),
            Op::Mul => self.numbers.iter().fold(1, |a, &b| a * b),
        }
    }
}

#[derive(Debug, Clone)]
struct Homework {
    questions: Vec<Question>,
}

impl From<&str> for Homework {
    fn from(value: &str) -> Self {
        let spreadsheet: Vec<Vec<&str>> = value
            .lines()
            .map(|l| l.split_ascii_whitespace().collect())
            .collect();
        let transposed = transpose(spreadsheet);
        Self {
            questions: transposed
                .iter()
                .map(|q| {
                    let numbers: Vec<i64> = q
                        .iter()
                        .take(q.len() - 1)
                        .map(|s| s.parse().expect("should be a number"))
                        .collect();
                    let op = match *q.last().expect("should not be empty") {
                        "*" => Op::Mul,
                        "+" => Op::Add,
                        _ => unreachable!(),
                    };
                    Question { numbers, op }
                })
                .collect(),
        }
    }
}

impl Homework {
    fn star_1(&self) -> i64 {
        self.questions.iter().map(|q| q.solve()).sum()
    }

    fn star_2(&self) -> i64 {
        0
    }
}

fn main() {
    let content = std::fs::read_to_string("data/input.txt").expect("should be able to read file");
    let homework: Homework = content.as_str().into();

    let star_1_start = SystemTime::now();
    let star_1: i64 = homework.star_1();
    let star_1_duration = SystemTime::now().duration_since(star_1_start).unwrap();
    println!("{star_1} {:?}", star_1_duration);

    let star_2_start = SystemTime::now();
    let star_2: i64 = homework.star_2();
    let star_2_duration = SystemTime::now().duration_since(star_2_start).unwrap();
    println!("{star_2} {:?}", star_2_duration);
}
