use aoc22_rs::solutions::{day1, day2, day3};

fn main() {

    let day1_solution = day1::solution("./input_files/day1/input_day1.txt");
    println!("Max: {}", day1_solution);

    let day2_solution1 = day2::solution1("./input_files/day2/input_day2_1.txt");
    println!("Score: {}", day2_solution1);

    let day2_solution2 = day2::solution2("./input_files/day2/input_day2_2.txt");
    println!("Score: {}", day2_solution2);

    let day3_solution1 = day3::solution1("./input_files/day3/input_day3_1.txt");
    println!("Total: {}", day3_solution1);

    let day3_solution2 = day3::solution2("./input_files/day3/input_day3_2.txt");
    println!("Total: {}", day3_solution2);
}
