use std::fs;
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::ops::{Sub};

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
    let mut snake: [Pos; 2] = [Pos { x: 0, y: 0 }; 2]; 
    snake_march(contents, &mut snake)
}

fn part2(contents: &[Input]) -> usize {
    let mut snake: [Pos; 10] = [Pos { x: 0, y: 0 }; 10]; 
    snake_march(contents, &mut snake)
}

fn snake_march(contents: &[Input], snake: &mut [Pos]) -> usize {
    let mut visited: HashSet<Pos> = HashSet::new();

    for cmd in contents {
        let (dir, steps) = cmd.split_once(" ").map(|t| (t.0, t.1.parse::<usize>().expect("not a usize"))).unwrap();
        for _ in 0..steps {
            // move head
            match dir {
                "R" => snake[0].move_right(),
                "U" => snake[0].move_up(),
                "L" => snake[0].move_left(),
                "D" => snake[0].move_down(),
                _ => (),
            }

            // move body
            for segment in 1..snake.len() {
                let diff = snake[segment - 1] - snake[segment];
                match diff {
                    // "corner" cases
                    Pos { x: -2, y: -2 } => snake[segment] = snake[segment - 1].right().up(),
                    Pos { x: -2, y: 2 } => snake[segment] = snake[segment - 1].right().down(),
                    Pos { x: 2, y: -2 } => snake[segment] = snake[segment - 1].left().up(),
                    Pos { x: 2, y: 2 } => snake[segment] = snake[segment - 1].left().down(),
                    // "edge" cases
                    Pos { x: -2, .. } => snake[segment] = snake[segment - 1].right(),
                    Pos { x: 2, .. } => snake[segment] = snake[segment - 1].left(),
                    Pos { y: -2, .. } => snake[segment] = snake[segment - 1].up(),
                    Pos { y: 2, .. } => snake[segment] = snake[segment - 1].down(),
                    // "happy" cases
                    _ => (),
                }
            }

            visited.insert(snake[snake.len() - 1].clone());
        }
    }
    visited.len()
}

#[test]
fn test_part1() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    assert_eq!(part1(&lines), 13);
}

#[test]
fn test_part2() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    assert_eq!(part2(&lines), 1);
}

