use std::fs;
use std::io::{self, BufRead};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

type Input = String;

fn load_file(filename: &str) -> io::Result<Vec<Input>> {
    fs::File::open(filename).map(|file| {
        io::BufReader::new(file)
            .lines()
            .map(|line| line.unwrap().parse::<Input>().unwrap())
            .collect()
    })
}

#[derive(Debug)]
struct Node {
    name: String,
    parent: Option<Rc<RefCell<Node>>>,
    children: HashMap<String, Rc<RefCell<Node>>>,
    size: u32,
}

impl Node {
    fn new(name: String, parent: Option<Rc<RefCell<Node>>>) -> Self {
        return Node {
            name,
            parent,
            children: HashMap::new(),
            size: 0,
        }
    }

    fn add_child(&mut self, name: &str, child: Rc<RefCell<Node>>) {
        self.children.insert(name.to_string(), child);
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node: {{ name: {}, size: {}, children: {{", self.name, self.size);
        for (n, child) in &self.children {
            write!(f, "{n}: {},", child.borrow());
        }
        write!(f, "}} }}")
    }
}



fn main() {
    let lines: Vec<Input> = load_file("/mnt/s/AdventOfCode/2022/input06.txt").unwrap();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn construct_filetree(commands: &[Input]) -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node::new("".to_string(), None)));

    let mut current = Rc::clone(&root);
    for cmd in commands {
        println!("{cmd}");
        if cmd.starts_with("$ ls") {
            // Do nothing
        }
        if cmd.starts_with("$ cd ") {
            let (_, new_dir) = cmd.split_at(5);
            match new_dir {
                "/" => current = Rc::clone(&root),
                ".." => {
                    let current_clone = Rc::clone(&current);
                    current = Rc::clone(&current_clone.borrow().parent.as_ref().unwrap());
                },
                _ => {
                    let current_clone = Rc::clone(&current);
                    current = Rc::clone(current_clone.borrow().children.get(new_dir).unwrap());
                },
            }
        }
        if cmd.starts_with("dir ") {
            // Adding a directory means adding a child node
            // "dir name-of-dir"
            let (_dir, name) = cmd.split_once(" ").unwrap();
            let parent = Some(Rc::clone(&current));
            let child = Rc::new(RefCell::new(Node::new(name.to_string(), parent)));
            current.borrow_mut().add_child(name, child);
        }
        if cmd.starts_with(|c: char| c.is_numeric()) {
            // Adding a file means adding a child node
            // "dir name-of-dir"
            let (size_str, name) = cmd.split_once(" ").unwrap();
            let parent = Some(Rc::clone(&current));
            let child = Rc::new(RefCell::new(Node::new(name.to_string(), parent)));
            child.borrow_mut().size = size_str.parse::<u32>().unwrap();
            current.borrow_mut().add_child(name, child);

        }
        println!("{}", root.borrow());
    }

    return root;
}

fn part1(contents: &[Input]) -> usize {

    let _root = construct_filetree(contents);

    panic!("Bad input!");
}

fn part2(_contents: &[Input]) -> usize {

    panic!("Bad input!");
}

#[test]
fn test_part1() {
    println!("Running test part 1");
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    println!("File loaded!");
    assert_eq!(part1(&lines), 95437);
}

#[test]
fn test_part2() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    assert_eq!(part2(&lines), 0);
}

