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
    let lines: Vec<Input> = load_file("/mnt/s/AdventOfCode/2022/input08.txt").unwrap();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(contents: &[Input]) -> usize {
    let grid: Vec<Vec<u8>> = contents.iter().map(|s| s.chars().map(|c| (c as u8) - 32).collect()).collect();

    let mut visible: usize = 4 * (grid.len() - 1);

    for r in 1..=grid.len() - 2 {
        let row = &grid[r];
        for c in 1..=row.len() - 2 {
            // Left
            let left = (0..c).all(|h| grid[r][h] < grid[r][c]);
            // Up
            let up = (0..r).all(|h| grid[h][c] < grid[r][c]);
            // Right
            let right = (c+1..grid.len()).all(|h| grid[r][h] < grid[r][c]);
            // Down
            let down = (r+1..row.len()).all(|h| grid[h][c] < grid[r][c]);
            if left || up || right || down {
                println!("{r}, {c}");
                visible += 1;
            }
        }
    }

    visible
}

fn part2(contents: &[Input]) -> usize {
    let grid: Vec<Vec<u8>> = contents.iter().map(|s| s.chars().map(|c| (c as u8) - 32).collect()).collect();

    let mut scenic = 0;
    scenic
}

#[test]
fn test_part1() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    assert_eq!(part1(&lines), 21);
}

#[test]
fn test_part2() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    assert_eq!(part2(&lines), 8);
}

