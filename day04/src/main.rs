use std::fs;
use std::io::{self, BufRead};

type Input = String;

fn load_file(filename: &str) -> io::Result<Vec<Input>> {
    fs::File::open(filename).map(|file| {
        io::BufReader::new(file)
            .lines()
            .map(|line| line.unwrap().parse::<Input>().unwrap())
            .collect()
    })
}


fn main() {
    let filename = String::from("/mnt/s/AdventOfCode/2022/input04.txt");
    let lines: Vec<Input> = load_file(&filename).unwrap();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

struct Assignment {
    first: usize,
    second: usize,
    third: usize,
    fourth: usize,
}

impl Assignment {
    fn contains(&self) -> bool {
        (self.first >= self.third && self.second <= self.fourth) ||
            (self.third >= self.first && self.fourth <= self.second)
    }

    fn overlaps(&self) -> bool {
        (self.first <= self.third && self.third <= self.second) ||
            (self.first <= self.fourth && self.fourth <= self.second) ||
            (self.third <= self.first && self.first <= self.fourth) ||
            (self.third <= self.second && self.second <= self.fourth)
    }
}

fn part1(contents: &[Input]) -> usize {
    contents.iter().map(|s| {
        let (left, right) = s.split_once(",").unwrap();
        let (first, second) = left.split_once("-").unwrap();
        let (third, fourth) = right.split_once("-").unwrap();
        let a = Assignment {
            first: first.parse::<usize>().unwrap(),
            second: second.parse::<usize>().unwrap(), 
            third: third.parse::<usize>().unwrap(), 
            fourth: fourth.parse::<usize>().unwrap(),
        };
        a.contains()
    })
    .filter(|&contain| contain)
    .count()
}

fn part2(contents: &[Input]) -> usize {
    contents.iter().map(|s| {
        let (left, right) = s.split_once(",").unwrap();
        let (first, second) = left.split_once("-").unwrap();
        let (third, fourth) = right.split_once("-").unwrap();
        let a = Assignment {
            first: first.parse::<usize>().unwrap(),
            second: second.parse::<usize>().unwrap(), 
            third: third.parse::<usize>().unwrap(), 
            fourth: fourth.parse::<usize>().unwrap(),
        };
        a.overlaps()
    })
    .filter(|&overlap| overlap)
    .count()
}

#[test]
fn test_part1() {
    let input = vec![
        String::from("2-4,6-8"),
        String::from("2-3,4-5"),
        String::from("5-7,7-9"),
        String::from("2-8,3-7"),
        String::from("6-6,4-6"),
        String::from("2-6,4-8"),
    ];
    assert_eq!(part1(&input), 2);
}

#[test]
fn test_part2() {
    let input = vec![
        String::from("2-4,6-8"),
        String::from("2-3,4-5"),
        String::from("5-7,7-9"),
        String::from("2-8,3-7"),
        String::from("6-6,4-6"),
        String::from("2-6,4-8"),
    ];
    assert_eq!(part2(&input), 4);
}

