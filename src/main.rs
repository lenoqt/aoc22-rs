use aoc22_rs::solutions::{day1, day2, day3, day4};

fn main() {
    let day1_solution = day1::solution("./input_files/day1/input_day1.txt");
    println!("Solution day 1: {}", day1_solution);

    let day2_solution1 = day2::solution1("./input_files/day2/input_day2_1.txt");
    println!("Solution day 2-1: {}", day2_solution1);

    let day2_solution2 = day2::solution2("./input_files/day2/input_day2_2.txt");
    println!("Solution day 2-2: {}", day2_solution2);

    let day3_solution1 = day3::solution1("./input_files/day3/input_day3_1.txt");
    println!("Solution day 3-1: {}", day3_solution1);

    let day3_solution2 = day3::solution2("./input_files/day3/input_day3_2.txt");
    println!("Solution day 3-2: {}", day3_solution2);

    let day4_solution1 = day4::solution1("./input_files/day4/input_day4_1.txt");
    println!("Solution day 4-1: {}", day4_solution1);

    let day4_solution2 = day4::solution2("./input_files/day4/input_day4_1.txt");
    println!("Solution day 4-2: {}", day4_solution2);
}
