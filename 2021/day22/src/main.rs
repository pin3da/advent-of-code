use std::cmp::max;
use std::cmp::min;

fn main() {
    // let a = Op::from_str("on x=-20..26,y=-36..17,z=-47..7");
    // let b = Op::from_str("on x=-20..33,y=-21..23,z=-26..28");
    // println!("{:?}", a.split(&b));

    println!("{:?}", solve(include_str!("example.txt")));
    // println!("{:?}", solve_2(include_str!("input.txt")));
}

#[derive(Clone, Copy, Debug)]
struct Range(i32, i32);

#[derive(Clone, Debug)]
struct Cube {
    x: Range,
    y: Range,
    z: Range,
}

#[derive(Clone, Debug)]
struct Op {
    on: bool,
    space: Cube,
}

impl Range {
    // does not cover "other contains self".
    fn intersects(&self, other: &Range) -> bool {
        (self.0 <= other.0 && self.1 >= other.0) || (self.0 <= other.1 && self.1 >= other.1)
    }
    fn contains(&self, other: Range) -> bool {
        self.0 <= other.0 && self.1 >= other.0 && self.0 <= other.1 && self.1 >= other.1
    }
    fn not_empty(&self) -> bool {
        self.0 < self.1
    }
    fn size(&self) -> i32 {
        self.1 - self.0 + 1
    }
    fn split(&self, other: &Range) -> Vec<Range> {
        assert!(self.intersects(other), "can not split disjoint ranges");
        [
            Range(min(self.0, other.0), max(self.0, other.0) - 1),
            if self.0 > other.0 {
                Range(self.0, other.1 - 1)
            } else {
                Range(other.0, min(self.1, other.1) - 1)
            },
            Range(min(self.1, other.1), max(self.1, other.1)),
        ]
        .to_vec()
    }
}

impl Cube {
    fn from_str(s: &str) -> Cube {
        let mut parts = s.split(",");
        fn parse_range(r: &str) -> Range {
            let (_, range) = r.split_at(2);
            let mut range = range.split("..");
            Range(
                range.next().unwrap().parse().unwrap(),
                range.next().unwrap().parse().unwrap(),
            )
        }
        Cube {
            x: parse_range(parts.next().unwrap()),
            y: parse_range(parts.next().unwrap()),
            z: parse_range(parts.next().unwrap()),
        }
    }
    fn intersects(&self, other: &Cube) -> bool {
        self.x.intersects(&other.x) && self.y.intersects(&other.y) && self.z.intersects(&other.z)
    }

    fn contains(&self, other: &Cube) -> bool {
        self.x.contains(other.x) && self.y.contains(other.y) && self.z.contains(other.z)
    }
    fn not_empty(&self) -> bool {
        self.x.not_empty() && self.y.not_empty() && self.z.not_empty()
    }

    fn area(&self) -> i32 {
        self.x.size() * self.y.size() * self.z.size()
    }

    fn split(&self, next: &Cube) -> Vec<Cube> {
        assert!(self.intersects(next), "can not split disjoint cuboids");
        let xs = self.x.split(&next.x);
        let ys = self.y.split(&next.y);
        let zs = self.z.split(&next.z);
        let mut ans: Vec<Cube> = Vec::new();
        for x in xs.iter() {
            for y in ys.iter() {
                for z in zs.iter() {
                    let c = Cube {
                        x: x.clone(),
                        y: y.clone(),
                        z: z.clone(),
                    };
                    if c.not_empty() {
                        ans.push(c);
                    }
                }
            }
        }
        ans
    }
}

impl Op {
    fn from_str(s: &str) -> Op {
        let mut parts = s.split_ascii_whitespace();
        Op {
            on: parts.next().unwrap() == "on",
            space: Cube::from_str(parts.next().unwrap()),
        }
    }

    fn split(&self, next: &Op) -> Vec<Op> {
        if next.space.contains(&self.space) {
            return [next.clone()].to_vec();
        }
        if self.space.intersects(&next.space) {
            let cubes = self.space.split(&next.space);
            let ops: Vec<Op> = cubes
                .iter()
                .map(|c| Op {
                    space: c.clone(),
                    on: if next.space.contains(&c) {
                        next.on
                    } else {
                        self.on
                    },
                })
                .collect();
            return ops;
        }
        [self.clone()].to_vec()
    }
}

fn split(state: &Vec<Op>, next: &Op) -> Vec<Op> {
    let mut next_state: Vec<Op> = state
        .iter()
        .flat_map(|op| op.split(next))
        .filter(|op| op.space.not_empty())
        .collect();

    if next_state.len() == state.len() {
        next_state.push(next.clone());
    }

    next_state
}

fn solve(s: &str) -> i32 {
    let bounding_box = Cube {
        x: Range(-50, 50),
        y: Range(-50, 50),
        z: Range(-50, 50),
    };
    let ops: Vec<Op> = s
        .lines()
        .map(|l| Op::from_str(l))
        .filter(|op| bounding_box.contains(&op.space))
        .collect();

    println!("Ops len {:?}", ops.len());

    let mut state: Vec<Op> = Vec::new();
    state.push(ops[0].clone());

    for i in 1..ops.len() {
        state = split(&state, &ops[i]);
        println!("State len at {}, {:?}", i, state.len());
    }

    println!("State {:?}", state);
    state
        .iter()
        .filter(|op| op.on)
        .map(|op| op.space.area())
        .sum()
}

fn solve_2(s: &str) -> i32 {
    10
}

#[test]
fn test_split() {
    let outer = Op {
        space: Cube {
            x: Range(0, 3),
            y: Range(0, 3),
            z: Range(0, 3),
        },
        on: false,
    };
    let inner = Op {
        space: Cube {
            x: Range(1, 1),
            y: Range(1, 1),
            z: Range(1, 1),
        },
        on: true,
    };

    let split = outer.split(&inner);
    assert_eq!(split.len(), 27);
    assert_eq!(split.iter().filter(|op| op.on).count(), 1);
}

#[test]
fn part_1() {
    assert_eq!(solve(include_str!("example.txt")), 20);
}

#[test]
fn part_2() {
    assert_eq!(solve_2(include_str!("example.txt")), 10);
}
