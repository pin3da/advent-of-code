use std::collections::HashMap;

fn main() {
    let example = parse_input(include_str!("example.txt"));
    let input = parse_input(include_str!("input.txt"));
    solve(&example);
    solve(&input);
}

fn parse_input(s: &str) -> Vec<Vec<i32>> {
    s.lines()
        .map(|it| it.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect()
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn add(&self, delta: &Pos) -> Pos {
        Pos {
            x: self.x + delta.x,
            y: self.y + delta.y,
        }
    }
    fn access(&self, input: &Vec<Vec<i32>>) -> i32 {
        input[self.x as usize][self.y as usize]
    }
    fn is_inside(&self, input: &Vec<Vec<i32>>) -> bool {
        self.x >= 0
            && self.x < (input.len() as i32)
            && self.y >= 0
            && self.y < (input[0].len() as i32)
    }
}

const DELTA: [Pos; 4] = [
    Pos { x: 0, y: -1 },
    Pos { x: 0, y: 1 },
    Pos { x: -1, y: 0 },
    Pos { x: 1, y: 0 },
];

fn solve(input: &Vec<Vec<i32>>) {
    let mut total = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let mut is_low = true;
            let cur = Pos {
                x: i as i32,
                y: j as i32,
            };
            for k in 0..4 {
                let next = cur.add(&DELTA[k]);
                if next.is_inside(input) {
                    if next.access(input) <= input[i][j] {
                        is_low = false;
                        break;
                    }
                }
            }
            if is_low {
                total += input[i][j] + 1;
            }
        }
    }
    println!("Part 1 {}", total);

    let mut color: HashMap<Pos, i32> = HashMap::new();
    let mut cur_color = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let c = Pos {
                x: i as i32,
                y: j as i32,
            };
            if !color.contains_key(&c) && c.access(input) != 9 {
                color.insert(c, cur_color);
                paint(&c, &mut color, input);
                cur_color += 1;
            }
        }
    }

    let mut color_size: HashMap<i32, i32> = HashMap::new();
    for c in color.iter() {
        *color_size.entry(*c.1).or_insert(0) += 1;
    }
    let mut sizes: Vec<i32> = color_size.into_values().collect();
    sizes.sort();
    let mut ans = 1;
    for size in sizes.iter().rev().take(3) {
        ans *= size
    }
    println!("part 2 {}", ans);
}

fn paint(c: &Pos, color: &mut HashMap<Pos, i32>, input: &Vec<Vec<i32>>) {
    for k in 0..4 {
        let next = c.add(&DELTA[k]);
        if !color.contains_key(&next) && next.is_inside(input) && next.access(input) < 9 {
            color.insert(next, *color.get(c).unwrap());
            paint(&next, color, input);
        }
    }
}
