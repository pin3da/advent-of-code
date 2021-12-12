use std::collections::BTreeSet;
use std::collections::HashSet;

struct Edge<'a>(&'a str, &'a str);
type Path<'a> = Vec<&'a str>;

fn main() {
    let edges = parse_input(include_str!("input.txt"));
    let mut paths: BTreeSet<Path> = BTreeSet::new();
    let mut cur: Path = Vec::new();
    cur.push("start");
    gen_paths(&edges, &mut paths, &mut cur, "none-rep");
    println!("part 1 {:?}", paths.len());
    let small: HashSet<&str> = get_small(&edges);
    for s in small {
        println!("can repeat {:?}", s);
        gen_paths(&edges, &mut paths, &mut cur, s);
    }
    println!("part 2 {:?}", paths.len());
}

fn parse_input(s: &str) -> Vec<Edge> {
    let mut edges: Vec<Edge> = Vec::new();
    for l in s.lines() {
        let a: Vec<&str> = l.split("-").collect();
        edges.push(Edge(a[0], a[1]));
        edges.push(Edge(a[1], a[0]));
    }
    edges
}

fn gen_paths<'a>(
    edges: &Vec<Edge<'a>>,
    paths: &mut BTreeSet<Path<'a>>,
    cur: &mut Path<'a>,
    repeat: &str,
) {
    let node: &str = cur.last().unwrap();
    if node == "end" {
        paths.insert(cur.clone());
        return;
    }
    for Edge(from, to) in edges {
        if *from != node {
            continue;
        }
        let seen_times = cur.iter().filter(|i| *i == to).count();
        if is_small(to) && (seen_times > 1 || (seen_times == 1 && *to != repeat)) {
            continue;
        }
        cur.push(*to);
        gen_paths(edges, paths, cur, repeat);
        cur.pop();
    }
}

fn is_small(s: &str) -> bool {
    s != s.to_uppercase()
}

fn get_small<'a>(edges: &Vec<Edge<'a>>) -> HashSet<&'a str> {
    let mut small = HashSet::new();
    for Edge(from, to) in edges {
        if is_small(from) {
            small.insert(*from);
        }
        if is_small(to) {
            small.insert(*to);
        }
    }
    small.remove("start");
    small.remove("end");
    small
}
