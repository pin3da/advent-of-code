use input_parsing::read_input;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn deltas(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn step(&self, direction: Direction) -> Position {
        let (dx, dy) = direction.deltas();
        Position::new(self.x + dx, self.y + dy)
    }

    fn inside(&self, rows: usize, cols: usize) -> bool {
        self.x >= 0 && self.x < rows as i32 && self.y >= 0 && self.y < cols as i32
    }
}

fn main() {
    let input = read_input("./src/example.txt");
    let grid: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut guard = Position::new(0, 0);
    let mut obstacles: HashSet<Position> = HashSet::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            if ch == '#' {
                obstacles.insert(Position::new(r as i32, c as i32));
            }
            if ch == '^' {
                guard = Position::new(r as i32, c as i32);
            }
        }
    }

    println!("Part 1: {}", solve_part_1(guard, rows, cols, &obstacles));
    println!("Part 2: {}", solve_part_2(guard, rows, cols, &obstacles));
}

fn solve_part_1(guard: Position, rows: usize, cols: usize, obstacles: &HashSet<Position>) -> usize {
    let mut guard = guard;
    let mut direction = Direction::Up;
    let mut seen: HashSet<Position> = HashSet::new();

    while guard.inside(rows, cols) {
        seen.insert(guard);
        let next_pos = guard.step(direction);
        if obstacles.contains(&next_pos) {
            direction = direction.turn_right();
        } else {
            guard = next_pos;
        }
    }

    seen.len()
}

fn solve_part_2(guard: Position, rows: usize, cols: usize, obstacles: &HashSet<Position>) -> usize {

    let mut handles = vec![];
    for r in 0..rows {
        for c in 0..cols {
            let pos = Position::new(r as i32, c as i32);
            if obstacles.contains(&pos) {
                continue;
            }
            let obstacles = obstacles.clone();
            let guard = guard;
            let handle = std::thread::spawn(move || {
                let mut new_obstacles = obstacles;
                new_obstacles.insert(pos);
                find_cycle(guard, rows, cols, &new_obstacles)
            });
            handles.push(handle);
        }
    }

    let mut num_cycles = 0;
    for handle in handles {
        if handle.join().unwrap() {
            num_cycles += 1;
        }
    }

    num_cycles
}

fn find_cycle(guard: Position, rows: usize, cols: usize, obstacles: &HashSet<Position>) -> bool {
    #[derive(Hash, Eq, PartialEq, Clone, Copy)]
    struct State {
        pos: Position,
        dir: Direction,
    }
    let mut state = State {
        pos: guard,
        dir: Direction::Up,
    };
    let mut seen: HashSet<State> = HashSet::new();
    while state.pos.inside(rows, cols) {
        if seen.contains(&state) {
            return true;
        }
        seen.insert(state);
        let next_pos = state.pos.step(state.dir);
        if obstacles.contains(&next_pos) {
            state.dir = state.dir.turn_right();
        } else {
            state.pos = next_pos;
        }
    }
    false
}
