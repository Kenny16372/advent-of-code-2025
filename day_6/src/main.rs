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

impl From<Vec<String>> for Question {
    fn from(value: Vec<String>) -> Self {
        let number_length = value.len() - 1;
        let number_count = value[0].len();
        let mut numbers = Vec::with_capacity(number_count);
        for col in 0..number_count {
            let mut acc = String::with_capacity(number_length);
            for row in 0..number_length {
                acc.push(value[row].chars().nth(col).expect("shouldn't be empty"));
            }
            numbers.push(
                acc.chars()
                    .filter(|c| !c.is_ascii_whitespace())
                    .collect::<String>()
                    .parse()
                    .expect("should be a number"),
            )
        }
        let op = match value
            .last()
            .expect("shouldn't be empty")
            .chars()
            .next()
            .expect("shouldn't be empty")
        {
            '+' => Op::Add,
            '*' => Op::Mul,
            _ => unreachable!(),
        };
        Self { numbers, op }
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
        self.questions.iter().map(|q| q.solve()).sum()
    }

    fn new_star_2(value: &str) -> Self {
        let lines: Vec<Vec<char>> = value.lines().map(|l| l.chars().collect()).collect();
        let mut accs = vec![String::new(); lines.len()];
        let mut questions = vec![];

        for col in 0..lines[0].len() {
            if lines.iter().all(|l| l[col].is_ascii_whitespace()) {
                questions.push(accs.iter().map(|s| s.clone()).collect::<Vec<_>>().into());
                accs.iter_mut().for_each(|s| s.clear());
                continue;
            }

            for row in 0..lines.len() {
                accs[row].push(lines[row][col]);
            }
        }
        questions.push(accs.iter().map(|s| s.clone()).collect::<Vec<_>>().into());

        Self { questions }
    }
}

fn main() {
    let content = std::fs::read_to_string("data/input.txt").expect("should be able to read file");

    let star_1_start = SystemTime::now();
    let homework: Homework = content.as_str().into();
    let star_1: i64 = homework.star_1();
    let star_1_duration = SystemTime::now().duration_since(star_1_start).unwrap();
    println!("{star_1} {:?}", star_1_duration);

    let star_2_start = SystemTime::now();
    let homework_star_2 = Homework::new_star_2(content.as_str());
    // println!("{:?}", homework_star_2);
    let star_2: i64 = homework_star_2.star_2();
    let star_2_duration = SystemTime::now().duration_since(star_2_start).unwrap();
    println!("{star_2} {:?}", star_2_duration);
}
