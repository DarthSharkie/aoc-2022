use std::fs;
use std::time::Instant;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::cmp::{Ord, Ordering, max};

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
    let start = Instant::now();
    let lines: Vec<Input> = load_file("/mnt/s/AdventOfCode/2022/input13.txt").unwrap();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
    let elapsed = start.elapsed();
    println!("Elapsed: {}µs", elapsed.as_micros());
}

fn part1(lines: &[Input]) -> usize {
    let packet_pairs = parse(lines);
    packet_pairs.iter().enumerate().map(|(idx, (left, right))| {
        if left.cmp(right) == Ordering::Greater {
            0
        } else {
            idx + 1
        }
    }).sum()
}

fn part2(lines: &[Input]) -> usize {
    let mut packets: Vec<Packet> = lines.iter()
        .filter(|line| line.len() > 0)
        .map(|line| Packet::from_str(line).expect("Bad input!"))
        .collect();
    let packet2 = Packet::List(vec![Packet::List(vec![Packet::Cons(2)])]);
    let packet2_copy = Packet::List(vec![Packet::List(vec![Packet::Cons(2)])]);
    let packet6 = Packet::List(vec![Packet::List(vec![Packet::Cons(6)])]);
    let packet6_copy = Packet::List(vec![Packet::List(vec![Packet::Cons(6)])]);
    packets.push(packet2);
    packets.push(packet6);
    packets.sort();

    let idx2 = packets.binary_search(&packet2_copy).expect("Didn't find packet2");
    let idx6 = packets.binary_search(&packet6_copy).expect("Didn't find packet6");
    
    (idx2+1)*(idx6+1)
}

#[derive(Debug, Eq)]
enum Packet {
    List(Vec<Packet>),
    Cons(usize),
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().peekable();
        let mut packets: Vec<Vec<Packet>> = Vec::new();
        let mut curr: Vec<Packet> = Vec::new();
        while let Some(c) = chars.next() {
            match c {
                '[' => {
                    packets.push(curr);
                    curr = Vec::new()
                },
                ']' => {
                    let list = Packet::List(curr);
                    curr = packets.pop().expect("stack should not be empty!");
                    curr.push(list);
                },
                '0'..='9' => {
                    let mut value = c as u8 - b'0';
                    while let Some(p) = chars.peek() {
                        match p {
                            '0'..='9' => { 
                                value = 10*value + (*p as u8 - b'0');
                                // Consume the next
                                let _ = chars.next();
                            },
                            _ => break,
                        }
                    }
                    curr.push(Packet::Cons(value.into()));
                },
                _ => (),
            }
        }
        Ok(curr.pop().expect("Should be a value"))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(l1), Packet::List(l2)) => {
                for idx in 0..max(l1.len(), l2.len()) {
                    let order = match (l1.get(idx), l2.get(idx)) {
                        (Some(l_val), Some(r_val)) => l_val.cmp(r_val),
                        (Some(_), None) => Ordering::Greater,
                        (None, Some(_)) => Ordering::Less,
                        (None, None) => Ordering::Equal,
                    };
                    if order != Ordering::Equal {
                        return order;
                    }
                }
                Ordering::Equal
            },
            (Packet::List(_), Packet::Cons(c2)) => self.cmp(&Packet::List(vec![Packet::Cons(*c2)])),
            (Packet::Cons(c1), Packet::List(_)) => Packet::List(vec![Packet::Cons(*c1)]).cmp(other),
            (Packet::Cons(c1), Packet::Cons(c2)) => c1.cmp(c2),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn parse(lines: &[Input]) -> Vec<(Packet, Packet)> {
    let mut iter = lines.split(|line| line.len() == 0);
    let mut pairs: Vec<(Packet, Packet)> = Vec::new();

    while let Some(chunk) = iter.next() {
        if let [left, right] = chunk {
            let left_packet = Packet::from_str(left);
            let right_packet = Packet::from_str(right);
            match (left_packet, right_packet) {
                (Ok(left), Ok(right)) => pairs.push((left, right)),
                _ => panic!("no pairs!"),
            }
        } else {
            panic!("Bad input!");
        }
    }
    pairs
}


#[test]
fn test_part1() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    assert_eq!(part1(&lines), 13);
}

#[test]
fn test_part2() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    assert_eq!(part2(&lines), 140);
}

