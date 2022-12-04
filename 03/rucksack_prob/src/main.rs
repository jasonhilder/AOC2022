use std::fs;

fn main() {
    let input = fs::read_to_string("./03/rucksack_prob/puzzle_input.txt").expect("failed to read file.");
    let rucksacks: Vec<&str> = input.split("\n").collect();
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();
    let mut total_sum = 0;
    let mut total_sum2 = 0;

    // part 1
    for rucksack in &rucksacks {
        let compartment_len = rucksack.len() / 2;
        let compartments = rucksack.split_at(compartment_len);
        let c1: Vec<char> = compartments.0.chars().collect();
        let c2: Vec<char> = compartments.1.chars().collect();

        for c in c1.iter() {
           if c2.contains(c) {
               total_sum += alphabet.iter().position(|&i| i == *c).unwrap() + 1;
               break;
           }
        }

    }

    // part 2
    for rucksack_group in rucksacks.chunks(3) {
        for c in rucksack_group.first().unwrap().chars().collect::<Vec<char>>() {
            if rucksack_group[1].contains(c) && rucksack_group[2].contains(c) {
               total_sum2 += alphabet.iter().position(|&i| i == c).unwrap() + 1;
               break;
            }
        }
    }

    println!("{}", total_sum);
    println!("{}", total_sum2);
}
