use std::cell::{RefCell, RefMut};
use std::fs;
use std::rc::Rc;

type Node = Vec<(usize, String)>;

#[derive(Debug)]
struct TreeNode {
    pub value: Option<Node>,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub name: String,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(name: &str) -> TreeNode {
        TreeNode {
            value: None,
            children: Vec::new(),
            name: name.to_string(),
            parent: None,
        }
    }

    pub fn add_child(&mut self, node: TreeNode) -> RefMut<TreeNode> {
        let previous = self.children.len();
        self.children.push(Rc::new(RefCell::new(node)));

        self.children[previous].borrow_mut()
    }

    // a nicer interface to add_child
    fn add_folder(&mut self, name: &str) -> RefMut<TreeNode> {
        let child = TreeNode::new(name);

        self.add_child(child)
    }

    pub fn add_file(&mut self, file: (usize, String)) {
        if self.value.is_some() {
            self.value.as_mut().unwrap().push(file);
        } else {
            let mut node = Node::new();
            node.push(file);

            self.value = Some(node);
        }
    }

    pub fn folder_size(&self) -> usize {
        if self.value.is_some() && self.value.as_ref().unwrap().len() > 0 {
            let items = self.value.as_ref().unwrap();
            let mut out = 0;

            for item in items {
                out += item.0;
            }

            out
        } else {
            0
        }
    }

    pub fn total_folder_size(&self) -> usize {
        let mut out = 0;

        out += self.folder_size();

        if self.children.len() > 0 {
            for child in &self.children {
                // calculate all nodes child nodes sizes
                out += child.borrow().total_folder_size();
            }
        }

        out
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
    let mut peekable_t = tokens.iter().peekable();
    let mut current_dir = String::new();
    let mut v_instructions = vec![];

    while let Some(tkn) = peekable_t.next() {
        match tkn.as_str() {
            "cd" => {
                let branch = peekable_t.next().unwrap().clone();
                current_dir = branch.to_string();
                v_instructions.push((0 as usize, current_dir.to_string()));
            }
            "ls" => {
                // now map over directory contents
                while let Some(dir_tkn) = peekable_t.next() {
                    match dir_tkn.as_str() {
                        "$" => break,
                        "dir" => {}
                        _ => {
                            let file_size = dir_tkn.parse::<usize>();
                            let vfile_size = dir_tkn.parse::<usize>();
                            if file_size.is_ok() {
                                let file_name = peekable_t.next().unwrap().clone();
                                v_instructions.push((vfile_size.unwrap(), file_name.to_string()));
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }

    println!("{:#?}", v_instructions);
    let root = TreeNode::new("/");
    let mut current = Rc::new(RefCell::new(root));

    for i in v_instructions {
        // we are out of the root
        if i.0 != 0 && i.1 != "/" {
            let child = current.borrow_mut().add_folder(&i.1);
            //current = Rc::new(child);

            current = child;
        }

        if i.0 > 0 {
            current.borrow_mut().add_file((i.0, i.1.to_string()));
        }

        break;
    }
}
