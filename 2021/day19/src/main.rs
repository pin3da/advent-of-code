use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point(i32, i32, i32);

type Matrix = [[i32; 3]; 3];
const ROT_X: Matrix = [[1, 0, 0], [0, 0, -1], [0, 1, 0]];
const ROT_Y: Matrix = [[0, 0, 1], [0, 1, 0], [-1, 0, 0]];
const ROT_Z: Matrix = [[0, -1, 0], [1, 0, 0], [0, 0, 1]];

#[derive(Debug, Clone)]
struct Scanner {
    beacons: HashSet<Point>,
}

impl Point {
    fn parse(lines: &mut VecDeque<&str>) -> Option<Point> {
        if let Some(line) = lines.pop_front() {
            if line.is_empty() {
                return None;
            }
            let nums: Vec<i32> = line.split(",").map(|s| s.parse().unwrap()).collect();
            Some(Point(nums[0], nums[1], nums[2]))
        } else {
            None
        }
    }

    fn minus(&self, o: &Point) -> Point {
        Point(self.0 - o.0, self.1 - o.1, self.2 - o.2)
    }

    fn plus(&self, o: &Point) -> Point {
        Point(self.0 + o.0, self.1 + o.1, self.2 + o.2)
    }

    fn l1_norm(&self) -> i32 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

impl Scanner {
    fn parse(lines: &mut VecDeque<&str>) -> Scanner {
        assert!(lines.len() > 0);
        let mut beacons: HashSet<Point> = HashSet::new();
        assert!(lines.pop_front().unwrap().len() > 0, "expected header");
        while let Some(p) = Point::parse(lines) {
            beacons.insert(p);
        }
        Scanner { beacons }
    }
}

fn main() {
    println!("{:?}", solve(include_str!("input.txt")));
}

fn solve(s: &str) -> (i32, i32) {
    let scanners = parse(s);
    let mut absolute_s0: HashSet<Point> = HashSet::from_iter(scanners[0].beacons.iter().cloned());
    let mut missing_match: VecDeque<Scanner> = VecDeque::from_iter(scanners);
    let mut pos_scanners: Vec<Point> = vec![Point(0, 0, 0)];
    missing_match.remove(0);
    while missing_match.len() > 0 {
        let mut found_scanner = false;
        for i in 0..missing_match.len() {
            for p0 in absolute_s0.clone().iter() {
                if let Some(pos_scanner) =
                    try_matching_at_p0(&mut absolute_s0, &missing_match[i].beacons, &p0)
                {
                    found_scanner = true;
                    pos_scanners.push(pos_scanner);
                    break;
                }
            }
            if found_scanner {
                missing_match.remove(i);
                break;
            }
        }
        assert_eq!(
            found_scanner,
            true,
            "did not find any match!. So far {:?}",
            absolute_s0.len()
        );
    }

    let mut p2 = 0;
    for s1 in pos_scanners.iter() {
        for s2 in pos_scanners.iter() {
            p2 = std::cmp::max(p2, s1.minus(s2).l1_norm());
        }
    }

    (absolute_s0.len() as i32, p2)
}

fn try_matching_at_p0(
    absolute_s0: &mut HashSet<Point>,
    beacons: &HashSet<Point>,
    abs_p0: &Point,
) -> Option<Point> {
    let rotations = vec![
        vec![],
        vec![ROT_Z],
        vec![ROT_Z, ROT_Z],
        vec![ROT_Z, ROT_Z, ROT_Z],
        vec![ROT_X],
        vec![ROT_Z, ROT_X],
        vec![ROT_Z, ROT_Z, ROT_X],
        vec![ROT_Z, ROT_Z, ROT_Z, ROT_X],
        vec![ROT_X, ROT_X],
        vec![ROT_Z, ROT_X, ROT_X],
        vec![ROT_Z, ROT_Z, ROT_X, ROT_X],
        vec![ROT_Z, ROT_Z, ROT_Z, ROT_X, ROT_X],
        vec![ROT_X, ROT_X, ROT_X],
        vec![ROT_Z, ROT_X, ROT_X, ROT_X],
        vec![ROT_Z, ROT_Z, ROT_X, ROT_X, ROT_X],
        vec![ROT_Z, ROT_Z, ROT_Z, ROT_X, ROT_X, ROT_X],
        vec![ROT_Y],
        vec![ROT_Z, ROT_Y],
        vec![ROT_Z, ROT_Z, ROT_Y],
        vec![ROT_Z, ROT_Z, ROT_Z, ROT_Y],
        vec![ROT_Y, ROT_Y, ROT_Y],
        vec![ROT_Z, ROT_Y, ROT_Y, ROT_Y],
        vec![ROT_Z, ROT_Z, ROT_Y, ROT_Y, ROT_Y],
        vec![ROT_Z, ROT_Z, ROT_Z, ROT_Y, ROT_Y, ROT_Y],
    ];
    for rot in rotations {
        if let Some(pos) = try_orientation(absolute_s0, beacons, &rot, abs_p0) {
            return Some(pos);
        }
    }
    return None;
}

fn try_orientation(
    absolute_s0: &mut HashSet<Point>,
    beacons: &HashSet<Point>,
    rot: &Vec<Matrix>,
    abs_p0: &Point,
) -> Option<Point> {
    let mut points: HashSet<Point> = beacons.clone();
    for r in rot {
        points = points.iter().map(|p| rotate(&p, &r)).collect();
    }
    return try_match(absolute_s0, &points, abs_p0);
}

fn try_match(
    absolute_s0: &mut HashSet<Point>,
    beacons: &HashSet<Point>,
    abs_p0: &Point,
) -> Option<Point> {
    for p in beacons.iter() {
        let pos_scanner = abs_p0.minus(p); // scanner sees p at position abs_p0;
        let abs_scanner = compute_absolute(beacons, &pos_scanner);
        let common: HashSet<_> = abs_scanner.intersection(&absolute_s0).collect();
        if common.len() >= 12 {
            println!("found scanner at {:?}", pos_scanner);
            absolute_s0.extend(abs_scanner);
            return Some(pos_scanner);
        }
    }
    return None;
}

fn compute_absolute(dist: &HashSet<Point>, p0: &Point) -> HashSet<Point> {
    dist.iter().map(|it| it.plus(p0)).collect()
}

fn parse(s: &str) -> Vec<Scanner> {
    let mut lines: VecDeque<&str> = s.lines().collect();
    let mut ob: Vec<Scanner> = Vec::new();
    while !lines.is_empty() {
        ob.push(Scanner::parse(&mut lines));
    }
    ob
}

fn rotate(p: &Point, rot: &Matrix) -> Point {
    Point(
        rot[0][0] * p.0 + rot[0][1] * p.1 + rot[0][2] * p.2,
        rot[1][0] * p.0 + rot[1][1] * p.1 + rot[1][2] * p.2,
        rot[2][0] * p.0 + rot[2][1] * p.1 + rot[2][2] * p.2,
    )
}

#[test]
fn test_part_1() {
    assert_eq!(79, solve(include_str!("example.txt")).0);
}

#[test]
fn test_part_2() {
    assert_eq!(3621, solve(include_str!("example.txt")).1);
}
