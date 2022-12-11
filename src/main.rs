use std::collections::{HashMap, HashSet};

use regex::Regex;

fn main() {
    day1a();
    day1b();
    day2a();
    day2b();
    day3a();
    day3b();
    day4a();
    day4b();
    day5a();
    day5b();
}

fn day1_process_data() -> Vec<Vec<i64>> {
    const INPUT: &str = include_str!("../input/day1.txt");
    let elf_calories = INPUT
        .split("\n\n")
        .map(|block| {
            block
                .trim()
                .split("\n")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    elf_calories
}

fn day1a() {
    let elf_calories = day1_process_data();

    let max_cals = elf_calories
        .iter()
        .map(|cals| cals.iter().sum::<i64>())
        .max()
        .unwrap();
    println!("{}", max_cals);
}

fn day1b() {
    let elf_calories = day1_process_data();

    let mut cals = elf_calories
        .iter()
        .map(|cals| cals.iter().sum::<i64>())
        .collect::<Vec<_>>();

    cals.select_nth_unstable_by(2, |a, b| b.cmp(a));

    let result = cals[0] + cals[1] + cals[2];

    println!("{}", result);
}

fn day2_process_data() -> Vec<(char, char)> {
    const INPUT: &str = include_str!("../input/day2.txt");
    let rounds = INPUT
        .split("\n")
        .map(|row| (row.chars().nth(0).unwrap(), row.chars().nth(2).unwrap()))
        .collect::<Vec<_>>();
    rounds
}

fn day2a() {
    let rounds = day2_process_data();

    let result = rounds
        .iter()
        .map(|(a, b)| {
            return match b {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => unreachable!(),
            } + match (a, b) {
                ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
                ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
                ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
                _ => unreachable!(),
            };
        })
        .sum::<i64>();

    println!("{}", result);
}

fn day2b() {
    let rounds = day2_process_data();

    let result = rounds
        .iter()
        .map(|(a, b)| {
            return match b {
                'X' => 0,
                'Y' => 3,
                'Z' => 6,
                _ => unreachable!(),
            } + match (a, b) {
                ('A', 'X') | ('B', 'Z') | ('C', 'Y') => 3, // scissors
                ('A', 'Z') | ('B', 'Y') | ('C', 'X') => 2, // paper
                ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 1, // rock
                _ => unreachable!(),
            };
        })
        .sum::<i64>();

    println!("{}", result);
}

fn common_chars<'a, 'b>(lhs: &'a str, rhs: &'a str) -> String {
    let set_lhs: HashSet<char> = lhs.chars().collect();
    let set_rhs: HashSet<char> = rhs.chars().collect();

    return set_lhs.intersection(&set_rhs).cloned().collect();
}

fn day3_process_data() -> Vec<&'static str> {
    const INPUT: &str = include_str!("../input/day3.txt");
    return INPUT.split("\n").collect::<Vec<_>>();
}

fn day3a() {
    let contents = day3_process_data();

    let bags: Vec<(&str, &str)> = contents
        .iter()
        .map(|row| row.split_at(row.len() / 2))
        .collect::<Vec<_>>();

    let common_items = bags
        .iter()
        .map(|(left, right)| {
            let common = common_chars(left, right);
            assert!(common.len() == 1);
            let val = common.chars().nth(0).unwrap();
            return val;
        })
        .collect::<Vec<_>>();

    let result = common_items
        .iter()
        .map(|ch| {
            return match ch {
                'a'..='z' => *ch as i64 - 'a' as i64 + 1,
                'A'..='Z' => *ch as i64 - 'A' as i64 + 27,
                _ => unreachable!(),
            };
        })
        .sum::<i64>();

    println!("{}", result);
}

fn day3b() {
    let bags = day3_process_data();
    let triples = (&bags[..]).chunks_exact(3);

    let result = triples
        .map(|bags| {
            let common = common_chars(bags[0], bags[1]);
            let common = common_chars(&common, bags[2]);
            assert!(common.len() == 1);
            let val = common.chars().nth(0).unwrap();
            return val;
        })
        .map(|ch| {
            return match ch {
                'a'..='z' => ch as i64 - 'a' as i64 + 1,
                'A'..='Z' => ch as i64 - 'A' as i64 + 27,
                _ => unreachable!(),
            };
        })
        .sum::<i64>();

    println!("{}", result);
}

struct Assignment {
    start: i32,
    end: i32,
}

fn day4_process_data() -> Vec<(Assignment, Assignment)> {
    const INPUT: &str = include_str!("../input/day4.txt");
    let assignment_pairs = INPUT
        .split("\n")
        .map(|line| {
            line.split(',')
                .map(|assignment| {
                    assignment
                        .split('-')
                        .map(|v| v.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|vv| {
            (
                Assignment {
                    start: vv[0][0],
                    end: vv[0][1],
                },
                Assignment {
                    start: vv[1][0],
                    end: vv[1][1],
                },
            )
        })
        .collect::<Vec<_>>();
    assignment_pairs
}

fn day4a() {
    let assignment_pairs = day4_process_data();

    let result = assignment_pairs
        .iter()
        .filter(|(ass1, ass2)| {
            (ass1.start >= ass2.start && ass1.end <= ass2.end)
                || (ass2.start >= ass1.start && ass2.end <= ass1.end)
        })
        .count();

    println!("{}", result);
}

fn day4b() {
    let assignment_pairs = day4_process_data();

    let result = assignment_pairs
        .iter()
        .filter(|(ass1, ass2)| {
            ass1.start <= ass2.start && ass1.end >= ass2.start
                || ass2.start <= ass1.start && ass2.end >= ass1.start
                || ass1.start >= ass2.start && ass1.start <= ass2.end
                || ass2.start >= ass1.start && ass2.start <= ass1.end
        })
        .count();

    println!("{}", result);
}

fn day5_process_data() -> (Vec<(usize, usize, usize)>, [Vec<char>; 9]) {
    const INPUT: &str = include_str!("../input/day5.txt");
    // decompose input file into initial crates section, and moves section
    let mut it = INPUT
        .split("\n\n")
        .map(|chunk| chunk.split('\n').collect::<Vec<_>>());
    let initial = it.next().unwrap();
    let moves = it.next().unwrap();
    // compute the index offsets using the last row of initial position
    let indexes = initial[initial.len() - 1]
        .chars()
        .enumerate()
        .filter(|(_pos, ch)| *ch != ' ')
        .map(|(pos, ch)| (pos, ch as usize - '0' as usize))
        .collect::<HashMap<_, _>>();
    let mut stacks: [Vec<char>; 9] = Default::default();
    // build the initial stack positions by stepping
    initial[..initial.len() - 1].iter().rev().for_each(|row| {
        indexes.iter().for_each(|(pos, stack)| {
            let ch = row.chars().nth(*pos).unwrap();
            if ch.is_alphabetic() {
                stacks[stack - 1].push(ch);
            }
        })
    });

    let re = Regex::new(r"^move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)$").unwrap();
    let moves = moves
        .iter()
        .map(|move_str| {
            re.captures(*move_str).map(|cap| {
                let count: usize = cap
                    .name("count")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("error parsing count");
                let from: usize = cap
                    .name("from")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("error parsing from");
                let to: usize = cap
                    .name("to")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("error parsing to");
                return (count, from, to);
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    (moves, stacks)
}

fn day5a() {
    let (moves, mut stacks) = day5_process_data();

    moves.iter().for_each(|(count, from, to)| {
        for _ in 0..*count {
            let c = stacks[*from - 1].pop().unwrap();
            stacks[*to - 1].push(c);
        }
    });

    let result = stacks
        .iter()
        .map(|s| s.last())
        .flatten()
        .collect::<String>();

    println!("{}", result);
}

fn day5b() {
    let (moves, mut stacks) = day5_process_data();

    moves.iter().for_each(|(count, from, to)| {
        let to_move: Vec<_> = stacks[*from - 1]
            .drain(stacks[*from - 1].len() - *count..)
            .collect();
        stacks[*to - 1].extend(to_move);
    });

    let result = stacks
        .iter()
        .map(|s| s.last())
        .flatten()
        .collect::<String>();

    println!("{}", result);
}
