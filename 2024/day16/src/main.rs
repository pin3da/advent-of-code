use input_parsing::read_input;
use std::collections::{BinaryHeap, HashMap, HashSet};

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

fn step(x: usize, y: usize, dir: i32) -> (usize, usize) {
    match dir {
        0 => (x.wrapping_sub(1), y), // north
        1 => (x, y.wrapping_add(1)), // east
        2 => (x.wrapping_add(1), y), // south
        3 => (x, y.wrapping_sub(1)), // west
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
}

fn find_pos(grid: &Grid, target: char) -> (usize, usize) {
    for i in 0..grid.x_size {
        for j in 0..grid.y_size {
            if grid.get(i, j) == Some(target) {
                return (i, j);
            }
        }
    }
    panic!("No start found");
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Node {
    pos: (usize, usize),
    dir: i32,
}

fn shortest_path(grid: &Grid, start: Node) -> HashMap<Node, usize> {
    use std::cmp::Ordering;

    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    struct State {
        cost: usize,
        node: Node,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut heap = BinaryHeap::new();
    let mut costs = HashMap::new();

    // start facing east
    heap.push(State {
        cost: 0,
        node: start,
    });
    costs.insert(start, 0);

    while let Some(State { cost, node }) = heap.pop() {
        if cost > *costs.get(&node).unwrap_or(&usize::MAX) {
            continue;
        }

        // Try moving forward
        let (next_x, next_y) = step(node.pos.0, node.pos.1, node.dir);
        if let Some(c) = grid.get(next_x, next_y) {
            if c != '#' {
                let next = Node {
                    pos: (next_x, next_y),
                    dir: node.dir,
                };
                let next_cost = cost + 1;

                if next_cost < *costs.get(&next).unwrap_or(&usize::MAX) {
                    heap.push(State {
                        cost: next_cost,
                        node: next,
                    });
                    costs.insert(next, next_cost);
                }
            }
        }

        // Try turning left and right
        for new_dir in [(node.dir + 1) % 4, (node.dir + 3) % 4] {
            let next = Node {
                pos: node.pos,
                dir: new_dir,
            };
            let next_cost = cost + 1000;

            if next_cost < *costs.get(&next).unwrap_or(&usize::MAX) {
                heap.push(State {
                    cost: next_cost,
                    node: next,
                });
                costs.insert(next, next_cost);
            }
        }
    }

    costs
}

fn part2(grid: &Grid, start: Node, end: (usize, usize), shortest_cost: usize) -> usize {
    let forward = shortest_path(grid, start);
    let mut nodes_in_shortest_path = HashSet::new();
    for dir in 0..4 {
        let backwards = shortest_path(grid, Node { pos: end, dir });

        for i in 0..grid.x_size {
            for j in 0..grid.y_size {
                for d in 0..4 {
                    if let Some(up_to_here) = forward.get(&Node {
                        pos: (i, j),
                        dir: d,
                    }) {
                        if let Some(back_to_start) = backwards.get(&Node {
                            pos: (i, j),
                            dir: (d + 2) % 4,
                        }) {
                            if up_to_here + back_to_start == shortest_cost {
                                nodes_in_shortest_path.insert((i, j));
                            }
                        }
                    }
                }
            }
        }
    }

    nodes_in_shortest_path.len()
}

fn main() {
    let input = read_input("./src/example.txt");
    let grid = Grid::new(&input);
    let start = find_pos(&grid, 'S');
    let end = find_pos(&grid, 'E');
    let costs = shortest_path(&grid, Node { pos: start, dir: 1 });
    let mut part1 = usize::MAX;
    for dir in 0..4 {
        let cost = costs.get(&Node { pos: end, dir }).unwrap();
        part1 = part1.min(*cost);
    }
    println!("part 1: {}", part1);
    println!(
        "part 2: {}",
        part2(&grid, Node { pos: start, dir: 1 }, end, part1)
    );
}
