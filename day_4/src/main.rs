use std::time::SystemTime;

#[derive(Debug)]
struct Warehouse {
    rolls: Vec<Vec<Option<()>>>,
}

impl From<&str> for Warehouse {
    fn from(value: &str) -> Self {
        Self {
            rolls: value
                .trim()
                .split_ascii_whitespace()
                .map(|row| {
                    row.chars()
                        .map(|c| if c == '@' { Some(()) } else { None })
                        .collect()
                })
                .collect(),
        }
    }
}

impl Warehouse {
    fn star_1(&self) -> i64 {
        let width = self.rolls[0].len();
        let height = self.rolls.len();
        (0..width)
            .flat_map(|x| (0..height).map(move |y| (x, y)))
            .filter(|&(x, y)| self.rolls[y][x].is_some())
            .map(|pos| (pos, self.count_3x3_hollow(pos)))
            .filter(|&(_, count)| count < 4)
            .count() as i64
    }

    fn count_3x3_hollow(&self, pos: (usize, usize)) -> i64 {
        let x = pos.0 as i32;
        let y = pos.1 as i32;
        ((x - 1)..=(x + 1))
            .flat_map(|x_offset| ((y - 1)..=(y + 1)).map(move |y_offset| (x_offset, y_offset)))
            .filter_map(|pos| match pos {
                p if p == (x, y) => None,
                p if p.0 < 0 || p.1 < 0 => None,
                _ => Some((pos.0 as usize, pos.1 as usize)),
            })
            .filter_map(|(x, y)| self.rolls.get(y).and_then(|row| row.get(x)))
            .filter(|&&cell| cell.is_some())
            .count() as i64
    }

    fn star_2(&self) -> i64 {
        0
    }
}

fn main() {
    let content = std::fs::read_to_string("data/input.txt").expect("should be able to read file");
    let warehouse: Warehouse = content.as_str().into();

    let star_1_start = SystemTime::now();
    let star_1: i64 = warehouse.star_1();
    let star_1_duration = SystemTime::now().duration_since(star_1_start).unwrap();
    println!("{star_1} {:?}", star_1_duration);

    let star_2_start = SystemTime::now();
    let star_2: i64 = warehouse.star_2();
    let star_2_duration = SystemTime::now().duration_since(star_2_start).unwrap();
    println!("{star_2} {:?}", star_2_duration);
}
