use input_parsing::read_input_sections;
use std::collections::HashMap;

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<char>>,
    x_size: usize,
    y_size: usize,
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid {{")?;
        for row in &self.grid {
            writeln!(f, "  {}", row.iter().collect::<String>())?;
        }
        write!(f, "}}")
    }
}

fn step(x: usize, y: usize, dir: char) -> (usize, usize) {
    match dir {
        '>' => (x, y.wrapping_add(1)),
        '<' => (x, y.wrapping_sub(1)),
        '^' => (x.wrapping_sub(1), y),
        'v' => (x.wrapping_add(1), y),
        _ => panic!("Invalid direction: '{}'.", dir),
    }
}

impl Grid {
    fn new(grid: &String) -> Self {
        Self {
            grid: grid.lines().map(|line| line.chars().collect()).collect(),
            x_size: grid.lines().next().unwrap().len(),
            y_size: grid.lines().count(),
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.x_size || y >= self.y_size {
            return None;
        }
        Some(self.grid[x][y])
    }

    fn move_blocks(&mut self, x: usize, y: usize, dir: char) -> bool {
        if x >= self.x_size || y >= self.y_size {
            return false;
        }
        if self.get(x, y) == Some('#') {
            return false;
        }

        let (nx, ny) = step(x, y, dir);

        if self.get(nx, ny) == Some('.') {
            self.grid[nx][ny] = self.grid[x][y];
            self.grid[x][y] = '.';
            return true;
        }

        if !self.move_blocks(nx, ny, dir) {
            return false;
        }

        assert!(self.get(nx, ny) == Some('.'));
        self.grid[nx][ny] = self.grid[x][y];
        self.grid[x][y] = '.';

        true
    }

    fn move_larger_blocks(&mut self, x: usize, y: usize, dir: char) -> bool {
        let mut state: HashMap<(usize, usize), bool> = HashMap::new();
        let mut to_move: Vec<(usize, usize)> = vec![];

        fn can_move(
            grid: &Grid,
            x: usize,
            y: usize,
            dir: char,
            state: &mut HashMap<(usize, usize), bool>,
            to_move: &mut Vec<(usize, usize)>,
        ) -> bool {
            if x >= grid.x_size || y >= grid.y_size {
                return false;
            }
            if grid.get(x, y) == Some('#') {
                return false;
            }
            if state.contains_key(&(x, y)) {
                return state[&(x, y)];
            }
            let (nx, ny) = step(x, y, dir);
            if grid.get(nx, ny) == Some('.') {
                to_move.push((x, y));
                state.insert((x, y), true);
                return true;
            }
            if grid.get(nx, ny) == Some('#') {
                return false;
            }

            let (mx, my) = match grid.get(nx, ny).unwrap() {
                '[' => (nx, ny + 1),
                ']' => (nx, ny - 1),
                _ => panic!("Char {} is not a box", grid.get(nx, ny).unwrap()),
            };

            let can_move_first = can_move(grid, nx, ny, dir, state, to_move);
            let can_move_second = can_move(grid, mx, my, dir, state, to_move);
            state.insert((x, y), can_move_first && can_move_second);
            if state[&(x, y)] {
                to_move.push((x, y));
                return true;
            }
            false
        }

        if can_move(self, x, y, dir, &mut state, &mut to_move) {
            for (cur_x, cur_y) in to_move {
                let (nx, ny) = step(cur_x, cur_y, dir);
                self.grid[nx][ny] = self.grid[cur_x][cur_y];
                self.grid[cur_x][cur_y] = '.';
            }
            return true;
        }
        false
    }
}

fn find_start(grid: &Grid) -> (usize, usize) {
    for i in 0..grid.x_size {
        for j in 0..grid.y_size {
            if grid.get(i, j) == Some('@') {
                return (i, j);
            }
        }
    }
    panic!("No start found");
}

fn gps_sum(grid: &Grid, target: char) -> usize {
    let mut sum = 0;
    for i in 0..grid.x_size {
        for j in 0..grid.y_size {
            if grid.get(i, j) == Some(target) {
                sum += i * 100 + j;
            }
        }
    }
    sum
}

fn part_1(grid: Grid, path: &str) -> usize {
    let mut grid = grid;
    let (mut rx, mut ry) = find_start(&grid);
    for line in path.lines() {
        for dir in line.chars() {
            if grid.move_blocks(rx, ry, dir) {
                (rx, ry) = step(rx, ry, dir);
            }
        }
    }
    gps_sum(&grid, 'O')
}

fn expand(grid: Grid) -> Grid {
    let mut new_grid: Vec<Vec<char>> = Vec::with_capacity(grid.x_size);
    for line in &grid.grid {
        let mut new_line = Vec::with_capacity(grid.y_size * 2);
        for c in line {
            match c {
                '#' => {
                    new_line.push('#');
                    new_line.push('#');
                }
                '.' => {
                    new_line.push('.');
                    new_line.push('.');
                }
                'O' => {
                    new_line.push('[');
                    new_line.push(']');
                }
                '@' => {
                    new_line.push('@');
                    new_line.push('.');
                }
                _ => panic!("Invalid character: '{}'.", c),
            }
        }
        new_grid.push(new_line);
    }
    Grid {
        grid: new_grid,
        x_size: grid.x_size,
        y_size: grid.y_size * 2,
    }
}

fn part_2(grid: Grid, path: &str) -> usize {
    let mut grid = expand(grid);
    let (mut rx, mut ry) = find_start(&grid);
    for line in path.lines() {
        for dir in line.chars() {
            match dir {
                '>' | '<' => {
                    if grid.move_blocks(rx, ry, dir) {
                        (rx, ry) = step(rx, ry, dir);
                    }
                }
                '^' | 'v' => {
                    if grid.move_larger_blocks(rx, ry, dir) {
                        (rx, ry) = step(rx, ry, dir);
                    }
                }
                _ => panic!("Invalid direction: '{}'.", dir),
            }
        }
    }
    gps_sum(&grid, '[')
}

fn main() {
    let input = read_input_sections("./src/example.txt");
    let [grid, path] = input.as_slice() else {
        panic!("Input must have exactly 2 sections");
    };
    let grid = Grid::new(grid);
    println!("Part 1: {}", part_1(grid.clone(), path));
    println!("Part 2: {}", part_2(grid, path));
}
