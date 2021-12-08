use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

struct Entry<'a> {
    input: Vec<&'a str>,
    output: Vec<&'a str>,
}

fn main() {
    let example = parse_input(include_str!("example.txt"));
    let input = parse_input(include_str!("input.txt"));
    solve(&example);
    solve(&input);
}

fn parse_input(s: &str) -> Vec<Entry> {
    s.lines()
        .map(|l| -> Entry {
            let mut it = l.split("|");
            Entry {
                input: it.next().unwrap().split_whitespace().collect(),
                output: it.next().unwrap().split_whitespace().collect(),
            }
        })
        .collect()
}

fn solve(input: &Vec<Entry>) {
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<Entry>) {
    let unique_sizes: HashSet<usize> = [2, 3, 4, 7].iter().cloned().collect();
    let ans = input
        .iter()
        .flat_map(|l| l.output.iter().map(|it| it.len()))
        .filter(|l| unique_sizes.contains(l))
        .count();
    println!("Part 1 {}", ans);
}

fn part2(entries: &Vec<Entry>) {
    let unique_sizes: HashMap<i32, i32> =
        [(2, 1), (3, 7), (4, 4), (7, 8)].iter().cloned().collect();
    let mut ans = 0;

    let mut encoding: HashMap<i32, &str> = HashMap::new();

    for entry in entries {
        let mut all_fives: Vec<&str> = Vec::new();
        let mut all_sixes: Vec<&str> = Vec::new();
        for pattern in entry.input.iter() {
            let len = pattern.len() as i32;
            if unique_sizes.contains_key(&len) {
                encoding.insert(unique_sizes[&len], pattern);
            } else if len == 5 {
                all_fives.push(pattern);
            } else {
                all_sixes.push(pattern);
            }
        }
        process_len_five(&all_fives, &mut encoding);
        process_len_six(&all_sixes, &mut encoding);
        ans += decode(&entry.output, &encoding)
    }
    println!("Part 2 {}", ans);
}

fn process_len_five<'a>(all: &Vec<&'a str>, encoding: &mut HashMap<i32, &'a str>) {
    let mut pending: HashSet<&str> = HashSet::new();
    for i in all.iter() {
        pending.insert(i);
    }
    let one_set: HashSet<char> = HashSet::from_iter(encoding[&1].chars());
    for p in pending.iter() {
        let s: HashSet<char> = HashSet::from_iter(p.chars());
        if s.intersection(&one_set).count() == one_set.len() {
            encoding.insert(3, p);
            break;
        }
    }
    pending.remove(encoding[&3]);

    let four_set: HashSet<char> = HashSet::from_iter(encoding[&4].chars());
    for p in pending.iter() {
        let s: HashSet<char> = HashSet::from_iter(p.chars());
        if s.intersection(&four_set).count() == 3 {
            encoding.insert(5, p);
            break;
        }
    }
    pending.remove(encoding[&5]);
    assert!(pending.len() == 1);
    encoding.insert(2, pending.iter().next().unwrap());
}

fn process_len_six<'a>(all: &Vec<&'a str>, encoding: &mut HashMap<i32, &'a str>) {
    let mut pending: HashSet<&str> = HashSet::new();
    for i in all.iter() {
        pending.insert(i);
    }
    let one_set: HashSet<char> = HashSet::from_iter(encoding[&1].chars());
    for p in pending.iter() {
        let s: HashSet<char> = HashSet::from_iter(p.chars());
        if s.intersection(&one_set).count() == 1 {
            encoding.insert(6, p);
            break;
        }
    }
    pending.remove(encoding[&6]);

    let four_set: HashSet<char> = HashSet::from_iter(encoding[&4].chars());
    for p in pending.iter() {
        let s: HashSet<char> = HashSet::from_iter(p.chars());
        if s.intersection(&four_set).count() == four_set.len() {
            encoding.insert(9, p);
            break;
        }
    }
    pending.remove(encoding[&9]);
    assert!(pending.len() == 1);
    encoding.insert(0, pending.iter().next().unwrap());
}

fn decode(output: &Vec<&str>, encoding: &HashMap<i32, &str>) -> i32 {
    let mut ans = 0;
    for d in output.iter() {
        let d_set: HashSet<char> = HashSet::from_iter(d.chars());
        for e in encoding.iter() {
            if HashSet::from_iter(e.1.chars()) == d_set {
                ans *= 10;
                ans += e.0;
            }
        }
    }
    ans
}
