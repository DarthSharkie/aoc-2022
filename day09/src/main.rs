use std::fs;
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::ops::{AddAssign, Sub};

type Input = String;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn move_right(&mut self) { self.x += 1; }
    fn move_left(&mut self) { self.x -= 1; }
    fn move_up(&mut self) { self.y += 1; }
    fn move_down(&mut self) { self.y -= 1; }

    fn right(self) -> Self { 
        Self { x: self.x + 1, y: self.y } 
    }
    fn left(self) -> Self { 
        Self { x: self.x - 1, y: self.y } 
    }
    fn up(self) -> Self { 
        Self { x: self.x, y: self.y + 1 } 
    }
    fn down(self) -> Self { 
        Self { x: self.x, y: self.y - 1 } 
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}


fn load_file(filename: &str) -> io::Result<Vec<Input>> {
    fs::File::open(filename).map(|file| {
        io::BufReader::new(file)
            .lines()
            .map(|line| line.unwrap().parse::<Input>().unwrap())
            .collect()
    })
}

fn main() {
    let lines: Vec<Input> = load_file("/mnt/s/AdventOfCode/2022/input09.txt").unwrap();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(contents: &[Input]) -> usize {
    let mut visited: HashSet<Pos> = HashSet::new();

    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };

    for cmd in contents {
        let (dir, steps) = cmd.split_once(" ").map(|t| (t.0, t.1.parse::<usize>().expect("not a usize"))).unwrap();
        for _ in 0..steps {
            // move head
            match dir {
                "R" => head.move_right(),
                "U" => head.move_up(),
                "L" => head.move_left(),
                "D" => head.move_down(),
                _ => (),
            }

            // move tail
            let diff = head - tail;
            match diff {
                Pos { x: -2, .. } => tail = head.right(),
                Pos { x: 2, .. } => tail = head.left(),
                Pos { y: -2, .. } => tail = head.up(),
                Pos { y: 2, .. } => tail = head.down(),
                _ => (),
            }

            visited.insert(tail.clone());
        }
    }
    visited.len()
}

fn part2(contents: &[Input]) -> usize {
    0
}

#[test]
fn test_part1() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    assert_eq!(part1(&lines), 13);
}

#[test]
fn test_part2() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    assert_eq!(part2(&lines), 0);
}

