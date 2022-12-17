use std::fs;
use std::time::Instant;
use std::io::{self, BufRead};
use std::collections::HashSet;

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

    fn manhattan(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn below(&self) -> Self { Pos { x: self.x, y: self.y + 1 } }
    fn below_left(&self) -> Self { Pos { x: self.x - 1, y: self.y + 1 } }
    fn below_right(&self) -> Self { Pos { x: self.x + 1, y: self.y + 1 } }
}

#[derive(Debug, PartialEq)]
enum Content {
}

fn load_file(filename: &str) -> io::Result<Vec<Input>> {
    fs::File::open(filename).map(|file| {
        io::BufReader::new(file)
            .lines()
            .map(|line| line.unwrap().parse::<Input>().unwrap())
            .collect()
    })
}

fn make_pairings(lines: &[Input]) -> Vec<(Pos, Pos)> {
    lines.iter().map(|line| {
        let words: Vec<&str> = line.split(' ').collect();
        let sensor = Pos { x: extract_number(words[2]), y: extract_number(words[3]) };
        let beacon = Pos { x: extract_number(words[8]), y: extract_number(words[9]) };
        (sensor, beacon)
    }).collect()
}

fn extract_number(s: &str) -> isize {
    s.chars().filter(|c| c.is_ascii_digit() || *c == '-').fold(String::new(), |mut a, b| {a.push(b); a}).parse().expect("Not a number")
}

fn main() {
    let start = Instant::now();
    let lines: Vec<Input> = load_file("/mnt/s/AdventOfCode/2022/input15.txt").unwrap();

    let pairings: Vec<(Pos, Pos)> = make_pairings(&lines);

    println!("Part 1: {}", part1(&pairings, 2_000_000));

    println!("Part 2: {}", part2(&pairings, 0, 4_000_000));

    let elapsed = start.elapsed();
    println!("Elapsed: {}µs", elapsed.as_micros());
}

fn part1(pairings: &[(Pos, Pos)], row: isize) -> usize {
    let mut nope = HashSet::new();
    pairings.iter().for_each(|(sensor, beacon)| {
        if (row - sensor.y).abs() <= sensor.manhattan(&beacon) {
            let vertical = (row - sensor.y).abs();
            let horizontal = sensor.manhattan(&beacon) - vertical;
            (-horizontal..=horizontal).for_each(|x| { nope.insert(sensor.x + x); });
        }
    });
    pairings.iter().for_each(|(sensor, beacon)| {
        if beacon.y == row {
            nope.remove(&beacon.x);
        }
    });
    nope.len()
}

fn part2(lines: &[(Pos, Pos)], lower: isize, upper: isize) -> usize {
    0
}

#[test]
fn test_part1() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    let pairings: Vec<(Pos, Pos)> = make_pairings(&lines);
    assert_eq!(part1(&pairings, 10), 26);
}

#[test]
fn test_part2() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    let pairings: Vec<(Pos, Pos)> = make_pairings(&lines);
    assert_eq!(part2(&pairings, 0, 20), 56000011);
}

