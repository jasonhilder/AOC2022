use std::fs;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("./08/puzzle_input.txt")?;
    let lines: Vec<&str> = input.split("\n").collect();
    let tree_rows: Vec<Vec<u64>> = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .map(|v| {
            v.iter()
                .map(|c| c.to_string().parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();

    let mut total_vis_trees = 0;
    let mut scenic_val = 0;

    // loop over row
    for (y, row) in tree_rows.iter().enumerate() {
        // loop over tree in row
        for (x, _) in row.iter().enumerate() {
            // PART ONE
            if check_surroundings(&tree_rows, (x, y)) {
                total_vis_trees += 1;
            }

            // PART TWO
            let v = calc_scenic_value(&tree_rows, (x, y));
            if v > scenic_val {
                scenic_val = v;
            }
        }
    }

    println!("part 1: {}", total_vis_trees);
    println!("part 2: {}", scenic_val);
    Ok(())
}

fn check_surroundings(trees: &Vec<Vec<u64>>, pos: (usize, usize)) -> bool {
    let current_tree = &trees[pos.1][pos.0];
    let row_length = &trees.first().unwrap().len();
    let pos_x = pos.0;
    let pos_y = pos.1;

    // check if we are on the edge of grid
    if pos_x == 0 || pos_y == 0 || pos_x == row_length - 1 || pos_y == trees.len() - 1 {
        return true;
    }

    // pos_x to left
    let mut l_pass = true;
    for j in (0..pos_x).rev() {
        if &trees[pos_y][j] >= &current_tree {
            l_pass = false;
            break;
        }
    }
    if l_pass {
        return true;
    }

    // pos to right
    let mut r_pass = true;
    for j in (pos_x + 1)..=row_length - 1 {
        if &trees[pos_y][j] >= &current_tree {
            r_pass = false;
            break;
        }
    }
    if r_pass {
        return true;
    }

    // pos_y to top
    let mut t_pass = true;
    for k in (0..pos_y).rev() {
        if &trees[k][pos_x] >= &current_tree {
            t_pass = false;
            break;
        }
    }
    if t_pass {
        return true;
    }

    // pos_y to bot
    let mut b_pass = true;
    for k in (pos_y + 1)..=trees.len() - 1 {
        if &trees[k][pos_x] >= &current_tree {
            b_pass = false;
            break;
        }
    }
    if b_pass {
        return true;
    }

    false
}

fn calc_scenic_value(trees: &Vec<Vec<u64>>, pos: (usize, usize)) -> u64 {
    let current_tree = &trees[pos.1][pos.0];
    let row_length = &trees.first().unwrap().len();
    let pos_x = pos.0;
    let pos_y = pos.1;

    let mut left_v = 0;
    let mut right_v = 0;
    let mut top_v = 0;
    let mut bottom_v = 0;

    // pos_x to left
    for j in (0..pos_x).rev() {
        left_v += 1;
        if &trees[pos_y][j] >= &current_tree {
            break;
        }
    }

    // pos to right
    for j in (pos_x + 1)..=row_length - 1 {
        right_v += 1;
        if &trees[pos_y][j] >= &current_tree {
            break;
        }
    }

    // pos_y to top
    for k in (0..pos_y).rev() {
        top_v += 1;
        if &trees[k][pos_x] >= &current_tree {
            break;
        }
    }

    // pos_y to bot
    for k in (pos_y + 1)..=trees.len() - 1 {
        bottom_v += 1;
        if &trees[k][pos_x] >= &current_tree {
            break;
        }
    }

    left_v * right_v * top_v * bottom_v
}
