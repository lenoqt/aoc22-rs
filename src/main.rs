use aoc22_rs::solutions::{day1, day2};

fn main() {

    let day1_solution = day1::solution("./input_files/day1/input_day1.txt");
    println!("Max: {}", day1_solution);

    let day2_solution1 = day2::solution1("./input_files/day2/input_day2_1.txt");
    println!("Score: {}", day2_solution1);

    let day2_solution2 = day2::solution2("./input_files/day2/input_day2_2.txt");
    println!("Score: {}", day2_solution2);
}
