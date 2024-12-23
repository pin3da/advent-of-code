use input_parsing::read_input;
use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};

type Grid = Vec<Vec<char>>;

lazy_static! {
    static ref NUM_PAD: Grid = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['@', '0', 'A'],
    ];
    static ref DIR_PAD: Grid = vec![vec!['@', '^', 'A'], vec!['<', 'v', '>'],];
}

fn get_neighbors(x: usize, y: usize) -> Vec<(usize, usize, char)> {
    vec![
        (x.wrapping_sub(1), y, '^'),
        (x.wrapping_add(1), y, 'v'),
        (x, y.wrapping_sub(1), '<'),
        (x, y.wrapping_add(1), '>'),
    ]
}

fn find_pos(c: char, keypad: &Grid) -> (usize, usize) {
    for (i, row) in keypad.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == c {
                return (i, j);
            }
        }
    }
    panic!("Character not found in keypad {}\n{:?}", c, keypad);
}

fn all_paths(start: (usize, usize), end: (usize, usize), keypad: &Grid) -> Vec<Vec<char>> {
    if start == end {
        return vec![vec!['A']];
    }

    let mut paths = vec![];
    let mut q: VecDeque<(usize, usize, Vec<char>)> = VecDeque::new();
    q.push_back((start.0, start.1, vec![]));
    let mut best_length = usize::MAX;

    while let Some((x, y, path)) = q.pop_front() {
        let mut can_improve = true;
        for (next_x, next_y, dir) in get_neighbors(x, y) {
            if next_x < keypad.len() && next_y < keypad[0].len() && keypad[next_x][next_y] != '@' {
                if next_x == end.0 && next_y == end.1 {
                    if best_length < path.len() + 1 {
                        can_improve = false;
                        break;
                    }
                    best_length = path.len() + 1;
                    let mut new_path = path.clone();
                    new_path.push(dir);
                    new_path.push('A');
                    paths.push(new_path);
                } else {
                    let mut new_path = path.clone();
                    new_path.push(dir);
                    q.push_back((next_x, next_y, new_path));
                }
            }
        }
        if !can_improve {
            break;
        }
    }

    paths
}

fn get_all_shortest_paths(code: &str, keypad: &Grid) -> Vec<Vec<char>> {
    let mut cur = 'A';
    let mut paths = vec![];
    for c in code.chars() {
        let from = find_pos(cur, keypad);
        let to = find_pos(c, keypad);
        let all_next_step = all_paths(from, to, keypad);
        if paths.is_empty() {
            paths = all_next_step;
        } else {
            let mut new_paths = vec![];
            for existing_path in &paths {
                for next_path in &all_next_step {
                    let mut combined = existing_path.clone();
                    combined.extend(next_path);
                    new_paths.push(combined);
                }
            }
            paths = new_paths;
        }
        cur = c;
    }
    paths
}

fn dir_len(from: (usize, usize), to: (usize, usize)) -> usize {
    let paths = all_paths(from, to, &DIR_PAD);
    assert!(paths.iter().all(|path| path.len() == paths[0].len()), "All paths must be the same length");
    paths[0].len()
}

fn solve_dirs(code: &Vec<char>, layers: usize) -> usize {
    let mut memo: HashMap<(Vec<char>, usize), usize> = HashMap::new();

    fn internal(
        code: &Vec<char>,
        layers: usize,
        memo: &mut HashMap<(Vec<char>, usize), usize>,
    ) -> usize {
        let key = (code.clone(), layers);
        if let Some(&ans) = memo.get(&key) {
            return ans;
        }

        let mut ans = 0;
        let mut cur = 'A';
        if layers == 1 {
            for c in code.iter() {
                ans += dir_len(find_pos(cur, &DIR_PAD), find_pos(*c, &DIR_PAD));
                cur = *c;
            }
            memo.insert(key, ans);
            return ans;
        }
        for &c in code.iter() {
            let min_len = all_paths(find_pos(cur, &DIR_PAD), find_pos(c, &DIR_PAD), &DIR_PAD)
                .iter()
                .map(|path| internal(&path, layers - 1, memo))
                .min()
                .unwrap();
            ans += min_len;
            cur = c;
        }
        memo.insert(key, ans);
        ans
    }

    internal(code, layers, &mut memo)
}

fn main() {
    let input = read_input("src/example.txt");

    let mut ans = 0;
    for code in input.lines() {
        let paths = get_all_shortest_paths(code, &NUM_PAD);
        let min_len = paths.iter().map(|path| solve_dirs(path, 25)).min().unwrap();
        ans += min_len * code[..code.len() - 1].parse::<usize>().unwrap();
    }
    println!("{}", ans);
}
