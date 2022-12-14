use std::fs;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("./08/puzzle_input.txt")?;
    let lines: Vec<&str> = input.split("\n").collect();
    let rows = lines.iter().len();
    let columns = lines.first().unwrap().len();
    let tree_rows: Vec<Vec<u64>> = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .map(|v| {
            v.iter()
                .map(|c| c.to_string().parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();

    let edge_trees: usize = (rows * 2) + ((columns - 2) * 2);
    let mut total_vis_trees = 0;

    // loop over row
    for (y, row) in tree_rows.iter().enumerate() {
        // skip top and bottom row
        if y > 0 && y < tree_rows.len() - 1 {
            //
            // loop over tree in row
            for (x, _) in row.iter().enumerate() {
                // skip first and last tree
                if x > 0 && x < row.len() - 1 {
                    println!("checking at: {:?}", &tree_rows[y][x]);
                    if check_surroundings(&tree_rows, (x, y)) {
                        total_vis_trees += 1;
                    }
                }
            }
            //
        }
    }

    println!("et: {} ", edge_trees);
    println!("tt: {}", total_vis_trees);
    println!("total: {}", edge_trees + total_vis_trees);
    Ok(())
}

fn check_surroundings(trees: &Vec<Vec<u64>>, pos: (usize, usize)) -> bool {
    let current_tree = &trees[pos.1][pos.0];
    let row_length = &trees.first().unwrap().len();
    let pos_x = pos.0;
    let pos_y = pos.1;
    // println!("ct: {}", current_tree);
    // println!("px: {:#?} ", (pos_x, pos_y));

    for j in 0..row_length - 1 {
        if &trees[pos_y][j] < current_tree && (j, pos_y) != pos {
            println!("passed!");
            return true;
        }
    }

    for j in (0..row_length - 1).rev() {
        if &trees[pos_y][j] < current_tree && (j, pos_y) != pos {
            println!("passed!");
            return true;
        }
    }

    for k in 0..trees.len() - 1 {
        if &trees[k][pos_x] < current_tree && (pos_x, k) != pos {
            println!("passed!");
            return true;
        }
    }

    for k in (0..trees.len() - 1).rev() {
        if &trees[k][pos_x] < current_tree && (pos_x, k) != pos {
            println!("passed!");
            return true;
        }
    }

    println!("failed!");
    false
}
