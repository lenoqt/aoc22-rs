use crate::solutions::utils::read_lines;

#[derive(Debug, Clone)]
pub struct Elf
{
    pub elf_id: i32,
    pub calories: i32,
}

impl Elf {
    pub fn new() -> Self {
        Elf { elf_id: 0, calories: 0 }
    }
    pub fn set_zero_calories(&mut self) {
        self.calories = 0;
    }
}

pub fn solution(input_path: &str) -> i32 {
    let elf = &mut Elf::new();
    let max_calories = &mut 0;
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(calories) = line.unwrap().trim().parse::<i32>() {
                elf.calories += calories;
            } else {
                if elf.calories > *max_calories {
                    *max_calories = elf.calories;
                }
                elf.elf_id = elf.elf_id + 1;
                elf.set_zero_calories();
                continue;
            }
        }
    } else {
        panic!("Can't find file")
    }
    *max_calories
}


#[test]
fn test_solution() {
    let input_path = "./input_files/day1/input_example1.txt";
    let max_calories = solution(input_path);
    assert_eq!(max_calories, 24000);
}