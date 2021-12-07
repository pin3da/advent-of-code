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
    let mut crabs = input.clone();
    crabs.sort();
    part1(&crabs);
    // println!("{:?}", crabs);
    part2(&crabs);
}

fn part1(crabs: &Vec<i32>) {
    let target = crabs[crabs.len() / 2];
    let ans: i32 = crabs.iter().map(|c| (c - target).abs()).sum();
    println!("{}", ans);
}

fn part2(crabs: &Vec<i32>) {
    let mut best = std::i32::MAX;
    for i in 0..1001 {
        let ans: i32 = crabs.iter().map(|c| sum((c - i).abs())).sum();
        best = std::cmp::min(best, ans);
    }
    println!("{}", best);
}

fn sum(x: i32) -> i32 {
    (x * (x + 1)) / 2
}
