use std::fs;
use std::time::Instant;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::cmp;

type Input = String;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn toward(&self, other: &Self) -> Self {
        Pos {
            x: self.x + (other.x - self.x).signum(),
            y: self.y + (other.y - self.y).signum(),
        }
    }

    fn below(&self) -> Self { Pos { x: self.x, y: self.y + 1 } }
    fn below_left(&self) -> Self { Pos { x: self.x - 1, y: self.y + 1 } }
    fn below_right(&self) -> Self { Pos { x: self.x + 1, y: self.y + 1 } }
}

#[derive(PartialEq)]
enum Content {
    Air,
    Sand,
    Rock,
}

fn load_file(filename: &str) -> io::Result<Vec<Input>> {
    fs::File::open(filename).map(|file| {
        io::BufReader::new(file)
            .lines()
            .map(|line| line.unwrap().parse::<Input>().unwrap())
            .collect()
    })
}

fn read_walls(lines: &[Input]) -> Vec<Vec<Pos>> {
    lines.iter().map(|line| {
        line.split(" -> ").map(|point| {
            let (x, y) = point.split_once(',').expect("Bad point!");
            Pos { x: x.parse().expect("Bad x!"), y: y.parse().expect("Bad y!") }
        }).collect()
    }).collect()
}

fn main() {
    let start = Instant::now();
    let lines: Vec<Input> = load_file("/mnt/s/AdventOfCode/2022/input14.txt").unwrap();
    let walls: Vec<Vec<Pos>> = read_walls(&lines);

    println!("Part 1: {}", part1(&walls));
    println!("Part 2: {}", part2(&walls));
    let elapsed = start.elapsed();
    println!("Elapsed: {}µs", elapsed.as_micros());
}

fn part1(walls: &Vec<Vec<Pos>>) -> usize {
    let mut grid: HashMap<Pos, Content> = HashMap::new();

    // Build the grid from the walls
    let mut bottom = 0;
    walls.iter().for_each(|wall| {
        //println!("{wall:?}");
        wall.windows(2).for_each(|joint| {
            //println!("{joint:?}");
            let mut block = joint[0];
            grid.insert(block, Content::Rock);
            while block != joint[1] {
                block = block.toward(&joint[1]);
                //println!("{block:?}");
                grid.insert(block, Content::Rock);
                // For some reason, sand falls toward greater y-values...
                bottom = cmp::max(block.y, bottom);
            }

        });
    });

    // Start dropping sand
    let origin = Pos {x: 500, y: 0};
    let mut grains = 0;
    let mut sand = origin.clone();
    while sand.y <= bottom {
        let options = vec![sand.below(), sand.below_left(), sand.below_right()];
        if let Some(next) = options.iter().find(|pos| grid.get(pos) == None) {
            sand = *next;
        } else {
            grid.insert(sand, Content::Sand);
            grains += 1;
            sand = origin.clone();
        }
    }
    grains
}

fn part2(walls: &Vec<Vec<Pos>>) -> usize {
    0
}

#[test]
fn test_part1() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    let grid = read_walls(&lines);
    assert_eq!(part1(&grid), 0);
}

#[test]
fn test_part2() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    let grid = read_walls(&lines);
    assert_eq!(part2(&grid), 0);
}

