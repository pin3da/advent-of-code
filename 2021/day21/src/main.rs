use std::collections::HashMap;

fn main() {
    println!("{:?}", solve(2, 10));
    println!("{:?}", solve_2(2, 10));
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Player {
    pos: i32,
    score: i32,
}

impl Player {
    fn play(&mut self, a: i32, b: i32, c: i32) -> bool {
        let new_pos = (self.pos - 1 + a + b + c) % 10 + 1;
        self.pos = new_pos;
        self.score += new_pos;
        self.score >= 1000
    }

    fn with_game(&self, a: i32, b: i32, c: i32) -> Player {
        let new_pos = (self.pos - 1 + a + b + c) % 10 + 1;
        Player {
            pos: new_pos,
            score: self.score + new_pos,
        }
    }
}

struct Dice {
    side: i32,
    times: i32,
}

impl Dice {
    fn next(&mut self) -> i32 {
        let ret = self.side;
        self.times += 1;
        self.side += 1;
        if self.side > 100 {
            self.side -= 100;
        }
        ret
    }
}

fn solve(pos1: i32, pos2: i32) -> i32 {
    let mut p1 = Player {
        pos: pos1,
        score: 0,
    };
    let mut p2 = Player {
        pos: pos2,
        score: 0,
    };
    let mut dice = Dice { side: 1, times: 0 };
    loop {
        if p1.play(dice.next(), dice.next(), dice.next()) {
            break;
        }
        if p2.play(dice.next(), dice.next(), dice.next()) {
            break;
        }
    }
    let loser = std::cmp::min(p1.score, p2.score);
    loser * dice.times
}

type Memo = HashMap<(Player, Player), (i64, i64)>;

fn solve_2(pos1: i32, pos2: i32) -> i64 {
    let p1 = Player {
        pos: pos1,
        score: 0,
    };
    let p2 = Player {
        pos: pos2,
        score: 0,
    };
    let mut memo: Memo = HashMap::new();
    let times_won = visit(p1, p2, &mut memo);
    std::cmp::max(times_won.0, times_won.1)
}

fn visit(p1: Player, p2: Player, memo: &mut Memo) -> (i64, i64) {
    if let Some(seen) = memo.get(&(p1.clone(), p2.clone())) {
        return *seen;
    }
    let mut times_won = (0, 0);
    for a in 1..4 {
        for b in 1..4 {
            for c in 1..4 {
                let p1_next = p1.with_game(a, b, c);
                if p1_next.score >= 21 {
                    times_won.0 += 1;
                    continue;
                }
                for x in 1..4 {
                    for y in 1..4 {
                        for z in 1..4 {
                            let p2_next = p2.with_game(x, y, z);
                            if p2_next.score >= 21 {
                                times_won.1 += 1;
                                continue;
                            }
                            let next = visit(p1_next.clone(), p2_next, memo);
                            times_won.0 += next.0;
                            times_won.1 += next.1;
                        }
                    }
                }
            }
        }
    }
    memo.insert((p1, p2), times_won);
    times_won
}

#[test]
fn part_1() {
    assert_eq!(solve(4, 8), 739785);
}

#[test]
fn part_2() {
    assert_eq!(solve_2(4, 8), 444356092776315i64);
}
