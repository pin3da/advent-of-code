use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    println!("part 1 {}", solve_1(include_str!("input.txt")));
    println!("part 2 {}", solve_2(include_str!("input.txt")));
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pos(i32, i32);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    pos: Pos,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid(Vec<Vec<i32>>, bool);

impl Grid {
    fn num_rows(&self) -> i32 {
        if self.1 {
            return (self.0.len() as i32) * 5;
        }
        self.0.len() as i32
    }

    fn num_cols(&self) -> i32 {
        if self.1 {
            return (self.0[0].len() as i32) * 5;
        }
        self.0[0].len() as i32
    }

    fn contains(&self, p: Pos) -> bool {
        let Pos(x, y) = p;
        if x < 0 || x >= self.num_rows() || y < 0 || y >= self.num_cols() {
            return false;
        }
        true
    }
    fn at(&self, p: Pos) -> i32 {
        if self.1 {
            let rows_ori = self.0.len() as i32;
            let cols_ori = self.0[0].len() as i32;
            let x = p.0 % rows_ori;
            let y = p.1 % cols_ori;
            let incr = (p.0 / rows_ori) + (p.1 / cols_ori);
            let val = self.0[x as usize][y as usize] + incr;
            return if val >= 10 { val - 9 } else { val };
        }
        self.0[p.0 as usize][p.1 as usize]
    }

    fn neighbors(&self, p: Pos) -> Vec<Pos> {
        let mut ans: Vec<Pos> = Vec::new();
        for delta in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next = Pos(p.0 + delta.0, p.1 + delta.1);
            if self.contains(next) {
                ans.push(next);
            }
        }
        ans
    }
}

fn solve_1(file: &str) -> i32 {
    let grid = Grid(parse_input(file), false);
    shortest_path(grid)
}

fn solve_2(file: &str) -> i32 {
    let grid = Grid(parse_input(file), true);
    shortest_path(grid)
}

fn shortest_path(grid: Grid) -> i32 {
    let mut dist: Vec<Vec<i32>> =
        vec![vec![i32::MAX; grid.num_cols() as usize]; grid.num_rows() as usize];
    let target = Pos(grid.num_rows() - 1, grid.num_cols() - 1);

    let mut states = BinaryHeap::new();
    states.push(State {
        cost: 0,
        pos: Pos(0, 0),
    });

    while let Some(State { cost, pos }) = states.pop() {
        if pos == target {
            return cost;
        }
        if cost > dist[pos.0 as usize][pos.1 as usize] {
            continue;
        }

        for to in grid.neighbors(pos) {
            let next = State {
                cost: cost + grid.at(to),
                pos: to,
            };
            if next.cost < dist[to.0 as usize][to.1 as usize] {
                states.push(next);
                dist[to.0 as usize][to.1 as usize] = next.cost;
            }
        }
    }

    -10
}

fn parse_input(s: &str) -> Vec<Vec<i32>> {
    s.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect()
}

#[test]
fn example_1() {
    assert_eq!(solve_1(include_str!("example.txt")), 40);
}

#[test]
fn example_2() {
    assert_eq!(solve_2(include_str!("example.txt")), 315);
}
