use anyhow::{bail, Result};

use crate::solutions::utils::read_lines;

#[derive(Debug, Default, PartialEq)]
struct ShipGrid {
    stack_name: Row,
    stacks: Vec<Stack>,
    rows: Vec<Row>,
    dimensions: (usize, usize),
}

trait Move {
    fn move_one(&mut self, from: usize, to: usize);
    fn move_many(&mut self, from: usize, to: usize);
}

#[derive(Debug, Default, PartialEq)]
struct Stack(Vec<Box<Crate>>);

#[derive(Debug, PartialEq, Clone, Default)]
struct Row(Vec<Crate>);

impl Row {
    fn new() -> Row {
        Row(Vec::new())
    }

    fn add(&mut self, elem: Crate) {
        self.0.push(elem);
    }
}

impl FromIterator<Crate> for Row {
    fn from_iter<T: IntoIterator<Item = Crate>>(iter: T) -> Self {
        let mut r = Row::new();
        for i in iter {
            r.add(i)
        }
        r
    }
}

impl std::ops::Index<usize> for Row {
    type Output = Crate;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Crate {
    Item(char),
    Nil,
}

impl Default for Crate {
    fn default() -> Self {
        Crate::Nil
    }
}

fn build_stacks(grid_vectors: &Vec<Row>) -> Vec<Stack> {
    let mut stacks: Vec<Stack> = vec![];

    for n in 0..grid_vectors.len() - 1 {
        let mut stack = Stack::default();
        for m in 0..grid_vectors.len() - 1 {
            stack.0.push(Box::new(grid_vectors[m][n]));
        }
        stacks.push(stack);
    }
    stacks
}

fn make_ship_grid(grid_vectors: Vec<Row>) -> Result<ShipGrid> {
    let number_rows = grid_vectors.len() - 1;
    let number_columns = grid_vectors[0].0.len();
    let mut vector_rows = grid_vectors.clone();
    let stacks = build_stacks(&grid_vectors);
    let ship = ShipGrid {
        stack_name: vector_rows.pop().unwrap(),
        stacks: stacks,
        rows: vector_rows,
        dimensions: (number_rows, number_columns),
    };
    Ok(ship)
}

fn parse_input_grid(input_file: &str) -> Result<Vec<Row>> {
    let mut grid: Vec<Row> = vec![];
    let lines = read_lines(input_file)?;
    for line in lines.into_iter() {
        let row = line.unwrap().to_owned();
        if row == "".to_string() {
            break;
        }
        let grid_row: Row = row
            .chars()
            .enumerate()
            .filter_map(|(i, s)| {
                if i % 4 == 1 {
                    match s {
                        ' ' => Some(Crate::Nil),
                        _ => Some(Crate::Item(s)),
                    }
                } else {
                    None
                }
            })
            .collect();
        grid.push(grid_row);
    }
    Ok(grid)
}

struct Instructions {
    lines: Vec<String>,
    index: usize,
}

impl Iterator for Instructions {
    type Item = (i16, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.lines.len() {
            let line = &self.lines[self.index];
            self.index += 1;

            let (_, amount, from, to) =
                parse_instructions(line).expect("Could not parse instruction");
            {
                return Some((amount, from, to));
            }
        }
        None
    }
}

fn parse_instructions(ins: &str) -> Result<(String, i16, usize, usize)> {
    let parts: Vec<&str> = ins.split(" ").collect();
    if parts.len() != 6 || parts[0] != "move" {
        bail!("Invalid line: {:?}", parts);
    }
    let amount = parts[1]
        .parse::<i16>()
        .expect("Could not parse `amount` to i16");
    let from = parts[3]
        .parse::<usize>()
        .expect("Could not parse `from` to usize");
    let to = parts[5]
        .parse::<usize>()
        .expect("Could not parse `to` to usize");

    Ok((parts[0].to_string(), amount, from, to))
}

pub fn solution1(input_file: &str) -> Result<String> {
    let grid = parse_input_grid(input_file)?;
    let ship = make_ship_grid(grid).expect("Couldn't build ship");

    Ok("".to_string())
}

#[test]
fn test_solution1() {
    let input_file = "./input_files/day5/input_example1.txt";
    let result = solution1(input_file).expect("Could not process solution.");

    assert_eq!(result, "CMZ");
}

#[test]
fn test_parse_grid() {
    let input_file = "./input_files/day5/input_example1.txt";
    let grid = parse_input_grid(input_file).expect("Can't parse file");
    let expected = vec![
        Row(vec![Crate::Nil, Crate::Item('D'), Crate::Nil]),
        Row(vec![Crate::Item('N'), Crate::Item('C'), Crate::Nil]),
        Row(vec![Crate::Item('Z'), Crate::Item('M'), Crate::Item('P')]),
        Row(vec![Crate::Item('1'), Crate::Item('2'), Crate::Item('3')]),
    ];

    assert_eq!(grid, expected);
}

#[test]
fn test_init_ship() {
    let input_file = "./input_files/day5/input_example1.txt";
    let grid = parse_input_grid(input_file).expect("Can't parse file");
    let ship = make_ship_grid(grid).expect("Couldn't build ship");
    let expected = ShipGrid {
        stack_name: Row(vec![Crate::Item('1'), Crate::Item('2'), Crate::Item('3')]),
        stacks: vec![
            Stack(vec![
                Box::new(Crate::Nil),
                Box::new(Crate::Item('N')),
                Box::new(Crate::Item('Z')),
            ]),
            Stack(vec![
                Box::new(Crate::Item('D')),
                Box::new(Crate::Item('C')),
                Box::new(Crate::Item('M')),
            ]),
            Stack(vec![
                Box::new(Crate::Nil),
                Box::new(Crate::Nil),
                Box::new(Crate::Item('P')),
            ]),
        ],
        rows: vec![
            Row(vec![Crate::Nil, Crate::Item('D'), Crate::Nil]),
            Row(vec![Crate::Item('N'), Crate::Item('C'), Crate::Nil]),
            Row(vec![Crate::Item('Z'), Crate::Item('M'), Crate::Item('P')]),
        ],
        dimensions: (3, 3),
    };

    assert_eq!(ship, expected);
}

#[test]
fn test_parse_instruction() {
    let (s, amount, from, to) =
        parse_instructions("move 1 from 3 to 5").expect("Could not parse instruction");

    assert_eq!(s, "move".to_string());
    assert_eq!(amount, 1);
    assert_eq!(from, 3);
    assert_eq!(to, 5);
}

#[test]
fn test_instructions_iterator() {
    let mut instructions = Instructions {
        lines: vec![
            "move 1 from 3 to 5".to_string(),
            "move 2 from 1 to 5".to_string(),
            "move 3 from 2 to 4".to_string(),
        ],
        index: 0,
    };

    assert_eq!(instructions.next(), Some((1, 3, 5)));
    assert_eq!(instructions.next(), Some((2, 1, 5)));
    assert_eq!(instructions.next(), Some((3, 2, 4)));
    assert_eq!(instructions.next(), None);
}
