use input_parsing::read_input;
use std::collections::{HashMap, VecDeque};

fn main() {
    let input = read_input("./src/example.txt");

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    println!("part1: {}", solve(&grid, 2));

    // 1029884 was too high -> Was always adding +2 to the distance (and not the cheat time)
    println!("part2: {}", solve(&grid, 20));
}

fn solve(grid: &Vec<Vec<char>>, cheat_time: usize) -> usize {
    let start = find_pos(&grid, 'S');
    let end = find_pos(&grid, 'E');
    let from_start = find_shortest_path(&grid, start);
    let from_end = find_shortest_path(&grid, end);
    let original = from_start.get(&end).unwrap();

    let mut cheats: Vec<usize> = Vec::new();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] != '#' {
                for ((dx, dy), dist) in get_neighbors(x, y, cheat_time) {
                    if dx < grid.len() && dy < grid[0].len() && grid[dx][dy] != '#' {
                        if let Some(start_dist) = from_start.get(&(x, y)) {
                            if let Some(end_dist) = from_end.get(&(dx, dy)) {
                                let dist = start_dist + end_dist + dist;
                                if dist < *original {
                                    cheats.push(*original - dist);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    cheats.iter().filter(|x| **x >= 100).count()
}

fn find_shortest_path(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
) -> HashMap<(usize, usize), usize> {
    let mut dist = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    dist.insert(start, 0);

    while let Some((x, y)) = queue.pop_front() {
        let current_distance = *dist.get(&(x, y)).unwrap();
        for ((dx, dy), _) in get_neighbors(x, y, 1) {
            if dx < grid.len() && dy < grid[0].len() {
                if grid[dx][dy] != '#' && dist.get(&(dx, dy)).is_none() {
                    queue.push_back((dx, dy));
                    dist.insert((dx, dy), current_distance + 1);
                }
            }
        }
    }

    dist
}

fn get_neighbors(x: usize, y: usize, max_dist: usize) -> Vec<((usize, usize), usize)> {
    let mut neighbors = Vec::new();
    for d in 1..=max_dist {
        for dx in 0..=d {
            let dy = d - dx;
            neighbors.push(((x.wrapping_add(dx), y.wrapping_add(dy)), d));
            neighbors.push(((x.wrapping_sub(dx), y.wrapping_sub(dy)), d));
            neighbors.push(((x.wrapping_add(dx), y.wrapping_sub(dy)), d));
            neighbors.push(((x.wrapping_sub(dx), y.wrapping_add(dy)), d));
        }
    }
    neighbors.sort_unstable();
    neighbors.dedup();
    neighbors
}

fn find_pos(grid: &Vec<Vec<char>>, c: char) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == c {
                return (i, j);
            }
        }
    }
    panic!("{} not found", c);
}
