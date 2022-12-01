use std::fs;
use std::io::{self, BufRead};

type Input = u32;

fn load_file(filename: &str) -> io::Result<Vec<Input>> {
    fs::File::open(filename).map(|file| {
        io::BufReader::new(file)
            .lines()
            .map(|line| line.unwrap().parse::<Input>().unwrap_or_else(|_| 0))
            .collect()
    })
}

fn main() {
    let filename = String::from("/mnt/s/AdventOfCode/2022/input01.txt");
    let lines: Vec<Input> = load_file(&filename).unwrap();
    
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(calories: &[u32]) -> usize {
    let mut load = 0;
    let mut max_load = 0;

    for c in calories {
        match c {
            0 => {
                println!("Total load: {}, current max: {}", load, max_load);
                if load > max_load {
                    max_load = load;
                }
                // Reset for next elf
                load = 0;
            },
            _ => load = load + c,
        }
    }
    max_load.try_into().unwrap()
}

fn part2(calories: &[u32]) -> usize {
    let mut vec = vec![0];

    let mut load = 0;
    for c in calories {
        match c {
            0 => {
                vec.push(load);
                // Reset for next elf
                load = 0;
            },
            _ => load = load + c,
        }
    }

    vec.sort();
    let max = vec.pop().unwrap();
    let max2 = vec.pop().unwrap();
    let max3 = vec.pop().unwrap();
    (max + max2 + max3).try_into().unwrap()
}


