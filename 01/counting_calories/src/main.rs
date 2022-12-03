use std::fs;

fn main() {
    let mut totals = Vec::<usize>::new();
    let numbers = fs::read_to_string("./src/puzzle_input.txt").expect("failed to read file.");
    let mut current_total: usize = 0;

    // part one just get the largest calories holder
    for num in numbers.split("\n") {

        match num {
            "" => {
                totals.push(current_total);
                current_total = 0;
            },
            _ => {
               let parsed_int: usize = num.parse().unwrap();
               current_total = current_total + parsed_int;
           }
        }
    }

    totals.sort();
    println!("{:#?}", totals.last().unwrap());

    // part two get the top 3 calories holder and return their total
    println!("{:#?}", totals.iter().rev().take(3).sum::<usize>());
}
