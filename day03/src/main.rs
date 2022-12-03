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
    let filename = String::from("/mnt/s/AdventOfCode/2022/input03.txt");
    let lines: Vec<Input> = load_file(&filename).unwrap();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(contents: &[Input]) -> usize {
    contents.iter().map(|c| {
        let (left, right) = c.split_at(c.len() / 2);
        for l in left.chars() {
            if right.contains(l) {
                return priority(&l);
            }
        }
        0
    }).sum()
}

fn part2(contents: &[Input]) -> usize {
    contents.chunks(3).map(|chunk| {
        let needle = chunk.get(0).unwrap();
        let h1 = chunk.get(1).unwrap();
        let h2 = chunk.get(2).unwrap();
        for n in needle.chars() {
            if h1.contains(n) && h2.contains(n) {
                return priority(&n);
            }
        }
        0
    }).sum()
}

fn priority(c: &char) -> usize {
    match c {
        'a'..='z' => (*c as usize) - 96,
        'A'..='Z' => (*c as usize) - 64 + 26,
        _ => 0
    }
}

#[test]
fn test_part1() {
    let input = vec![
        String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
        String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
        String::from("PmmdzqPrVvPwwTWBwg"),
        String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
        String::from("ttgJtRGJQctTZtZT"),
        String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
    ];
    assert_eq!(part1(&input), 157);
}

#[test]
fn test_part2() {
    let input = vec![
        String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
        String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
        String::from("PmmdzqPrVvPwwTWBwg"),
        String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
        String::from("ttgJtRGJQctTZtZT"),
        String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
    ];
    assert_eq!(part2(&input), 70);
}

