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
    let filename = String::from("/mnt/s/AdventOfCode/2022/input02.txt");
    let lines: Vec<Input> = load_file(&filename).unwrap();
    
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(games: &[Input]) -> usize {

    let mut total = 0;

    for game in games {
        let score = match game.as_str() {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => 0,
        };
        total = total + score;
    }      
    total
}

fn part2(games: &[Input]) -> usize {
    let mut total = 0;

    for game in games {
        let score = match game.as_str() {
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => 0,
        };
        total = total + score;
    }
    total
}

#[test]
fn test_part1() {
    let input = vec![String::from("A Y"), String::from("B X"), String::from("C Z")];
    assert_eq!(part1(&input), 15);
}

#[test]
fn test_part2() {
    let input = vec![String::from("A Y"), String::from("B X"), String::from("C Z")];
    assert_eq!(part2(&input), 12);
}

