use std::fs;
use std::io::{self, BufRead};
use std::collections::{HashSet, HashMap, VecDeque};
use std::cmp::max;

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
    let lines: Vec<Input> = load_file("/mnt/s/AdventOfCode/2022/input12.txt").unwrap();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn make_grid(lines: &[Input]) -> (HashMap<(usize, usize), char>, (usize, usize), (usize, usize)) {
    let mut s: (usize, usize) = (0, 0);
    let mut e: (usize, usize) = (0, 0);
    let mut grid: HashMap<(usize, usize), char> = HashMap::new();
    lines.iter().enumerate().for_each(|(idx, line)| {
        line.char_indices().for_each(|(idx2, c)| {
            // Add 1 to allow usize to work when checking outside lower grid boundary
            grid.insert((idx + 1, idx2 + 1), c);
            match c {
                'S' => s = (idx + 1, idx2 + 1),
                'E' => e = (idx + 1, idx2 + 1),
                _ => (),
            }
        })
    });
    grid.insert(s, 'a');
    grid.insert(e, 'z');
    (grid, s, e)
}

fn part1(lines: &[Input]) -> usize {
    let (grid, s, e) = make_grid(lines);
    println!("Start: {s:?}");
    println!("End: {e:?}");

    bfs(&grid, &s, &e)
}

fn part2(lines: &[Input]) -> usize {
    0
}

fn bfs(grid: &HashMap<(usize, usize), char>, s: &(usize, usize), e: &(usize, usize)) -> usize {
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
    queue.push_back((*s, 0));
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(*s);
    while let Some((pos, depth)) = queue.pop_front() {
        println!("Pos: {pos:?}, Depth: {depth}");
        if &pos == e {
            return depth;
        }

        
        Vec::from([(pos.0 - 1, pos.1), (pos.0, pos.1 - 1), (pos.0 + 1, pos.1), (pos.0, pos.1 + 1)]).into_iter().for_each(|neighbor| {
            if grid.contains_key(&neighbor) && !visited.contains(&neighbor) && is_step(&grid, &pos, &neighbor) {
                visited.insert(neighbor);
                queue.push_back((neighbor, depth + 1));
            }
        });
    }
    unreachable!();
}

fn is_step(grid: &HashMap<(usize, usize), char>, pos: &(usize, usize), next: &(usize, usize)) -> bool {
    if let Some(c) = grid.get(pos) {
        if let Some(n) = grid.get(next) {
            //println!("{n} <= {c} + 1");
            return *n <= ((*c as u8) + 1) as char;
        }
    }
    return false;
}


#[test]
fn test_part1() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    assert_eq!(part1(&lines), 31);
}

#[test]
fn test_part2() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    assert_eq!(part2(&lines), 0);
}

