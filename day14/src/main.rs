use std::fs;
use std::time::Instant;
use std::io::{self, BufRead};
use std::collections::HashMap;

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

#[derive(Debug, PartialEq)]
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

fn build_grid(walls: &Vec<Vec<Pos>>) -> HashMap<Pos, Content> {
    let mut grid: HashMap<Pos, Content> = HashMap::new();
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
            }

        });
    });
    grid
}

fn main() {
    let start = Instant::now();
    let lines: Vec<Input> = load_file("/mnt/s/AdventOfCode/2022/input14.txt").unwrap();
    let walls: Vec<Vec<Pos>> = read_walls(&lines);

    let mut grid = build_grid(&walls);
    println!("Part 1: {}", part1(&mut grid));

    let mut grid = build_grid(&walls);
    println!("Part 2: {}", part2(&mut grid));

    let elapsed = start.elapsed();
    println!("Elapsed: {}µs", elapsed.as_micros());
}

fn part1(grid: &mut HashMap<Pos, Content>) -> usize {

    // For some reason, sand falls toward greater y-values...
    let bottom = grid.keys().map(|pos| pos.y).max().expect("No grid!");

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

fn part2(grid: &mut HashMap<Pos, Content>) -> usize {

    // For some reason, sand falls toward greater y-values...
    let bottom = grid.keys().map(|pos| pos.y).max().expect("No grid!") + 2;
    // Fill in the floor
    let left = 500 - bottom - 5;
    let right = 500 + bottom + 5;
    println!("{left} - {right} is the floor");
    for x in left..=right {
        grid.insert(Pos { x, y: bottom }, Content::Rock);
    }

    // Start dropping sand
    let origin = Pos {x: 500, y: 0};
    let mut grains = 0;
    let mut sand = origin.clone();
    while grid.get(&origin) == None {
        let options = vec![sand.below(), sand.below_left(), sand.below_right()];
        if let Some(next) = options.iter().find(|pos| grid.get(pos) == None) {
            sand = *next;
            // print!("{sand:?} -> ");
        } else {
            //println!("{sand:?}");
            grid.insert(sand, Content::Sand);
            grains += 1;
            sand = origin.clone();
            //println!("{:?}", grid.get(&sand));
        }
    }
    grains
}

#[test]
fn test_part1() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    let mut grid = build_grid(&read_walls(&lines));
    assert_eq!(part1(&mut grid), 24);
}

#[test]
fn test_part2() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    let mut grid = build_grid(&read_walls(&lines));
    assert_eq!(part2(&mut grid), 93);
}

