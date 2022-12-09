use crate::solutions::utils::read_lines;

const ALPHABET1: &str = "abcdefghijklmnopqrstuvwxyz";
const ALPHABET2: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Debug, Default)]
struct Elf {
    id: i32,
    items: Vec<char>,
}

impl Elf {
    fn is_bag_filled(&self) -> bool {
        !self.items.is_empty()
    }
}

#[derive(Debug, Default)]
struct Elves {
    id: i32,
    badge: Option<char>,
    members: [Elf; 3],
    total: i32,
}

impl Elves {

    fn all_bags_filled(&self) -> bool {
        self.members.iter().all(|x| x.is_bag_filled())
    }

    fn set_badge(&mut self) {
        for a in self.members[0].items.iter() {
            if self.members[1].items.contains(a)
                && self.members[2].items.contains(a)
            {
                self.badge = Some(*a);
            }
        }
    }

    fn compute_total(&mut self) {
        if self.badge.unwrap().is_lowercase() {
            self.total = ALPHABET1
                .chars()
                .position(
                    |r| r == self.badge.unwrap()
                )
                .unwrap() as i32 + 1
        } else {
            self.total = ALPHABET2
                .chars()
                .position(
                    |r| r == self.badge.unwrap()
                )
                .unwrap() as i32 + 27
        }
    }
}

pub fn solution1(input_path: &str) -> i32 {
    let mut total_score: i32 = 0;
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            let rucksack: Vec<char> = line.unwrap().chars().collect();
            let mid = &rucksack.len() / 2;
            let left = &mut rucksack[..mid].to_vec();
            left.sort();
            let right = &mut rucksack[mid..].to_vec();
            right.sort();
            let mut repeated = None;
            for a in left.iter() {
                if right.contains(a) {
                    repeated = Some(a);
                }
            }
            if repeated.unwrap().is_lowercase() {
                total_score += ALPHABET1
                    .chars()
                    .position(
                        |r| r == *repeated.unwrap()
                    )
                    .unwrap() as i32 + 1;
            } else {
                total_score += ALPHABET2
                    .chars()
                    .position(
                        |r| r == *repeated.unwrap()
                    )
                    .unwrap() as i32 + 27;
            }
        }
    } else {
        panic!("Can't read file");
    }
    total_score
}

pub fn solution2(input_path: &str) -> i32 {
    let mut total = 0;
    let mut group = Elves::default();
    if let Ok(lines) = read_lines(input_path) {
        for (n, line) in lines.enumerate() {
            if n % 3 == 0 {
                group.id = (n as i32 / 3) + 1;
            }
            let current_group = &mut group;
            let members = n % 3;
            match members {
                0 => {
                    current_group.members[0].id = 1;
                    current_group.members[0].items = line
                        .unwrap()
                        .chars()
                        .collect();
                }
                1 => {
                    current_group.members[1].id = 2;
                    current_group.members[1].items = line
                        .unwrap()
                        .chars()
                        .collect();
                }
                2 => {
                    current_group.members[2].id = 3;
                    current_group.members[2].items = line
                        .unwrap()
                        .chars()
                        .collect();
                }
                _ => unreachable!()
            }
            current_group.set_badge();
            if current_group.all_bags_filled() {
                current_group.compute_total();
                total += current_group.total;
                group = Elves::default();
            }
        }
    }
    total
}

#[test]
fn test_solution1() {
    let input_path = "./input_files/day3/input_example1.txt";
    let solution = solution1(input_path);
    assert_eq!(solution, 157);
}

#[test]
fn test_solution2() {
    let input_path = "./input_files/day3/input_example2.txt";
    let solution = solution2(input_path);
    assert_eq!(solution, 70);
}