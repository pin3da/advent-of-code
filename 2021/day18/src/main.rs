use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum Snail {
    Root { val: i32 },
    Node { left: Box<Snail>, right: Box<Snail> },
}

struct ExploidResult {
    exploided: bool,
    add_left: i32,
    add_rigth: i32,
    updated: Snail,
}

fn main() {
    println!("{:?}", solve_1(include_str!("input.txt")));
    println!("{:?}", solve_2(include_str!("input.txt")));
}

fn solve_1(s: &str) -> i64 {
    let mut input = parse_input(s);
    while input.len() > 1 {
        let left = Box::new(input.pop_front().unwrap());
        let right = Box::new(input.pop_front().unwrap());
        input.push_front(reduce(Snail::Node { left, right }));
    }
    magnitude(input.pop_front().unwrap())
}

fn solve_2(s: &str) -> i64 {
    let input = parse_input(s);
    let mut best = 0;
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j {
                continue;
            }
            let add = reduce(Snail::Node {
                left: Box::new(input[i].clone()),
                right: Box::new(input[j].clone()),
            });
            best = std::cmp::max(best, magnitude(add));
        }
    }
    best
}

fn reduce(ori: Snail) -> Snail {
    let mut p = ori.clone();
    loop {
        let exp = try_explode(&p, 1);
        if exp.exploided {
            p = exp.updated;
            continue;
        }
        if splits(&mut p) {
            continue;
        }
        return p;
    }
}

fn get_expected_root(s: &Snail) -> i32 {
    match s {
        Snail::Root { val } => *val,
        _ => panic!("expected root"),
    }
}

fn try_explode(node: &Snail, level: i8) -> ExploidResult {
    match node {
        Snail::Node { left, right } => {
            if level > 4 {
                ExploidResult {
                    updated: Snail::Root { val: 0 },
                    exploided: true,
                    add_left: get_expected_root(left),
                    add_rigth: get_expected_root(right),
                }
            } else {
                let exp = try_explode(left, level + 1);
                if exp.exploided {
                    let new_right = add_leftmost_child(right, exp.add_rigth);
                    return ExploidResult {
                        updated: Snail::Node {
                            left: Box::new(exp.updated),
                            right: Box::new(new_right),
                        },
                        exploided: true,
                        add_left: exp.add_left,
                        add_rigth: 0,
                    };
                }
                let exp = try_explode(right, level + 1);
                if exp.exploided {
                    let new_left = add_rightmost_child(left, exp.add_left);
                    return ExploidResult {
                        updated: Snail::Node {
                            left: Box::new(new_left),
                            right: Box::new(exp.updated),
                        },
                        exploided: true,
                        add_left: 0,
                        add_rigth: exp.add_rigth,
                    };
                }

                ExploidResult {
                    updated: node.clone(),
                    exploided: false,
                    add_left: 0,
                    add_rigth: 0,
                }
            }
        }
        Snail::Root { val: _ } => ExploidResult {
            updated: node.clone(),
            exploided: false,
            add_left: 0,
            add_rigth: 0,
        },
    }
}

fn add_leftmost_child(node: &Snail, to_add: i32) -> Snail {
    match node {
        Snail::Node { left, right } => Snail::Node {
            left: Box::new(add_leftmost_child(left, to_add)),
            right: right.clone(),
        },
        Snail::Root { val } => Snail::Root { val: val + to_add },
    }
}

fn add_rightmost_child(node: &Snail, to_add: i32) -> Snail {
    match node {
        Snail::Node { left, right } => Snail::Node {
            left: left.clone(),
            right: Box::new(add_rightmost_child(right, to_add)),
        },
        Snail::Root { val } => Snail::Root { val: val + to_add },
    }
}

fn splits(p: &mut Snail) -> bool {
    match p {
        Snail::Node { left, right } => splits(&mut *left) || splits(&mut *right),
        Snail::Root { val } => {
            if *val >= 10 {
                *p = Snail::Node {
                    left: Box::new(Snail::Root { val: *val / 2 }),
                    right: Box::new(Snail::Root {
                        val: (*val + 1) / 2,
                    }),
                };
                true
            } else {
                false
            }
        }
    }
}

fn magnitude(p: Snail) -> i64 {
    match p {
        Snail::Node { left, right } => magnitude(*left) * 3 + magnitude(*right) * 2,
        Snail::Root { val } => val as i64,
    }
}

fn parse_input(s: &str) -> VecDeque<Snail> {
    s.lines().map(|l| parse_line(l)).collect()
}

fn parse_line(s: &str) -> Snail {
    let mut buff: VecDeque<char> = s.chars().collect();
    parse(&mut buff)
}

fn parse(s: &mut VecDeque<char>) -> Snail {
    let first = s.pop_front().unwrap();
    if first == '[' {
        let left = Box::new(parse(s));
        assert_eq!(s.pop_front().unwrap(), ',');
        let right = Box::new(parse(s));
        assert_eq!(s.pop_front().unwrap(), ']');
        return Snail::Node { left, right };
    }
    Snail::Root {
        val: first.to_digit(10).unwrap() as i32,
    }
}

#[test]
fn part_1() {
    assert_eq!(29, magnitude(parse_line("[9,1]")), "parse line");
    assert_eq!(4140, solve_1(include_str!("example.txt")), "part 1");
    assert_eq!(3993, solve_2(include_str!("example.txt")), "part 2");
}
