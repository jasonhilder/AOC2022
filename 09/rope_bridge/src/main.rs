use std::fs::File;
use std::io::{self, prelude::*, BufReader, Lines};

#[derive(Debug)]
pub struct Cell {
    x: usize,
    y: usize,
    passed: bool,
}

#[derive(Debug)]
pub struct Grid {
    _width: usize,
    _height: usize,
    head_pos: (usize, usize),
    tail_pos: (usize, usize),
    items: Vec<Cell>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        let mut v = Vec::new();
        for y in 0..=size {
            for x in 0..=size {
                v.push(Cell {
                    x,
                    y,
                    passed: false,
                })
            }
        }

        Self {
            _width: size,
            _height: size,
            head_pos: (size, 0),
            tail_pos: (size, 0),
            items: v,
        }
    }

    pub fn get_passed_cells(&self) -> i32 {
        let mut count = 0;
        for i in &self.items {
            if i.passed {
                count += 1
            }
        }

        count
    }

    pub fn simulate_instructions(&self, instructions: Vec<(String, usize)>) {
        /*
            Given direction, steps(num)
            track current position and move on the grid, only check passed if
            current position updates
        */
        for instruction in instructions {
            match instruction.0.as_str() {
                "U" => {
                    println!("found up instruction");
                    println!("{}", instruction.1);
                }
                "D" => {
                    println!("found down instruction");
                    println!("{}", instruction.1);
                }
                "L" => {
                    println!("found left instruction");
                    println!("{}", instruction.1);
                }
                "R" => {
                    println!("found right instruction");
                    println!("{}", instruction.1);
                }
                _ => {}
            }
        }
    }

    fn step_grid() {
        /*
        if steps == 1 move the head but dont update the tail,
        if steps > 1 move the head and tail, when the tail updates
        update the cell
        */
    }
}

fn main() -> io::Result<()> {
    let file = File::open("./09/rope_bridge/puzzle_input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let grid_info = parse_input(lines);
    // grid_info.0 is the max value from instructions
    let grid = Grid::new(grid_info.0);
    // grid_info.1 is a Vec of tuples for movements direction, steps
    grid.simulate_instructions(grid_info.1);

    // println!("gs: {:#?}", grid);
    // println!("pc: {}", grid.get_passed_cells());
    // println!("pi: {:#?}", grid_info.1);
    Ok(())
}

/// Parse the puzzle input return the highest value and the parsed instructions as a tuple
/// eg: (18, [direction, value...])
fn parse_input(input: Lines<BufReader<File>>) -> (usize, Vec<(String, usize)>) {
    let mut num = 0;
    let mut instructions = Vec::<(String, usize)>::new();
    for line in input {
        if let Ok(l) = line {
            let split: Vec<&str> = l.split(" ").collect();
            let v = l.split(" ").last().unwrap().parse::<usize>().unwrap();

            if v > num {
                num = v;
            }
            instructions.push((split.first().unwrap().to_string(), v));
        }
    }

    (num, instructions)
}
