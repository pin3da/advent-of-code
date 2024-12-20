use input_parsing::read_input_sections;
use std::collections::HashMap;

fn main() {
    let input = read_input_sections("./src/example.txt");
    let pieces = input
        .get(0)
        .unwrap()
        .split(",")
        .map(|s| s.trim())
        .collect::<Vec<&str>>();
    let targets = input.get(1).unwrap().split("\n").collect::<Vec<&str>>();

    println!("{:?}", pieces.len());

    let part1 = targets
        .iter()
        .filter(|target| can_build_target(&pieces, target) > 0)
        .count();
    println!("part1: {}", part1);

    let part2 = targets
        .iter()
        .map(|target| can_build_target(&pieces, target))
        .sum::<i64>();

    println!("part2: {}", part2);
}

fn can_build_target(pieces: &Vec<&str>, target: &str) -> i64 {
    let mut memo: HashMap<usize, i64> = HashMap::new();
    
    fn internal(pieces: &Vec<&str>, target: &str, index: usize, memo: &mut HashMap<usize, i64>) -> i64 {
        if let Some(&result) = memo.get(&index) {
            return result;
        }
        
        if index >= target.len() {
            return if index == target.len() { 1 } else { 0 };
        }

        let mut result = 0;
        for piece in pieces {
            if target[index..].starts_with(piece) {
                result += internal(pieces, target, index + piece.len(), memo);
            }
        }
        
        memo.insert(index, result);
        result
    }

    internal(pieces, target, 0, &mut memo)
}
