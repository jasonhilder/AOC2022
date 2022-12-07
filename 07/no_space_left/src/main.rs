use std::{fs, iter::Peekable, slice::Iter};

#[derive(Debug, Default)]
struct Node<T> {
    dir: T,
    contents: Option<u64>,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

fn generate_tree(tokens: &Peekable<Iter<String>>) -> Option<Box<Node<String>>> {
    let mut root = Node::<String>::default();
    let mut peekable_t = tokens.clone();

    while let Some(tkn) = peekable_t.next() {
        match tkn.as_str() {
            "cd" => {
                let branch = peekable_t.next().unwrap().clone();
                Some(Box::new(Node {
                    dir: branch.to_string(),
                    contents: None,
                    left: generate_tree(&peekable_t),
                    right: generate_tree(&peekable_t),
                }));
            }
            _ => (),
        }
    }

    None
}

fn main() {
    let input =
        fs::read_to_string("./07/no_space_left/puzzle_input.txt").expect("failed to read file.");

    let mut bites = input.chars().peekable();

    let mut tokens: Vec<String> = Vec::new();
    let mut token = String::new();
    while let Some(bite) = bites.next() {
        match bite {
            '\n' | ' ' => {
                if !token.is_empty() {
                    tokens.push(token);
                    token = String::new();
                }
            }
            _ => token.push(bite),
        }
    }

    //println!("{:#?}", empty);
    println!("{:#?}", tokens);

    let p_tokens = tokens.iter().peekable();
    let x = generate_tree(&p_tokens);

    println!("{:#?}", x);
}
