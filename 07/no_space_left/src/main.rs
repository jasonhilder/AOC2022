use std::{cell::RefCell, fs, iter::Peekable, rc::Rc, slice::Iter};

#[derive(Debug, Default, PartialEq)]
struct Node {
    value: Option<String>,
    files: Vec<u64>,
    size: u64,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new() -> Node {
        return Node {
            value: None,
            size: 0,
            files: vec![],
            children: vec![],
            parent: None,
        };
    }

    pub fn print(&self) -> String {
        if let Some(value) = &self.value {
            let v = value.to_string();

            let x = String::from("[")
                + &self
                    .children
                    .iter()
                    .map(|tn| tn.borrow().print())
                    .collect::<Vec<String>>()
                    .join(",")
                + "]";

            format!(
                "dir:{},dir_size:{}, dir_files: {:?}, dir_children: {}",
                v, &self.size, &self.files, x
            )
        } else {
            "None".to_string()
        }
    }

    pub fn get_dir_size(&self) -> u64 {
        let mut total_size = 0;

        for f in &self.files {
            total_size += f;
        }

        total_size
    }

    pub fn get_dir_totals(&self, cap: u64) {
        let mut total = 0;

        if &self.size <= &cap {
            total += &self.size;
        }

        for f in &self.files {
            total_size += f;
        }
    }
}

fn generate_tree(tokens: &Peekable<Iter<String>>) {
    let root = Rc::new(RefCell::new(Node::new()));
    let mut current = Rc::clone(&root);
    let mut peekable_t = tokens.clone();

    while let Some(tkn) = peekable_t.next() {
        match tkn.as_str() {
            "cd" => {
                let branch = peekable_t.next().unwrap().clone();
                if branch == "/" {
                    current.borrow_mut().value = Some(branch);
                } else if branch == ".." {
                    let parent = current.borrow().parent.clone().unwrap();
                    current = parent;
                } else {
                    let child = Rc::new(RefCell::new(Node::new()));
                    child.borrow_mut().value = Some(branch);
                    child.borrow_mut().parent = Some(Rc::clone(&current));
                    current.borrow_mut().children.push(Rc::clone(&child));

                    current = child;
                }
            }
            "ls" => {
                // now map over directory contents

                while let Some(dir_tkn) = peekable_t.next() {
                    match dir_tkn.as_str() {
                        "$" => break,
                        "dir" => {}
                        _ => {
                            let is_num = dir_tkn.parse::<u64>();
                            if is_num.is_ok() {
                                current.borrow_mut().files.push(is_num.unwrap());
                            }
                        }
                    }
                }

                let size = current.borrow_mut().get_dir_size();
                current.borrow_mut().size = size;
                println!("a: {}", size);

                if current.borrow_mut().parent.is_some() {
                    root.borrow_mut().size += size;
                    current
                        .borrow_mut()
                        .parent
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .size += size;
                }
            }

            _ => (),
        }
    }

    //println!("{:?}", root.borrow_mut().cal_total());
    //println!("Tree: {:#?}", root.borrow().print(););
    let mut total = 0;
    for n in root.borrow_mut() {
        let x = n.borrow_mut();
    }
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

    //println!("{:#?}", tokens);

    let p_tokens = tokens.iter().peekable();
    generate_tree(&p_tokens);
}
