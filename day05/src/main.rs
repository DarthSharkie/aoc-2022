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
    let filename = String::from("/mnt/s/AdventOfCode/2022/input05.txt");
    let lines: Vec<Input> = load_file(&filename).unwrap();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn read_stacks(contents: &[Input]) -> (Vec<Vec<char>>, usize) {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _i in 0..10 {
        stacks.push(Vec::new());
    }

    let mut skips = 0;
    for level in contents {
        skips = skips + 1;
        println!("|{}|", level);
        let mut i = 1;
        loop {
            let c = level.chars().nth(i);
            if c == None {
                break;
            }
            match c {
                Some('1') => break,
                Some('A'..='Z') => stacks[(i-1)/4+1].insert(0, c.unwrap()),
                _ => (),
            };
            i = i + 4;
            // let mut buffer = String::new();
            // io::stdin().read_line(&mut buffer);
        }
        if level.len() == 0 {
            break;
        }
    }
    (stacks, skips)
}

fn part1(contents: &[Input]) -> usize {
    let (mut stacks, skips): (Vec<Vec<char>>, usize) = read_stacks(&contents);
    for ins in &contents[skips..] {
        let v: Vec<&str> = ins.split(' ').collect();
        let (count, from, to) = (v[1].parse::<usize>().unwrap(), v[3].parse::<usize>().unwrap(), v[5].parse::<usize>().unwrap());

        for i in 0..count {
            let p = stacks[from].pop().unwrap();
            stacks[to].push(p);
        }
    }

    for stack in &stacks {
        for c in stack {
            print!("{}", c);
        }
        println!("");
    }

    for stack in &stacks {
        match stack.last() {
            Some(x) => print!("{}", x),
            _ => (),
        }
    }
    println!("");

    0
}

fn part2(contents: &[Input]) -> usize {
    let (mut stacks, skips): (Vec<Vec<char>>, usize) = read_stacks(&contents);
    for ins in &contents[skips..] {
        let v: Vec<&str> = ins.split(' ').collect();
        let (count, from, to) = (v[1].parse::<usize>().unwrap(), v[3].parse::<usize>().unwrap(), v[5].parse::<usize>().unwrap());

        let mut tmp: Vec<char> = Vec::new();
        for i in 0..count {
            let p = stacks[from].pop().unwrap();
            tmp.push(p);
        }
        for i in 0..count {
            let p = tmp.pop().unwrap();
            stacks[to].push(p);
        }
    }

    for stack in &stacks {
        for c in stack {
            print!("{}", c);
        }
        println!("");
    }

    for stack in &stacks {
        match stack.last() {
            Some(x) => print!("{}", x),
            _ => (),
        }
    }
    println!("");

    0
}

#[test]
fn test_part1() {
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
    assert_eq!(part1(&input), 0);
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

