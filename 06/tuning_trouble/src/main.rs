use std::fs;

fn main() {
    let input = fs::read_to_string("./06/tuning_trouble/puzzle_input.txt").expect("failed to read file.");
    let mut count = 0;
    let mut sop = false;
    let mut marker = String::new();

    for c in input.chars() {
        count += 1;
        if marker.contains(c) {
            // to remove all character before duplicate
            // split current string at the duplicated character (c)
            // aksdk == [ak][skdk]
            marker = marker.split(c).last().unwrap().to_string();
            marker.push(c);
        } else {
            marker.push(c);
        }

        if marker.len() == 4 && sop == false {
            println!("sop marker found!: {}", marker);
            println!("{}", count);
            sop = true;
        }

        if marker.len() == 14 {
            println!("marker found!: {}", marker);
            println!("{}", count);
            return
        }
    }
}
