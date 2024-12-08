use input_parsing::read_input;  
use std::collections::{HashMap, HashSet};
use std::ops::{Sub, Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn inside(&self, rows: usize, cols: usize) -> bool {
        self.x >= 0 && self.x < rows as i32 && self.y >= 0 && self.y < cols as i32
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position { x: self.x + other.x, y: self.y + other.y }
    }
}

fn main() {
    let input = read_input("./src/example.txt");
    let grid : Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut antenna_by_frequency : HashMap<char, Vec<Position>> = HashMap::new();
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != '.' {
                antenna_by_frequency.entry(*c).or_default().push(Position::new(x as i32, y as i32));
            }
        }
    }
    let y_max = grid.len();
    let x_max = grid[0].len();
    println!("y_max: {}, x_max: {}", y_max, x_max);
    let mut part1_antinodes : HashSet<Position> = HashSet::new();
    let mut part2_antinodes = HashSet::new();
    for (_, positions) in &antenna_by_frequency {
        for p in positions.iter() {
            for q in positions.iter() {
                if p == q {
                    continue;
                }
                let delta = *p - *q;
                let mut antinode = *p;
                for i in 0..1000 {
                    if antinode.inside(y_max, x_max) {
                        if i == 1 {
                            part1_antinodes.insert(antinode);
                        }
                        part2_antinodes.insert(antinode);
                    } else {
                        break;
                    }
                    antinode = antinode + delta;
                }
            }
        }
    } 
    println!("Part 1: {}", part1_antinodes.len());
    println!("Part 2: {}", part2_antinodes.len());
}
