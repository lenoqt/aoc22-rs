use crate::solutions::utils::read_lines;

/*
Draw = 3
Win = 6
A, X: Rock = 1 ,
B, Y: Paper = 2,
C, Z: Scissors = 3,
 */

/*
X = Lose
Y = Draw
Z = Win
 */
pub fn draw_strategy(play_a: char, result: char) -> char {
    match (play_a, result) {
        ('A', 'X') => 'Z',
        ('A', 'Y') => 'X',
        ('A', 'Z') => 'Y',
        ('B', 'X') => 'X',
        ('B', 'Y') => 'Y',
        ('B', 'Z') => 'Z',
        ('C', 'X') => 'Y',
        ('C', 'Y') => 'Z',
        ('C', 'Z') => 'X',
        _ => panic!("Invalid"),
    }
}

pub fn compute_score(play_a: char, play_b: char) -> i32 {
    match (play_a, play_b) {
        ('A', 'X') => 4, // Rock vs Rock
        ('A', 'Y') => 8, // Rock vs Paper
        ('A', 'Z') => 3, // Rock vs Scissors
        ('B', 'X') => 1, // Paper vs Rock
        ('B', 'Y') => 5, // Paper vs Paper
        ('B', 'Z') => 9, // Paper vs Scissors
        ('C', 'X') => 7, // Scissors vs Rock
        ('C', 'Y') => 2, // Scissors vs Paper
        ('C', 'Z') => 6, // Scissors vs Scissors
        _ => panic!("Invalid"),
    }
}

pub fn solution1(input_path: &str) -> i32 {
    let mut total_score =  0;
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            let  (play_a, play_b) = line
                .as_deref()
                .expect("Can't read line")
                .split_once(" ")
                .unwrap();
            let score = compute_score(
                     play_a
                         .chars()
                         .next()
                         .expect("Empty"),
                     play_b
                         .chars()
                         .next()
                         .expect("Empty"));
            total_score += score;
        }
    } else {
        panic!("Can't read input file");
    }
    total_score
}

pub fn solution2(input_path: &str) -> i32 {
    let mut total_score =  0;
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            let  (play_a, result) = line
                .as_deref()
                .expect("Can't read line")
                .split_once(" ")
                .unwrap();
            let play_b = draw_strategy(
                play_a
                    .chars()
                    .next()
                    .expect("Empty string"),
                result
                    .chars()
                    .next()
                    .expect("Empty string")
            );
            total_score += compute_score(play_a
                                             .chars()
                                             .next()
                                             .expect("Empty string"),
                                         play_b);
        }
    }
    total_score
}

#[test]
fn test_solution1() {
    let input_path = "./input_files/day2/input_example1.txt";
    let solution = solution1(input_path);
    assert_eq!(solution, 15);
}

#[test]
fn test_solution2() {
    let input_path = "./input_files/day2/input_example2.txt";
    let solution = solution2(input_path);
    assert_eq!(solution, 12);
}