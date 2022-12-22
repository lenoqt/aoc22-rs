use crate::solutions::utils::read_lines;

#[derive(Debug, Clone)]
struct Elf {
    id: i32,
    assigned_section: [bool; 99],
}

impl Default for Elf {
    fn default() -> Self {
        Self {
            id: 0,
            assigned_section: [false; 99],
        }
    }
}

impl Elf {
    fn fill_assigned_range(&mut self, range: &str) {
        if let Some((low, high)) = range.split_once('-') {
            let range = std::ops::RangeInclusive::new(
                low.parse::<i32>().unwrap() - 1,
                high.parse::<i32>().unwrap() - 1,
            );
            let mut result = [false; 99];
            for i in range {
                result[i as usize] = true;
            }
            self.assigned_section = result;
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Elves {
    id: i32,
    elf_pair: [Elf; 2],
    full_overlapped: bool,
    partially_overlapped: bool, // Better to combine these two in 1 variable i.e 0,1,2 or a hex value.
}

impl Elves {
    fn is_fully_overlapped(&mut self) {
        if self.elf_pair[0].assigned_section.len() != self.elf_pair[1].assigned_section.len() {
            panic!("Mismatched containers size for elves");
        }
        if self.elf_pair[0].assigned_section == self.elf_pair[1].assigned_section {
            self.full_overlapped = true;
            return;
        }
        let filtered_vector = filter_array(
            &self.elf_pair[0].assigned_section,
            &self.elf_pair[1].assigned_section,
        );
        let filtered = filtered_vector.as_slice();

        if filtered == self.elf_pair[0].assigned_section
            || filtered == self.elf_pair[1].assigned_section
        {
            self.full_overlapped = true;
            return;
        }
        self.full_overlapped = false;
    }

    fn is_partially_overlapped(&mut self) {
        if self.elf_pair[0].assigned_section.len() != self.elf_pair[1].assigned_section.len() {
            panic!("Mismatched containers size for elves");
        }
        if self.elf_pair[0].assigned_section == self.elf_pair[1].assigned_section {
            self.full_overlapped = true;
            self.partially_overlapped = true;
            return;
        }
        let filtered_vector = filter_array(
            &self.elf_pair[0].assigned_section,
            &self.elf_pair[1].assigned_section,
        );
        let filtered = filtered_vector.as_slice();
        if filtered.iter().any(|x| *x) {
            self.partially_overlapped = true;
            return;
        }
        self.full_overlapped = false;
        self.partially_overlapped = false;
    }
}

fn filter_array<'a>(a: &'a [bool], b: &'a [bool]) -> Vec<bool> {
    // This should return a vector equal to the slice (a or b)
    // if there's a complete overlap
    // if no overlap neither of (a or b) will be equal to the output.
    let mut result = Vec::new();
    // Just cloning here will clone the reference and not the inner type.
    a.to_vec().clone_into(&mut result);
    for i in 0..a.len() {
        let element = a[i] && b[i];
        result[i] = element;
    }
    result
}

#[allow(dead_code)]
fn graph_containers(a: &[bool], b: &[bool]) {
    for x in a.iter() {
        let mut c = ".";
        if *x {
            c = "i";
        }
        print!("{}", c);
    }
    println!();
    for y in b.iter() {
        let mut c = ".";
        if *y {
            c = "i";
        }
        print!("{}", c);
    }
    println!();
}

pub fn solution1(input_path: &str) -> i32 {
    let mut total = 0;
    if let Ok(lines) = read_lines(input_path) {
        for (n, line) in lines.enumerate() {
            let current_pair = &mut Elves::default();
            current_pair.id = n as i32;
            if let Some((elf_a_range, elf_b_range)) = line.unwrap().split_once(',') {
                current_pair.elf_pair[0].id = 0;
                current_pair.elf_pair[1].id = 1;

                current_pair.elf_pair[0].fill_assigned_range(elf_a_range);
                current_pair.elf_pair[1].fill_assigned_range(elf_b_range);
            }
            current_pair.is_fully_overlapped();
            if !current_pair.full_overlapped {
                continue;
            }
            total += 1;
        }
    } else {
        panic!("Can't read input file");
    }
    total
}

pub fn solution2(input_path: &str) -> i32 {
    let mut total = 0;
    if let Ok(lines) = read_lines(input_path) {
        for (n, line) in lines.enumerate() {
            let current_pair = &mut Elves::default();
            current_pair.id = n as i32;
            if let Some((elf_a_range, elf_b_range)) = line.unwrap().split_once(',') {
                current_pair.elf_pair[0].id = 0;
                current_pair.elf_pair[1].id = 1;

                current_pair.elf_pair[0].fill_assigned_range(elf_a_range);
                current_pair.elf_pair[1].fill_assigned_range(elf_b_range);
            }
            current_pair.is_partially_overlapped();
            if !current_pair.partially_overlapped {
                continue;
            }
            total += 1;
        }
    } else {
        panic!("Can't read input file");
    }
    total
}

#[test]
fn test_solution1() {
    let input_path = "./input_files/day4/input_example1.txt";
    let result = solution1(input_path);
    assert_eq!(2, result);
}

#[test]
fn test_solution2() {
    let input_path = "./input_files/day4/input_example1.txt";
    let result = solution2(input_path);
    assert_eq!(4, result);
}

#[test]
fn test_fill_range() {
    let elf = &mut Elves::default();
    let elf_a_range = "2-8";
    let elf_b_range = "3-7";
    elf.elf_pair[0].fill_assigned_range(elf_a_range);
    elf.elf_pair[1].fill_assigned_range(elf_b_range);
    let array_a = [
        false, true, true, true, true, true, true, true, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false,
    ];
    let array_b = [
        false, false, true, true, true, true, true, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false,
    ];

    assert_eq!(array_a, elf.elf_pair[0].assigned_section);
    assert_eq!(array_b, elf.elf_pair[1].assigned_section);
}

#[test]
fn test_full_overlapping() {
    let elf = &mut Elves::default();
    let elf_a_range = "2-8";
    let elf_b_range = "3-7";
    elf.elf_pair[0].fill_assigned_range(elf_a_range);
    elf.elf_pair[1].fill_assigned_range(elf_b_range);
    let mut array_a = [false; 99];
    let mut array_b = [false; 99];
    let fill_values = |x: &str, arr: &mut [bool; 99]| {
        let v: Vec<&str> = x.split('-').collect();
        let start = v[0].parse::<usize>().unwrap() - 1;
        let end = v[1].parse::<usize>().unwrap() - 1;
        for i in start..=end {
            arr[i] = true;
        }
    };
    fill_values(elf_a_range, &mut array_a);
    fill_values(elf_b_range, &mut array_b);
    elf.is_fully_overlapped();

    assert!(elf.full_overlapped);
}

#[test]
fn test_partial_overlapping() {
    let elf = &mut Elves::default();
    let elf_a_range = "5-7";
    let elf_b_range = "7-9";
    elf.elf_pair[0].fill_assigned_range(elf_a_range);
    elf.elf_pair[1].fill_assigned_range(elf_b_range);
    let mut array_a = [false; 99];
    let mut array_b = [false; 99];
    let fill_values = |x: &str, arr: &mut [bool; 99]| {
        let v: Vec<&str> = x.split('-').collect();
        let start = v[0].parse::<usize>().unwrap() - 1;
        let end = v[1].parse::<usize>().unwrap() - 1;
        for i in start..=end {
            arr[i] = true;
        }
    };
    fill_values(elf_a_range, &mut array_a);
    fill_values(elf_b_range, &mut array_b);
    elf.is_partially_overlapped();

    assert!(elf.partially_overlapped);
}

#[test]
fn test_filter() {
    let a = [false, true, true, true, true, true, true, true, false];
    let b = [false, false, true, true, true, true, true, false, false];
    let result_filter = filter_array(&a, &b);
    let filtered = result_filter.as_slice();
    assert_eq!(&b, filtered);

    let a = [false, true, true, true, false, true, true, true, false];
    let b = [false, false, true, true, true, true, true, false, false];
    let filter_vec = filter_array(&a, &b);
    let filtered_ne = filter_vec.as_slice();

    assert_ne!(&a, filtered_ne);
    assert_ne!(&b, filtered_ne);
}
