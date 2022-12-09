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
    overlapped: bool,
}

impl Elves {
    fn set_overlapped(&mut self) {
        if self.elf_pair[0].assigned_section.len() != self.elf_pair[1].assigned_section.len() {
            panic!("Mismatched containers size for elves");
        }
        let convoluted_vector = convolve(
            &self.elf_pair[0].assigned_section,
            &self.elf_pair[1].assigned_section,
        );
        let convoluted = convoluted_vector.as_slice();
        if !(convoluted == &self.elf_pair[0].assigned_section)
            || !(convoluted == &self.elf_pair[1].assigned_section)
        {
            return;
        }
        self.overlapped = true;
    }
}

fn convolve<'a>(a: &'a [bool], b: &'a [bool]) -> Vec<bool> {
    // This should return a vector equal to the slice (a or b)
    // if there's a complete overlap
    // if no overlap neither of (a or b) will be equal to the output.
    let mut result = a.clone().to_vec();
    for i in 0..a.len() {
        let element = a[i] && b[i];
        result[i] = element;
    }
    result
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
            current_pair.set_overlapped();
            if !current_pair.overlapped {
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
fn test_convolve() {
    let a = [false, true, true, true, true, true, true, true, false];
    let b = [false, false, true, true, true, true, true, false, false];
    let result_conv = convolve(&a, &b);
    let convoluted = result_conv.as_slice();
    assert_eq!(&b, convoluted);

    let a = [false, true, true, true, false, true, true, true, false];
    let b = [false, false, true, true, true, true, true, false, false];
    let convol_vec = convolve(&a, &b);
    let convoluted = convol_vec.as_slice();

    assert_ne!(&a, convoluted);
    assert_ne!(&b, convoluted);
}
