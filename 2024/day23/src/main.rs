use input_parsing::read_input;
use std::collections::BTreeSet;

fn main() {
    let input = read_input("./src/example.txt");
    let mut nodes = BTreeSet::new();
    let mut edges = BTreeSet::new();
    for line in input.lines() {
        let parts = line.split('-').map(|s| s.trim()).collect::<Vec<&str>>();
        let left = parts[0];
        let right = parts[1];
        nodes.insert(left);
        nodes.insert(right);

        edges.insert((left, right));
        edges.insert((right, left));
    }
    let nodes = nodes.into_iter().collect::<Vec<&str>>();

    println!("Part1 {}", part1(&nodes, &edges));
    println!("Part2 {}", part2(&nodes, &edges));
}

fn part2(nodes: &[&str], edges: &BTreeSet<(&str, &str)>) -> String {
    let mut max_set = BTreeSet::new();
    for &start in nodes {
        let mut cur_set = BTreeSet::new();
        cur_set.insert(start);
        for &other in nodes {
            let connected_to_all = cur_set.iter().all(|&n| edges.contains(&(n, other)));
            if connected_to_all {
                cur_set.insert(other);
            }
        }
        if cur_set.len() > max_set.len() {
            max_set = cur_set;
        }
    }

    max_set.into_iter().collect::<Vec<&str>>().join(",")
}

fn part1(nodes: &[&str], edges: &BTreeSet<(&str, &str)>) -> usize {
    let mut valid = BTreeSet::new();
    for i in 0..nodes.len() {
        if !nodes[i].starts_with('t') {
            continue;
        }
        for j in 0..nodes.len() {
            for k in j + 1..nodes.len() {
                if edges.contains(&(nodes[i], nodes[j]))
                    && edges.contains(&(nodes[j], nodes[k]))
                    && edges.contains(&(nodes[k], nodes[i]))
                {
                    let mut triplet = vec![nodes[i], nodes[j], nodes[k]];
                    triplet.sort();
                    valid.insert((triplet[0], triplet[1], triplet[2]));
                }
            }
        }
    }

    valid.len()
}
