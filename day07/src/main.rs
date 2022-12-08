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
    size: usize,
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

    fn total_size(&self, filter: fn(usize) -> bool, accum: &mut usize) -> usize {
        let s = if self.children.len() == 0 {
            self.size
        } else {
            let sum = self.children.iter().map(|(_, val)| val.borrow().total_size(filter, accum)).sum();
            if filter(sum) {
                *accum = *accum + sum;
            }
            sum
        };
        s
    }

    fn get_sizes(&self, accum: &mut HashMap<String, usize>) -> usize {
        if self.children.len() > 0 {
            let sum = self.children.iter().map(|(_, val)| val.borrow().get_sizes(accum)).sum();
            accum.insert(self.name.clone(), sum);
            sum
        } else {
            self.size
        }
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
    let lines: Vec<Input> = load_file("/mnt/s/AdventOfCode/2022/input07.txt").unwrap();

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
            child.borrow_mut().size = size_str.parse::<usize>().unwrap();
            current.borrow_mut().add_child(name, child);

        }
    }

    return root;
}

fn part1(contents: &[Input]) -> usize {

    let root = construct_filetree(contents);

    let mut sum: usize = 0;
    root.borrow().total_size(|size| size < 100000, &mut sum);
    sum
}

fn part2(contents: &[Input]) -> usize {

    let root = construct_filetree(contents);
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();

    let used_space = root.borrow().get_sizes(&mut dir_sizes);
    let free_space = 70_000_000 - used_space;

    let mut large_dirs: Vec<(&String, &usize)> = dir_sizes.iter().filter(|(_, &size)| free_space + size >= 30_000_000).collect();
    large_dirs.sort_by(|&o1, &o2| o1.1.partial_cmp(&o2.1).unwrap());
    println!("{large_dirs:#?}");
    *large_dirs.get(0).unwrap().1
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
    assert_eq!(part2(&lines), 24933642);
}

