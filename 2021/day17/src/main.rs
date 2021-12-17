fn main() {
    println!("{:?}", solve("150 193 -136 -86"));
}

#[derive(Eq, PartialEq, Debug)]
struct Target {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

#[derive(Debug)]
struct Vel {
    x: i32,
    y: i32,
}
type Pos = Vel;

impl Vel {
    fn next(&self) -> Vel {
        Vel {
            x: if self.x == 0 {
                0
            } else {
                self.x - (self.x / self.x.abs())
            },
            y: self.y - 1,
        }
    }
}

impl Pos {
    fn add(&self, v: &Vel) -> Pos {
        Pos {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

impl Target {
    fn from_str(s: &str) -> Target {
        let t: Vec<i32> = s.split_whitespace().map(|i| i.parse().unwrap()).collect();
        Target {
            x_min: t[0],
            x_max: t[1],
            y_min: t[2],
            y_max: t[3],
        }
    }

    fn contains(&self, p: &Pos) -> bool {
        self.x_min <= p.x && self.x_max >= p.x && self.y_min <= p.y && self.y_max >= p.y
    }
}

fn solve(s: &str) -> (i32, i32) {
    let target: Target = Target::from_str(s);
    let mut max_y = 0;
    let mut total = 0;
    for x in 1..200 {
        for y in -200..200 {
            let cur_y = max_high(&target, Vel { x, y });
            max_y = std::cmp::max(max_y, cur_y);
            if cur_y > -200 {
                total += 1
            }
        }
    }
    (max_y, total)
}

fn max_high(target: &Target, _vel: Vel) -> i32 {
    let mut pos = Pos { x: 0, y: 0 };
    let mut vel = _vel;
    let mut max_y = -200;
    while pos.x <= target.x_max && pos.y >= target.y_min {
        max_y = std::cmp::max(max_y, pos.y);
        if target.contains(&pos) {
            return max_y;
        }
        pos = pos.add(&vel);
        vel = vel.next();
    }
    -200
}

#[test]
fn test_part_1() {
    assert!(Target::from_str("20 30 -10 -5").contains(&Pos { x: 28, y: -7 }));
    assert_eq!(solve("20 30 -10 -5").0, 45);
    assert_eq!(solve("20 30 -10 -5").1, 112)
}
