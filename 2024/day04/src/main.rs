use std::collections::HashSet;

fn count_matches(grid: &Vec<Vec<u8>>, target: &Vec<u8>) -> usize {
    let target_len = target.len();
    let mut matches = 0;

    for row in grid {
        for window in row.windows(target_len) {
            if window == target {
                matches += 1;
            }
        }
    }

    for col in 0..grid[0].len() {
        for row in 0..=grid.len()-target_len {
            let mut matches_target = true;
            for i in 0..target_len {
                if grid[row+i][col] != target[i] {
                    matches_target = false;
                    break;
                }
            }
            if matches_target {
                matches += 1;
            }
        }
    }

    // top-left to bottom-right
    for row in 0..=grid.len()-target_len {
        for col in 0..=grid[0].len()-target_len {
            let mut matches_target = true;
            for i in 0..target_len {
                if grid[row+i][col+i] != target[i] {
                    matches_target = false;
                    break;
                }
            }
            if matches_target {
                matches += 1;
            }
        }
    }

    // top-right to bottom-left
    for row in 0..=grid.len()-target_len {
        for col in (target_len-1)..grid[0].len() {
            let mut matches_target = true;
            for i in 0..target_len {
                if grid[row+i][col-i] != target[i] {
                    matches_target = false;
                    break;
                }
            }
            if matches_target {
                matches += 1;
            }
        }
    }

    matches
}

fn count_x_matches(grid: &Vec<Vec<u8>>, target: &Vec<u8>) -> usize {
    let mut matches = 0;
    let len = target.len();
    let half = len / 2;

    // Count X if 2 diagonals have the same center. 
    let mut centers : HashSet<(usize, usize)> = HashSet::new();

    // top-left to bottom-right
    for row in 0..=grid.len()-len {
        for col in 0..=grid[0].len()-len {
            let mut matches_target = true;
            let mut matches_reverse = true;
            for i in 0..len {
                if grid[row+i][col+i] != target[i] {
                    matches_target = false;
                }
                if grid[row+i][col+i] != target[len-i-1] {
                    matches_reverse = false;
                }
            }
            if matches_target || matches_reverse {
                let center = (row + half, col + half);
                centers.insert(center);
            }
        }
    }

    // top-right to bottom-left
    for row in 0..=grid.len()-len {
        for col in (len-1)..grid[0].len() {
            let mut matches_target = true;
            let mut matches_reverse = true;
            for i in 0..len {
                if grid[row+i][col-i] != target[i] {
                    matches_target = false;
                }
                if grid[row+i][col-i] != target[len-i-1] {
                    matches_reverse = false;
                }
            }
            if matches_target || matches_reverse {
                let center = (row + half, col - half);
                if centers.contains(&center) {
                    matches += 1;
                }
            }
        }
    }

    matches
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let default_path = String::from("./src/example.txt");
    let input_file = args.get(1).unwrap_or(&default_path);

    let input_string = std::fs::read_to_string(input_file).unwrap();

    let grid: Vec<Vec<u8>> = input_string
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let mut target : Vec<u8> = b"XMAS".to_vec();

    let mut matches = count_matches(&grid, &target);
    target.reverse();
    matches += count_matches(&grid, &target);
    println!("Part 1: {}", matches);

    let target : Vec<u8> = b"MAS".to_vec();
    println!("Part 2: {}", count_x_matches(&grid, &target));
}
