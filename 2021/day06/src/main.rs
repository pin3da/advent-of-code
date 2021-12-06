use std::collections::VecDeque;

fn main() {
    let example = parse_input(include_str!("example.txt"));
    let input = parse_input(include_str!("input.txt"));
    solve(&example);
    solve(&input);
}

fn parse_input(s: &str) -> Vec<i32> {
    s.lines()
        .flat_map(|l| l.split(","))
        .map(|i| i.parse().unwrap())
        .collect()
}

fn solve(input: &Vec<i32>) {
    let mut freq: VecDeque<usize> = VecDeque::new();
    for _ in 0..9 {
        freq.push_back(0);
    }
    for i in input.iter() {
        freq[*i as usize] += 1
    }
    for _ in 0..256 {
        let today = freq.pop_front().unwrap();
        freq.push_back(today);
        freq[6] += today;
    }
    println!("{}", freq.iter().sum::<usize>())
}
