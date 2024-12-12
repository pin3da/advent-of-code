use input_parsing::read_input;
use std::collections::HashMap;

fn main() {
    let mut freq: HashMap<i64, i64> = HashMap::new();
    read_input("./src/example.txt")
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .for_each(|stone| {
            *freq.entry(stone).or_insert(0) += 1;
        });

    println!("{:?}", freq);

    for i in 0..75 {
        let mut next_freq = HashMap::new();
        for (stone, count) in freq.iter() {
            if *stone == 0 {
                *next_freq.entry(1).or_insert(0) += count;
            } else {
                let digits = stone.to_string();
                if digits.len() & 1 == 0 {
                    let left = digits[0..digits.len() / 2].parse::<i64>().unwrap(); 
                    let right = digits[digits.len() / 2..].parse::<i64>().unwrap();
                    *next_freq.entry(left).or_insert(0) += count;
                    *next_freq.entry(right).or_insert(0) += count;
                } else {
                    *next_freq.entry(*stone * 2024).or_insert(0) += count;
                }
            }
        }
        freq = next_freq;
        if i == 24 {
            println!("part1: {}", freq.values().sum::<i64>());
        }
    }
    println!("part2: {}", freq.values().sum::<i64>());
}

