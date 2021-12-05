use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
struct Line {
    start: Point,
    end: Point,
}

impl Point {
    fn from_str(s: &str) -> Point {
        let mut coord = s.split(",");
        Point {
            x: coord.next().unwrap().parse().unwrap(),
            y: coord.next().unwrap().parse().unwrap(),
        }
    }
}

impl FromStr for Line {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split_whitespace();
        let start = Point::from_str(line.next().unwrap());
        // Skip "->"
        line.next();
        let end = Point::from_str(line.next().unwrap());
        Ok(Line { start, end })
    }
}

fn main() {
    let example = parse_input(include_str!("example.txt"));
    let input = parse_input(include_str!("input.txt"));
    solve(&example);
    solve(&input);
}

fn parse_input(input: &str) -> Vec<Line> {
    let lines: Vec<Line> = input.lines().map(|l| l.parse().unwrap()).collect();
    lines
}

fn solve(input: &Vec<Line>) {
    mark(
        &(input
            .iter()
            .filter(|l| l.start.x == l.end.x || l.start.y == l.end.y)
            .map(|l| *l)
            .collect()),
    );
    mark(input);
}

fn mark(input: &Vec<Line>) {
    let mut grid = [[0; 1000]; 1000];
    for line in input {
        let dx = norm(line.end.x - line.start.x);
        let dy = norm(line.end.y - line.start.y);
        let mut x = line.start.x;
        let mut y = line.start.y;
        while x != line.end.x || y != line.end.y {
            grid[x as usize][y as usize] += 1;
            x += dx;
            y += dy;
        }
        grid[line.end.x as usize][line.end.y as usize] += 1
    }
    let ans: usize = grid
        .iter()
        .map(|row| row.iter().filter(|cell| **cell > 1).count())
        .sum();
    println!("{:}", ans);
}

fn norm(x: i32) -> i32 {
    if x == 0 {
        0
    } else {
        x / x.abs()
    }
}
