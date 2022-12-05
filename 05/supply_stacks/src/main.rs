use std::fs;

fn main() {
    // let mut stacks = vec![
    //     vec!['Z', 'N'],
    //     vec!['M', 'C', 'D'],
    //     vec!['P']
    // ];

    let mut stacks = vec![
        vec!['G', 'D', 'V', 'Z', 'J', 'S', 'B'],
        vec!['Z', 'S', 'M', 'G', 'V', 'P'],
        vec!['C', 'L', 'B', 'S', 'W', 'T', 'Q', 'F'],
        vec!['H', 'J', 'G', 'W', 'M', 'R', 'V', 'Q'],
        vec!['C', 'L', 'S', 'N', 'F', 'M', 'D'],
        vec!['R', 'G', 'C', 'D'],
        vec!['H', 'G', 'T', 'R', 'J', 'D', 'S', 'Q'],
        vec!['P', 'F', 'V'],
        vec!['D', 'R', 'S', 'T', 'J']
    ];

    let mut _instructions: Vec<(usize, usize, usize)> = Vec::new();
    let input = fs::read_to_string("./05/supply_stacks/puzzle_input.txt").expect("failed to read file.");
    let mut bites = input.chars().peekable();


    let mut keywords: Vec<String> = Vec::new();
    let mut keyword = String::new();
    while let Some(bite) = bites.next() {
        match bite {
            ' ' | '\n' => {
               if !keyword.is_empty() {
                   keywords.push(keyword);
                   keyword = String::new();
               }
            }
           _ => keyword.push(bite)
        }
    }

    let mut words = keywords.iter().peekable();
    let mut instruction: (usize, usize, usize) = (0, 0, 0);
    while let Some(word) = words.next() {
        match word.as_str() {
            "move" => {
                let amount_to_move = words.next().unwrap();
                instruction.0 = amount_to_move.trim().parse::<usize>().unwrap();
            },
            "from" => {
                let from_array = words.next().unwrap();
                instruction.1 = from_array.trim().to_string().parse().unwrap();
            },
            "to" => {
                let to_array = words.next().unwrap();
                instruction.2 = to_array.trim().parse::<usize>().unwrap();

                _instructions.push(instruction);
                instruction = (0, 0, 0);
            },
            _ => ()
        }
    }

    println!("{:?}", &_instructions);

    // parrt 1
    // for i in &_instructions {
    //     for _ in 0..i.0 {
    //         let v = stacks[i.1 - 1].pop().unwrap();
    //         stacks[i.2 - 1].push(v);
    //     }
    // }

    //part 2
    for i in _instructions {
        let final_length = stacks[i.1 - 1].len().saturating_sub(i.0);
        let vals = stacks[i.1 - 1].split_off(final_length);
        stacks[i.2 - 1].extend(vals);
    }

    let mut code = String::new();
    for s in stacks.iter() {
        code.push(s.clone().pop().unwrap());
    }

    println!("{:#?}", code);

}
