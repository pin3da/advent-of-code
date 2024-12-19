use input_parsing::read_input;

fn main() {
    let input = read_input("./src/example.txt");
    let falling: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            (parts[1].parse().unwrap(), parts[0].parse().unwrap())
        })
        .collect();

    let grid_size = (71, 71);
    let simulated_time = 1024;

    let mut grid = vec![vec![false; grid_size.1]; grid_size.0];
    for i in 0..simulated_time {
        grid[falling[i].0][falling[i].1] = true;
    }

    let shortest_path = find_shortest_path(&grid, grid_size);
    println!("Part 1: {:?}", shortest_path);

    // This function is monotonic so we could do binary search,
    // but it is not needed for the input size.
    for i in simulated_time..(grid_size.0 * grid_size.1) {
        grid[falling[i].0][falling[i].1] = true;
        if find_shortest_path(&grid, grid_size).is_none() {
            println!("Part 2: {},{}", falling[i].1, falling[i].0);
            break;
        }
    }
}

fn find_shortest_path(grid: &Vec<Vec<bool>>, grid_size: (usize, usize)) -> Option<usize> {
    use std::collections::VecDeque;

    let target = (grid_size.0 - 1, grid_size.1 - 1);

    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; grid_size.1]; grid_size.0];
    let mut distances = vec![vec![usize::MAX; grid_size.1]; grid_size.0];

    queue.push_back((0, 0));
    distances[0][0] = 0;
    visited[0][0] = true;

    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == (target.0, target.1) {
            break;
        }

        for (new_x, new_y) in neighbors(x, y) {
            if new_x < grid_size.0 && new_y < grid_size.1 {
                if !grid[new_x][new_y] && !visited[new_x][new_y] {
                    visited[new_x][new_y] = true;
                    distances[new_x][new_y] = distances[x][y] + 1;
                    queue.push_back((new_x, new_y));
                }
            }
        }
    }

    let shortest_path = if distances[target.0][target.1] == usize::MAX {
        None
    } else {
        Some(distances[target.0][target.1])
    };

    shortest_path
}

fn neighbors(x: usize, y: usize) -> [(usize, usize); 4] {
    [
        (x + 1, y),
        (x, y + 1),
        (x.wrapping_sub(1), y),
        (x, y.wrapping_sub(1)),
    ]
}
