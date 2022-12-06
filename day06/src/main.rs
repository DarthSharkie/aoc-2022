use std::fs;
use std::io::{self, BufRead};
use std::collections::HashSet;

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
    let filename = String::from("/mnt/s/AdventOfCode/2022/input06.txt");
    let lines: Vec<Input> = load_file(&filename).unwrap();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(contents: &[Input]) -> usize {
    let chars: Vec<char> = contents[0].chars().collect();
    for i in 0..chars.len()-3 {
        let mut set: HashSet<char> = HashSet::new();
        for j in 0..4 {
            set.insert(chars[i+j]);
        }
        if set.len() == 4 {
            return i + 4;
        }
    }

    panic!("Bad input!");
}

fn part2(contents: &[Input]) -> usize {
    let chars: Vec<char> = contents[0].chars().collect();
    for i in 0..chars.len()-13 {
        let mut set: HashSet<char> = HashSet::new();
        for j in 0..14 {
            set.insert(chars[i+j]);
        }
        if set.len() == 14 {
            return i + 14;
        }
    }

    panic!("Bad input!");
}

#[test]
fn test_part1() {
    let input = vec![
String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"),
String::from("nppdvjthqldpwncqszvftbrmjlhg"),
String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
    ];
    assert_eq!(part1(&input[0..1]), 7);
    assert_eq!(part1(&input[1..2]), 5);
    assert_eq!(part1(&input[2..3]), 6);
    assert_eq!(part1(&input[3..4]), 10);
    assert_eq!(part1(&input[4..5]), 11);
}

#[test]
fn test_part2() {
    let input = vec![
String::from("    [D]    "),
String::from("[N] [C]    "),
String::from("[Z] [M] [P]"),
String::from(" 1   2   3 "),
String::from(""),
String::from("move 1 from 2 to 1"),
String::from("move 3 from 1 to 3"),
String::from("move 2 from 2 to 1"),
String::from("move 1 from 1 to 2"),
    ];
    assert_eq!(part2(&input), 0);
}

