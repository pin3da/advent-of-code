use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Pos(i32, i32);

type Grid<'a> = HashSet<Pos>;

#[derive(Debug)]
struct Inst(char, i32);

fn main() {
    let input = parse_input(include_str!("input.txt"));
    let mut grid = input.0;
    for Inst(op, val) in input.1 {
        grid = grid
            .iter()
            .map(|p| -> Pos {
                if op == 'y' {
                    fold_up(p, val)
                } else {
                    fold_left(p, val)
                }
            })
            .collect();
        println!("Part 1 {}", grid.len());
    }
    for i in 0..10 {
        for j in 0..50 {
            if grid.contains(&Pos(j, i)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn fold_up(p: &Pos, y: i32) -> Pos {
    if p.1 > y {
        return Pos(p.0, 2 * y - p.1);
    }
    *p
}

fn fold_left(p: &Pos, x: i32) -> Pos {
    if p.0 > x {
        return Pos(2 * x - p.0, p.1);
    }
    *p
}

fn parse_input(s: &str) -> (Grid, Vec<Inst>) {
    let mut i = 0;
    let lines: Vec<&str> = s.lines().collect();
    let mut grid = HashSet::new();
    while i < lines.len() {
        if lines[i] == "" {
            break;
        }
        let p: Vec<i32> = lines[i].split(",").map(|it| it.parse().unwrap()).collect();
        grid.insert(Pos(p[0], p[1]));
        i += 1;
    }
    i += 1;

    let mut instuctions: Vec<Inst> = Vec::new();
    while i < lines.len() {
        let mut inst = lines[i].split("=");
        let s: &str = inst.next().unwrap();
        let v: i32 = inst.next().unwrap().parse().unwrap();
        instuctions.push(Inst(s.chars().last().unwrap(), v));
        i += 1;
    }
    (grid, instuctions)
}
