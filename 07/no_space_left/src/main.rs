use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

fn directory_size(reader: &mut Lines<BufReader<File>>, list: &mut Vec<usize>) -> usize {
    let mut size: usize = 0;

    while let Some(line) = reader.next() {
        let line = line.unwrap();

        if line == "$ cd .." {
            break;
        }

        let words: Vec<&str> = line.split(" ").collect();
        if words[1] == "cd" {
            size += directory_size(reader, list);
        }

        size += words[0].parse::<usize>().unwrap_or(0);
    }
    list.push(size);

    size
}

fn main() -> std::io::Result<()> {
    let input = File::open("./07/no_space_left/puzzle_input.txt")?;
    let mut reader = BufReader::new(input).lines();
    let mut directories: Vec<usize> = Vec::new();
    let total = directory_size(&mut reader, &mut directories);

    println!("total {:?}", directories);

    //part 1
    let mut p1_total = 0;
    for d in directories.iter() {
        if d <= &100_000 {
            p1_total += d
        }
    }
    //println!("p1: {}", p1_total);
    for d in directories.iter() {
        if (70000000 - total) + d >= 30000000 {
            println!("d: {:?}", d);
            break;
        }
    }

    //part 2

    Ok(())
}
