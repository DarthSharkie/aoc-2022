use std::fs;
use std::time::Instant;
use std::io::{self, BufRead};
use std::collections::HashSet;

type Input = String;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn manhattan(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn load_file(filename: &str) -> io::Result<Vec<Input>> {
    fs::File::open(filename).map(|file| {
        io::BufReader::new(file)
            .lines()
            .map(|line| line.unwrap().parse::<Input>().unwrap())
            .collect()
    })
}

fn make_pairings(lines: &[Input]) -> Vec<(Pos, Pos)> {
    lines.iter().map(|line| {
        let words: Vec<&str> = line.split(' ').collect();
        let sensor = Pos { x: extract_number(words[2]), y: extract_number(words[3]) };
        let beacon = Pos { x: extract_number(words[8]), y: extract_number(words[9]) };
        (sensor, beacon)
    }).collect()
}

fn extract_number(s: &str) -> isize {
    s.chars().filter(|c| c.is_ascii_digit() || *c == '-').fold(String::new(), |mut a, b| {a.push(b); a}).parse().expect("Not a number")
}

fn main() {
    let start = Instant::now();
    let lines: Vec<Input> = load_file("/mnt/s/AdventOfCode/2022/input15.txt").unwrap();

    let pairings: Vec<(Pos, Pos)> = make_pairings(&lines);

    println!("Part 1: {}", part1(&pairings, 2_000_000));
    println!("Part 2: {}", part2(&pairings, 0, 4_000_000));

    let elapsed = start.elapsed();
    println!("Elapsed: {}µs", elapsed.as_micros());
}

fn part1(pairings: &[(Pos, Pos)], row: isize) -> usize {
    let mut nope = HashSet::new();
    pairings.iter().for_each(|(sensor, beacon)| {
        if (row - sensor.y).abs() <= sensor.manhattan(&beacon) {
            let vertical = (row - sensor.y).abs();
            let horizontal = sensor.manhattan(&beacon) - vertical;
            (-horizontal..=horizontal).for_each(|x| { nope.insert(sensor.x + x); });
        }
    });
    pairings.iter().for_each(|(_sensor, beacon)| {
        if beacon.y == row {
            nope.remove(&beacon.x);
        }
    });
    nope.len()
}

fn part2(pairings: &[(Pos, Pos)], lower: isize, upper: isize) -> usize {
    // Compute line segments that are manhattan+1 from sensors
    // Find intersections of segments
    // Check MD of intersections from sensors relative to the closest beacon

    // Create positively-sloped diagonals
    let pos_diags: Vec<(Pos, Pos)> = pairings.iter().map(|(sensor, beacon)| {
        let md = sensor.manhattan(&beacon);
        vec![(sensor.x - md - 1, sensor.y, sensor.x, sensor.y + md + 1),
             (sensor.x, sensor.y - md - 1, sensor.x + md + 1, sensor.y)]
    })
    .flatten()
    .map(|(mut x1, mut y1, mut x2, mut y2)| {
        if x1 < lower {
            y1 += (lower - x1);
            x1 = lower;
        }
        if y1 < lower {
            x1 += (lower - y1);
            y1 = lower;
        }
        if x2 > upper {
            y2 -= (x2 - upper);
            x2 = upper;
        }
        if y2 > upper {
            x2 -= (y2 - upper);
            y2 = upper;
        }
        (x1, y1, x2, y2)
    })
    .filter(|(x1, y1, x2, y2)| x1 <= x2)
    .map(|(x1, y1, x2, y2)| (Pos { x: x1, y: y1 }, Pos { x: x2, y: y2 }))
    .collect();
    pos_diags.iter().for_each(|p| println!("{p:?}"));

    // Create negatively-sloped diagonals
    let neg_diags: Vec<(Pos, Pos)> = pairings.iter().map(|(sensor, beacon)| {
        let md = sensor.manhattan(&beacon);
        vec![(sensor.x - md - 1, sensor.y, sensor.x, sensor.y - md - 1),
             (sensor.x, sensor.y + md + 1, sensor.x + md + 1, sensor.y)]
    })
    .flatten()
    .map(|(mut x1, mut y1, mut x2, mut y2)| {
        if x1 < lower {
            y1 -= (lower - x1);
            x1 = lower;
        }
        if y1 > upper {
            x1 += (y1 - upper);
            y1 = upper;
        }
        if x2 > upper {
            y2 += (x2 - upper);
            x2 = upper;
        }
        if y2 < lower {
            x2 -= (lower - y2);
            y2 = lower;
        }
        (x1, y1, x2, y2)
    })
    .filter(|(x1, y1, x2, y2)| x1 <= x2)
    .map(|(x1, y1, x2, y2)| (Pos { x: x1, y: y1 }, Pos { x: x2, y: y2 }))
    .collect();

    // For each positive, see if it intersects a negative
    let possible_positions: HashSet<Pos> = pos_diags.iter().flat_map(|(p1, p2)| {
        neg_diags.iter().filter(|(n1, n2)| {
            let n = n1.x + n1.y;
            // Sum of coordinates must satisfy p1 <= n <= p2 to intersect
            // and given discrete grid, the difference must be even.  If odd, the intersection
            // would be between grid locations (at a ".5").
            p1.x + p1.y <= n && n <= p2.x + p2.y && (p1.x + p1.y - n) % 2 == 0
        }).map(|(n1, n2)| {
            // If so, then the half the difference from p1 coordinate sum to n1 coordinate sum
            // is added to each p1 coordinate to produce the intersection point.  This point is
            // on two perpendicular diagonals, so worth further examination.
            let n = n1.x + n1.y;
            let delta = (n - (p1.x + p1.y)) / 2;
            Pos { x: p1.x + delta, y: p1.y + delta }
        })
    }).collect::<HashSet<Pos>>();

    // Now, filter the possibles by distance to any sensor, compared to its beacon's MD
    let remaining_positions: Vec<&Pos> = possible_positions.iter().filter(|possible| {
        // Any sensor with a beacon the same or further MD away eliminates this possible location
        !pairings.iter().any(|(sensor, beacon)| sensor.manhattan(&beacon) >= sensor.manhattan(&possible))
    }).collect();
    println!("{remaining_positions:?}");
    (remaining_positions[0].x * upper + remaining_positions[0].y) as usize
}

#[test]
fn test_part1() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    let pairings: Vec<(Pos, Pos)> = make_pairings(&lines);
    assert_eq!(part1(&pairings, 10), 26);
}

#[test]
fn test_part2() {
    let lines: Vec<Input> = load_file("test.txt").unwrap();
    let pairings: Vec<(Pos, Pos)> = make_pairings(&lines);
    assert_eq!(part2(&pairings, 0, 20), 56000011);
}

