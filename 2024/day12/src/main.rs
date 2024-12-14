use input_parsing::read_input;

fn main() {
    let input = read_input("./src/example.txt");
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut regions = vec![vec![0; grid[0].len()]; grid.len()];
    let mut max_region_id = 0;
    let mut area_by_region = vec![0; grid.len() * grid[0].len()];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if regions[i][j] == 0 {
                max_region_id += 1;
                area_by_region[max_region_id] = dfs(i, j, &grid, &mut regions, max_region_id);
            }
        }
    }
    max_region_id += 1;

    let mut perimeter_by_region = vec![0; max_region_id];
    let mut corners_by_region = vec![0; max_region_id];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let region_id = regions[i][j];
            perimeter_by_region[region_id] += get_neighbors(i, j)
                .iter()
                .filter(|&&(r, c)| {
                    r >= grid.len() || c >= grid[r].len() || regions[r][c] != region_id
                })
                .count();
            corners_by_region[region_id] += is_corner(i, j, &regions);
        }
    }

    let part1 = (1..max_region_id)
        .map(|i| perimeter_by_region[i] * area_by_region[i])
        .sum::<usize>();
    println!("part1: {}", part1);
    let part2 = (1..max_region_id)
        .map(|i| corners_by_region[i] * area_by_region[i])
        .sum::<usize>();
    println!("part2: {}", part2);
}

fn is_corner(row: usize, col: usize, regions: &Vec<Vec<usize>>) -> usize {
    fn get_region_id(row: usize, col: usize, regions: &Vec<Vec<usize>>) -> usize {
        if row >= regions.len() || col >= regions[row].len() {
            return 10000;
        }
        regions[row][col]
    }
    let mut corners = 0;
    for i in 0..4 {
        let neighbors = vec![
            get_neighbors(row, col)[i],
            get_neighbors(row, col)[(i + 1) % 4],
        ];
        // Convex
        if neighbors
            .iter()
            .all(|&(r, c)| get_region_id(r, c, regions) != get_region_id(row, col, regions))
        {
            corners += 1;
        }

        // Concave
        let diagonal = get_diagonals(row, col)[i];
        if neighbors
            .iter()
            .all(|&(r, c)| get_region_id(r, c, regions) == get_region_id(row, col, regions))
            && get_region_id(diagonal.0, diagonal.1, regions)
                != get_region_id(row, col, regions)
        {
            corners += 1;
        }
    }
    corners
}

fn dfs(
    row: usize,
    col: usize,
    grid: &Vec<Vec<char>>,
    regions: &mut Vec<Vec<usize>>,
    region_id: usize,
) -> usize {
    if regions[row][col] != 0 {
        return 0;
    }
    regions[row][col] = region_id;
    let mut area = 1;
    for (r, c) in get_neighbors(row, col) {
        if r < grid.len() && c < grid[r].len() && grid[r][c] == grid[row][col] {
            area += dfs(r, c, grid, regions, region_id);
        }
    }
    area
}

fn get_neighbors(row: usize, col: usize) -> [(usize, usize); 4] {
    [
        (row.wrapping_sub(1), col), // up
        (row, col.wrapping_add(1)), // right
        (row.wrapping_add(1), col), // down
        (row, col.wrapping_sub(1)), // left
    ]
}

fn get_diagonals(row: usize, col: usize) -> [(usize, usize); 4] {
    [
        (row.wrapping_sub(1), col.wrapping_add(1)), // up-right
        (row.wrapping_add(1), col.wrapping_add(1)), // right-down
        (row.wrapping_add(1), col.wrapping_sub(1)), // down-left
        (row.wrapping_sub(1), col.wrapping_sub(1)), // left-up
    ]
}
