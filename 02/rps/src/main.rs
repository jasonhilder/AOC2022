use std::{fs, collections::HashMap};

// 1 2 3
// A B C
// X Y Z
// L D W
fn main() {
    const WIN:usize = 6;
    const DRAW:usize = 3;
    let values = HashMap::from([
        ('A', 0), ('X', 0),
        ('B', 1), ('Y', 1),
        ('C', 2), ('Z', 2),
    ]);
    let input = fs::read_to_string("./02/rps/puzzle_input.txt").expect("failed to read file.");
    let stripped = input.trim().replace(" ", "");
    let mut rounds = stripped.chars().peekable();
    let mut score:usize = 0;

    let mut score2:usize = 0;

    // part 1
    let mut r1 = rounds.clone();
    while let Some(p1) = r1.next() {
        if p1 == '\n' { continue }

        if let Some(p2) = r1.next() {
            score += values[&p2] + 1;

            if values[&p1] == values[&p2] {
                score += DRAW;
            } else if (values[&p2] + 1)  % 3 != values[&p1] {
                score += WIN;
            }
        }
    }

    // part 2
    while let Some(p1) = rounds.next() {
        if p1 == '\n' { continue }

        if let Some(p2) = rounds.next() {
            if p2 == 'Z' {
                score2 += WIN;
                match p1 {
                    'A' => score2 += 2,
                    'B' => score2 += 3,
                    'C' => score2 += 1,
                    _ => ()
                }
            } else if p2 == 'Y' {
                score2 += DRAW + values[&p1] + 1;
            } else {
                match p1 {
                    'A' => score2 += 3,
                    'B' => score2 += 1,
                    'C' => score2 += 2,
                    _ => ()
                }

            }
        }
    }

    println!("{}", score);
    println!("{}", score2);
}
