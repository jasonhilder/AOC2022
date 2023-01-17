use std::fs::File;
use std::io::{self, prelude::*, BufReader, Lines};

#[derive(Debug)]
pub struct Cell {
    _x: usize,
    _y: usize,
    passed: bool,

}
#[derive(Debug)]
pub struct Grid {
    width: usize,
    height: usize,
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
                    _x: x,
                    _y: y,
                    passed: false,
                })
            }
        }

        Self {
            width: size,
            height: size,
            head_pos: (size, 0),
            tail_pos: (size, 0),
            items: v,
        }
    }
    /*
        index = width * row + col
        col = index % width
        row = index / width
    */
    fn flat_index(&self, pos: (usize, usize)) -> usize {
        pos.0 * (self.width - 1) + pos.1 * (self.height - 1)
    }

    pub fn at_pos(&mut self, pos: (usize, usize)) -> &Cell {
        &self.items[self.flat_index(pos)]
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

    pub fn simulate_instructions(&mut self, instructions: Vec<(String, usize)>) {
        /*
            Given direction, steps(num)
            track current position and move on the grid, only check passed if
            current position updates
        */
        for instruction in instructions {
            match instruction.0.as_str() {
                "U" => {
                    self.step_grid("col", "decrease", instruction.1);
                }
                "D" => {
                    self.step_grid("col", "increase", instruction.1);
                }
                "L" => {
                    self.step_grid("row", "decrease", instruction.1);
                }
                "R" => {
                    self.step_grid("row", "increase", instruction.1);
                }
                _ => {}
            }
        }
    }

    // compare distance from head to tail, if greater than 1 between
    // move tail one place and update its cell.
    fn update_tail(&mut self) {
        // mark this cell as "passed" before anything as the tail is here.
        let tail_cell_index = self.flat_index(self.tail_pos);

        if !self.items[tail_cell_index].passed {
            self.items[tail_cell_index].passed  = true;
        };

        // compare rows first
        if self.head_pos.0 - self.tail_pos.0 == 2 {
            self.tail_pos = ((self.head_pos.0 - 1), self.tail_pos.1);
        };

        // compare cols
        if self.head_pos.1 - self.tail_pos.1 == 2 {
            self.tail_pos = (self.tail_pos.0, (self.head_pos.1 - 1));
        };
    }

    fn step_grid(&mut self, axis: &str, direction: &str, steps: usize) {
        /*
        if steps == 1 move the head but dont update the tail,
        if steps > 1 move the head and tail, when the tail updates
        update the cell

        todo:
            using the head_pos increment along each step and set the passed field if needed
        */
        for _s in 0..steps {
            if axis == "row" {
                if direction == "increase" {
                    self.head_pos = ((self.head_pos.0 + 1), self.head_pos.1)                
                } else {
                    self.head_pos = ((self.head_pos.0 - 1), self.head_pos.1)                
                }
            } else {
                if direction == "increase" {
                    self.head_pos = (self.head_pos.0, (self.head_pos.1 + 1))                
                } else {
                    if self.head_pos.1 > 0 {
                        self.head_pos = (self.head_pos.0, (self.head_pos.1 - 1))
                    } else {
                        self.head_pos = (self.head_pos.0, 0)
                    }
                }
            } 

            self.update_tail();
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("./09/rope_bridge/puzzle_input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let grid_info = parse_input(lines);

    // grid_info.0 is the max value from instructions so we can create the square grid
    let mut grid = Grid::new(grid_info.0);
    // grid_info.1 is a Vec of tuples for movements == direction, steps
    //println!("pi: {:#?}", &grid_info.1);

    grid.simulate_instructions(grid_info.1);

    //println!("gs: {:#?}", grid);
    println!("pc: {}", grid.get_passed_cells());
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
