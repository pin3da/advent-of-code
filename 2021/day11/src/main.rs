#[derive(Debug)]
struct Input {
    grid: Vec<Vec<i32>>,
}

impl Input {
    fn increase_level(&mut self) -> i32 {
        let mut start: Vec<(i32, i32)> = Vec::new();
        for i in 0..10 {
            for j in 0..10 {
                self.grid[i][j] += 1;
                if self.grid[i][j] == 10 {
                    start.push((i as i32, j as i32))
                }
            }
        }
        while start.len() > 0 {
            let mut next: Vec<(i32, i32)> = Vec::new();
            for p in start {
                for i in -1..2 {
                    for j in -1..2 {
                        let nx: i32 = p.0 + i;
                        let ny: i32 = p.1 + j;
                        if nx >= 0 && nx < 10 && ny >= 0 && ny < 10 {
                            if self.grid[nx as usize][ny as usize] >= 10 {
                                continue;
                            }
                            self.grid[nx as usize][ny as usize] += 1;
                            if self.grid[nx as usize][ny as usize] == 10 {
                                next.push((nx, ny));
                            }
                        }
                    }
                }
            }
            start = next;
        }
        self.count_and_reset()
    }

    fn count_and_reset(&mut self) -> i32 {
        let mut ans = 0;
        for i in 0..10 {
            for j in 0..10 {
                if self.grid[i][j] == 10 {
                    self.grid[i][j] = 0;
                    ans += 1;
                }
            }
        }
        ans
    }

    fn all_zero(&self) -> bool {
        for i in 0..10 {
            for j in 0..10 {
                if self.grid[i][j] > 0 {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let mut example = parse_input(include_str!("example.txt"));
    let mut input = parse_input(include_str!("input.txt"));
    solve(&mut example);
    solve(&mut input);
}

fn parse_input(s: &str) -> Input {
    let grid: Vec<Vec<i32>> = s
        .lines()
        .map(|it| it.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();
    Input { grid }
}

fn solve(input: &mut Input) {
    let mut ans = 0;
    for _ in 0..100 {
        ans += input.increase_level();
    }
    println!("Part 1 {}", ans);
    for i in 0..1000 {
        input.increase_level();
        if input.all_zero() {
            println!("part 2 {}", i + 101);
            return;
        }
    }
}
