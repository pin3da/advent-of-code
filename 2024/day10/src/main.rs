use input_parsing::read_input;

fn main() {
    let input = read_input("./src/example.txt");
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                part1 += get_score(i, j, &grid);
                part2 += get_rating(i, j, &grid);
            }
        }
    }
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn get_neighbors(row: usize, col: usize) -> [(usize, usize); 4] {
    [
        (row.wrapping_sub(1), col),
        (row + 1, col),
        (row, col.wrapping_sub(1)),
        (row, col + 1),
    ]
}

fn get_rating(i: usize, j: usize, grid: &Vec<Vec<i32>>) -> i32 {
    fn dfs(row: usize, col: usize, grid: &Vec<Vec<i32>>) -> i32 {
        if grid[row][col] == 9 {
            return 1;
        }

        let mut paths = 0;
        let neighbors = get_neighbors(row, col);

        for (next_row, next_col) in neighbors {
            if next_row < grid.len() && next_col < grid[0].len() {
                if grid[next_row][next_col] == grid[row][col] + 1 {
                    paths += dfs(next_row, next_col, grid);
                }
            }
        }
        paths
    }

    let rating = dfs(i, j, grid);
    rating
}

fn get_score(i: usize, j: usize, grid: &Vec<Vec<i32>>) -> i32 {
    let mut score = 0;
    use std::collections::VecDeque;

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = VecDeque::new();
    queue.push_back((i, j));
    visited[i][j] = true;

    while let Some((row, col)) = queue.pop_front() {
        if grid[row][col] == 9 {
            score += 1;
            continue;
        }

        let current = grid[row][col];
        let neighbors = get_neighbors(row, col);

        for (next_row, next_col) in neighbors {
            if next_row < grid.len()
                && next_col < grid[0].len()
                && !visited[next_row][next_col]
                && grid[next_row][next_col] == current + 1
            {
                visited[next_row][next_col] = true;
                queue.push_back((next_row, next_col));
            }
        }
    }

    score
}
