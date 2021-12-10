use std::collections::VecDeque;

type Chunk = VecDeque<char>;
type Input = Vec<Chunk>;

fn main() {
    let example = parse_input(include_str!("example.txt"));
    let input = parse_input(include_str!("input.txt"));
    solve(&example);
    solve(&input);
}

fn parse_input(s: &str) -> Input {
    s.lines().map(|it| it.chars().collect()).collect()
}

fn solve(input: &Input) {
    let part1: Vec<(&Chunk, i32)> = input.iter().map(cost_first_error).collect();
    println!("Part 1: {}", part1.iter().map(|i| i.1).sum::<i32>());
    let mut part2: Vec<i64> = part1
        .iter()
        .filter(|i| i.1 == 0)
        .map(|it| it.0)
        .map(cost_fixing)
        .collect();
    part2.sort();
    println!("Part 2: {:?}", part2[part2.len() / 2]);
}

fn cost_first_error(input: &Chunk) -> (&Chunk, i32) {
    let mut matched = Chunk::new();
    for i in input {
        match i {
            '{' | '(' | '[' | '<' => matched.push_back(*i),
            _ => {
                if matched.pop_back().unwrap_or('#') != matching(i) {
                    return (input, cost(i).unwrap());
                }
            }
        }
    }

    return (input, 0);
}

fn cost_fixing(input: &Chunk) -> i64 {
    let mut matched = Chunk::new();
    for i in input {
        match i {
            '{' | '(' | '[' | '<' => matched.push_back(*i),
            _ => {
                assert!(matched.pop_back().unwrap_or('#') == matching(i));
            }
        }
    }
    let mut score: i64 = 0;
    while matched.len() > 0 {
        let cur = cost(&matched.pop_back().unwrap()).unwrap();
        score *= 5;
        score += cur as i64;
    }
    return score;
}

fn cost(c: &char) -> Result<i32, &str> {
    match c {
        ')' => Ok(3),
        ']' => Ok(57),
        '}' => Ok(1197),
        '>' => Ok(25137),
        // cost of adding matching pair.
        '(' => Ok(1),
        '[' => Ok(2),
        '{' => Ok(3),
        '<' => Ok(4),
        _ => Err("invalid char"),
    }
}

fn matching(c: &char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => 'x',
    }
}
