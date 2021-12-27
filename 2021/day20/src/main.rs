use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::ops::Range;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn add(&self, dx: i32, dy: i32) -> Pos {
        Pos {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

fn main() {
    println!("{:?}", solve(include_str!("input.txt"), 2));
    println!("{:?}", solve(include_str!("input.txt"), 50));
}

fn solve(s: &str, iters: i32) -> i32 {
    let mut lines: VecDeque<&str> = VecDeque::from_iter(s.lines());
    let inst: Vec<char> = lines.pop_front().unwrap().chars().collect();

    assert!(lines.pop_front().unwrap().is_empty());

    let mut image: HashMap<Pos, char> = HashMap::new();
    for (x, row) in lines.iter().enumerate() {
        for (y, c) in row.chars().enumerate() {
            image.insert(
                Pos {
                    x: x as i32,
                    y: y as i32,
                },
                c,
            );
        }
    }

    for i in 0..iters {
        let outside = if i % 2 == 1 { inst[0] } else { '.' };
        image = transform(&image, &inst, outside);
    }

    let ans = image.iter().map(|p| p.1).filter(|p| **p == '#').count();

    ans as i32
}

fn transform(image: &HashMap<Pos, char>, inst: &Vec<char>, outside: char) -> HashMap<Pos, char> {
    let mut transformed: HashMap<Pos, char> = HashMap::new();
    for x in window_x(image) {
        for y in window_y(image) {
            let pixel_pos = Pos { x, y };
            let mut pos = 0;
            for dx in -1..2 {
                for dy in -1..2 {
                    let next = pixel_pos.add(dx, dy);
                    let bit = if *image.get(&next).unwrap_or(&outside) == '#' {
                        1
                    } else {
                        0
                    };
                    pos *= 2;
                    pos += bit;
                }
            }
            transformed.insert(pixel_pos, inst[pos]);
        }
    }

    transformed
}

fn window_x(image: &HashMap<Pos, char>) -> Range<i32> {
    let mut ans = (0, 0);
    for pos in image.keys() {
        ans.0 = std::cmp::min(ans.0, pos.x);
        ans.1 = std::cmp::max(ans.1, pos.x);
    }
    ans.0 - 2..ans.1 + 2
}
fn window_y(image: &HashMap<Pos, char>) -> Range<i32> {
    let mut ans = (0, 0);
    for pos in image.keys() {
        ans.0 = std::cmp::min(ans.0, pos.y);
        ans.1 = std::cmp::max(ans.1, pos.y);
    }
    ans.0 - 2..ans.1 + 2
}

#[test]
fn test_part_1() {
    assert_eq!(35, solve(include_str!("example.txt"), 2));
    assert_eq!(5081, solve(include_str!("input.txt"), 2));
}

#[test]
fn test_part_2() {
    assert_eq!(3351, solve(include_str!("example.txt"), 50));
}
