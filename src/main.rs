use std::collections::HashSet;

fn main() {
    day1a();
    day1b();
    day2a();
    day2b();
    day3a();
    day3b();
    day4a();
    day4b();
}

fn day1a() {
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

    let max_cals = elf_calories
        .iter()
        .map(|cals| cals.iter().sum::<i64>())
        .max()
        .unwrap();
    println!("{}", max_cals);
}

fn day1b() {
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

    let mut cals = elf_calories
        .iter()
        .map(|cals| cals.iter().sum::<i64>())
        .collect::<Vec<_>>();

    cals.select_nth_unstable_by(2, |a, b| b.cmp(a));

    let result = cals[0] + cals[1] + cals[2];

    println!("{}", result);
}

fn day2a() {
    const INPUT: &str = include_str!("../input/day2.txt");

    let rounds = INPUT
        .split("\n")
        .map(|row| (row.chars().nth(0).unwrap(), row.chars().nth(2).unwrap()))
        .collect::<Vec<_>>();

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
    const INPUT: &str = include_str!("../input/day2.txt");

    let rounds = INPUT
        .split("\n")
        .map(|row| (row.chars().nth(0).unwrap(), row.chars().nth(2).unwrap()))
        .collect::<Vec<_>>();

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

fn day3a() {
    const INPUT: &str = include_str!("../input/day3.txt");

    let bags: Vec<(&str, &str)> = INPUT
        .split("\n")
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
    const INPUT: &str = include_str!("../input/day3.txt");

    let bags = INPUT.split("\n").collect::<Vec<_>>();
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

fn day4a() {
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
