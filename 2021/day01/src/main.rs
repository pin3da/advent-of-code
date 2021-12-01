fn main() {
    let input = parse_ints(include_str!("input.txt"));
    let example = parse_ints(include_str!("example.txt"));
    println!("part 1");
    part1(&input);
    part1(&example);
    println!("part 2");
    part2(&input);
    part2(&example);
}

fn parse_ints(input: &str) -> Vec<i32> {
    return input.lines().map(|it| it.parse().unwrap()).collect();
}

fn part1(input: &Vec<i32>) {
    let mut last = std::i32::MAX;
    let mut ans = 0;
    for current in input {
        if *current > last {
            ans = ans + 1;
        }
        last = *current;
    }

    println!("{}", ans);
}

fn part2(input: &Vec<i32>) {
    let mut windows = Vec::new();
    for i in 0..input.len() - 2 {
        let mut current = 0;
        for j in 0..3 {
            current += input[i + j];
        }
        windows.push(current);
    }
    part1(&windows);
}
