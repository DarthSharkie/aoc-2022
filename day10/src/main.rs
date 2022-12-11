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
    let lines: Vec<Input> = load_file("/mnt/s/AdventOfCode/2022/input10.txt").unwrap();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

enum Type {
    Noop,
    Addx(isize),
}
struct Instruction {
    ins_type: Type,
    cycles: usize,
}

fn part1(contents: &[Input]) -> isize {
    let mut instructions = read_ins(contents);
    let mut ins_iter = instructions.iter_mut();
    let mut x: isize = 1;
    let mut cycle: isize = 0;
    let cycles: Vec<isize> = vec![20, 60, 100, 140, 180, 220];
    let mut values: Vec<isize> = Vec::new();
    // part 2
    let mut crt: isize = 0;

    while let Some(ins) = ins_iter.next() {
        while ins.cycles > 0 {
            ins.cycles -= 1;
            if x.abs_diff(crt) <= 1 {
                print!("#");
            } else {
                print!(".");
            }
            crt += 1;
            crt %= 40;
            if crt == 0 {
                println!("");
            }
            cycle += 1;

            if cycles.contains(&cycle) {
                values.push(x);
            }
            match ins.ins_type {
                Type::Noop => (),
                Type::Addx(dx) => {
                    if ins.cycles == 0 {
                        x += dx;
                    }
                },
            }
        }
    }
    println!("Values: {values:?}");
    values.iter().enumerate().map(|(idx, val)| val * cycles[idx]).sum()
}

fn part2(contents: &[Input]) -> usize {
    0
}

fn read_ins(lines: &[Input]) -> Vec<Instruction> {
    lines.iter().map(|line| {
        match line.split_once(' ') {
            None => Instruction { ins_type: Type::Noop, cycles: 1, },
            Some(("addx", x)) => Instruction { ins_type: Type::Addx(x.parse::<isize>().unwrap()), cycles: 2, },
            Some(_) => panic!("Instruction not understood!"),
        }
    }).collect()
}

#[test]
fn test_part1() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    assert_eq!(part1(&lines), 13140);
}

#[test]
fn test_part2() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    assert_eq!(part2(&lines), 0);
}

