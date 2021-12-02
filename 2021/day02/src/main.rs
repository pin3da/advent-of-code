use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
    aim: i32,
}

const DELTA: [Pos; 3] = [
    Pos { x: 0, y: 1, aim: 0 },
    Pos { x: 1, y: 0, aim: 0 },
    Pos {
        x: 0,
        y: -1,
        aim: 0,
    },
];

impl Pos {
    fn apply_part1(&self, inst: &Instruction) -> Pos {
        Pos {
            x: self.x + DELTA[inst.inst_id].x * inst.delta,
            y: self.y + DELTA[inst.inst_id].y * inst.delta,
            aim: 0,
        }
    }
    fn apply_part2(&self, inst: &Instruction) -> Pos {
        match inst.inst_id {
            0 => Pos {
                aim: self.aim - inst.delta,
                ..*self
            },
            2 => Pos {
                aim: self.aim + inst.delta,
                ..*self
            },
            _ => Pos {
                x: self.x + inst.delta,
                y: self.y - inst.delta * self.aim,
                aim: self.aim,
            },
        }
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    inst_id: usize,
    delta: i32,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();
        let inst_id = match split[0] {
            "up" => 0,
            "forward" => 1,
            _ => 2,
        };
        let delta: i32 = split[1].parse()?;
        return Ok(Instruction { inst_id, delta });
    }
}

fn main() {
    let input = parse_input(include_str!("input.txt"));
    let example = parse_input(include_str!("example.txt"));
    println!("part 1");
    part1(&input);
    part1(&example);
    println!("part 2");
    part2(&input);
    part2(&example);
}

fn parse_input(input: &str) -> Vec<Instruction> {
    return input.lines().map(|it| it.parse().unwrap()).collect();
}

fn part1(input: &Vec<Instruction>) {
    let mut pos = Pos { x: 0, y: 0, aim: 0 };
    for inst in input {
        pos = pos.apply_part1(inst);
    }
    println!("{:?}: {}", pos, pos.x * -pos.y);
}

fn part2(input: &Vec<Instruction>) {
    let mut pos = Pos { x: 0, y: 0, aim: 0 };
    for inst in input {
        pos = pos.apply_part2(inst);
    }
    println!("{:?}: {}", pos, pos.x * -pos.y);
}
