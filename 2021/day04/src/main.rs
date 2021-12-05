use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::Peekable;

#[derive(Debug, Clone)]
struct Cell {
    row: usize,
    col: usize,
    is_marked: bool,
}

#[derive(Debug, Clone)]
struct Board {
    positions: HashMap<i32, Cell>,
    freq_col: [i32; 5],
    freq_row: [i32; 5],
    id: i32,
}

impl Board {
    fn new_from_buffer(lines: &mut Peekable<std::str::Lines>, id: i32) -> Board {
        assert!(lines.next().expect("empty") == "");
        let mut pos = HashMap::new();
        for i in 0..5 {
            let row = vec_from_buffer(lines, " ");
            for j in 0..5 {
                pos.insert(
                    row[j],
                    Cell {
                        row: i,
                        col: j,
                        is_marked: false,
                    },
                );
            }
        }
        Board {
            positions: pos,
            freq_col: [0; 5],
            freq_row: [0; 5],
            id,
        }
    }

    fn try_mark(&mut self, number: &i32) -> bool {
        let cell = self.positions.get_mut(number);
        if cell.is_none() {
            return false;
        }
        let cell = cell.unwrap();
        self.freq_col[cell.col] += 1;
        self.freq_row[cell.row] += 1;
        cell.is_marked = true;
        if self.freq_col[cell.col] == 5 || self.freq_row[cell.row] == 5 {
            return true;
        }
        return false;
    }

    fn get_score(&self, last_call: &i32) -> i32 {
        let unmarked: i32 = self
            .positions
            .iter()
            .filter(|entry| !entry.1.is_marked)
            .map(|entry| entry.0)
            .sum();
        last_call * unmarked
    }
}

#[derive(Debug, Clone)]
struct Game {
    instructions: Vec<i32>,
    boards: Vec<Board>,
}

fn main() {
    let input = parse_input(include_str!("input.txt"));
    let example = parse_input(include_str!("example.txt"));
    solve(&example);
    solve(&input);
}

fn parse_input(input: &str) -> Game {
    let mut lines = input.lines().peekable();
    let inst = vec_from_buffer(&mut lines, ",");
    let mut boards: Vec<Board> = Vec::new();
    while lines.peek().is_some() {
        boards.push(Board::new_from_buffer(&mut lines, boards.len() as i32));
    }
    Game {
        instructions: inst,
        boards: boards,
    }
}

fn vec_from_buffer(lines: &mut Peekable<std::str::Lines>, sep: &str) -> Vec<i32> {
    lines
        .next()
        .expect("input")
        .split(sep)
        .filter(|it| !it.is_empty())
        .map(|it| it.parse().unwrap())
        .collect()
}

fn solve(ori_game: &Game) {
    println!("New game");
    let mut game = ori_game.clone();
    let mut already_won: HashSet<i32> = HashSet::new();
    for curr_number in game.instructions.iter() {
        for board in game.boards.iter_mut() {
            if already_won.contains(&board.id) {
                continue;
            }
            if board.try_mark(curr_number) {
                println!(
                    "Board {} won with score {:?}",
                    board.id,
                    board.get_score(curr_number)
                );
                already_won.insert(board.id);
            }
        }
    }
}
