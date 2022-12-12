use std::collections::VecDeque;
use std::cell::RefCell;

fn main() {
    let mut monkeys: Vec<Monkey> = Vec::new();
    monkeys.push(Monkey::new(vec![71, 56, 50, 73], |old| old * 11, |worry| worry % 13 == 0, 1, 7));
    monkeys.push(Monkey::new(vec![70, 89, 82], |old| old + 1, |worry| worry % 7 == 0, 3, 6));
    monkeys.push(Monkey::new(vec![52, 95], |old| old * old, |worry| worry % 3 == 0, 5, 4));
    monkeys.push(Monkey::new(vec![94, 64, 69, 87, 70], |old| old + 2, |worry| worry % 19 == 0, 2, 6));
    monkeys.push(Monkey::new(vec![98, 72, 98, 53, 97, 51], |old| old + 6, |worry| worry % 5 == 0, 0, 5));
    monkeys.push(Monkey::new(vec![79], |old| old + 7, |worry| worry % 2 == 0, 7, 0));
    monkeys.push(Monkey::new(vec![77, 55, 63, 93, 66, 90, 88, 71], |old| old * 7, |worry| worry % 11 == 0, 2, 4));
    monkeys.push(Monkey::new(vec![54, 97, 87, 70, 59, 82, 59], |old| old + 8, |worry| worry % 17 == 0, 1, 3));
    println!("Part 1: {}", part1(&mut monkeys));
    println!("Part 2: {}", part2(&mut monkeys));
}

struct Monkey {
    items: RefCell<VecDeque<usize>>,
    operation: fn(usize) -> usize,
    test: fn(usize) -> bool,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn new(items: Vec<usize>, operation: fn(usize) -> usize, test: fn(usize) -> bool, if_true: usize, if_false: usize) -> Monkey {
        Monkey {
            items: RefCell::new(VecDeque::from(items)),
            operation,
            test,
            if_true,
            if_false,
        }
    }
}

fn part1(monkeys: &mut Vec<Monkey>) -> usize {
    let mut inspections: Vec<usize> = vec![0; monkeys.len()];
    
    for _ in 0..20 {
        for (idx, m) in monkeys.iter().enumerate() {
            inspections[idx] += m.items.borrow().len();
            while let Some(worry) = m.items.borrow_mut().pop_front() {
                let worry = (m.operation)(worry);
                let worry = worry / 3;
                if (m.test)(worry) {
                    monkeys[m.if_true].items.borrow_mut().push_back(worry);
                } else {
                    monkeys[m.if_false].items.borrow_mut().push_back(worry);
                }
            }
        }
    }

    inspections.sort();
    inspections[inspections.len() - 2] * inspections[inspections.len() - 1]
}

fn part2(monkeys: &mut [Monkey]) -> usize {
    0
}

#[test]
fn test_part1() {
    let mut monkeys: Vec<Monkey> = Vec::new();
    monkeys.push(Monkey::new(vec![79, 98], |old| old * 19, |worry| worry % 23 == 0, 2, 3));
    monkeys.push(Monkey::new(vec![54, 65, 75, 74], |old| old + 6, |worry| worry % 19 == 0, 2, 0));
    monkeys.push(Monkey::new(vec![79, 60, 97], |old| old * old, |worry| worry % 13 == 0, 1, 3));
    monkeys.push(Monkey::new(vec![74], |old| old + 3, |worry| worry % 17 == 0, 0, 1));
    assert_eq!(part1(&mut monkeys), 10605);
}

#[test]
fn test_part2() {
    let mut monkeys: Vec<Monkey> = Vec::new();
    monkeys.push(Monkey::new(vec![79, 98], |old| old * 19, |worry| worry % 23 == 0, 2, 3));
    monkeys.push(Monkey::new(vec![54, 65, 75, 74], |old| old + 6, |worry| worry % 19 == 0, 2, 0));
    monkeys.push(Monkey::new(vec![79, 60, 97], |old| old * old, |worry| worry % 13 == 0, 1, 3));
    monkeys.push(Monkey::new(vec![74], |old| old + 3, |worry| worry % 17 == 0, 0, 1));
    assert_eq!(part2(&mut monkeys), 0);
}

