use input_parsing::read_input;
use std::collections::{HashMap, HashSet};

const MOD: i64 = 16777216;

fn main() {
    let input = read_input("./src/example.txt")
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let part1: i64 = input.iter()
        .map(|&seed| *generate_secrets(seed, 2000).last().unwrap())
        .sum();
    println!("Part 1: {}", part1);


    let start_time = std::time::Instant::now();

    let mut total_by_seq: HashMap<Vec<i64>, i64> = HashMap::new();
    for seed in &input {
        let secret = generate_secrets(*seed, 2000).iter().map(|v| v % 10).collect::<Vec<i64>>();
        let windows = secret.windows(5);
        let mut seen_seq = HashSet::new();
        for w in windows {
            let diff = w.windows(2).map(|d| d[1] - d[0]).collect::<Vec<i64>>();
            if seen_seq.contains(&diff) {
                continue;
            }
            seen_seq.insert(diff.clone());
            total_by_seq.entry(diff).and_modify(|v| *v += w[4]).or_insert(w[4]);
        }
    }

    let part2 = total_by_seq.values().max().unwrap_or(&0);

    // 1608 too low -> Introduced a bug in the generation of the sequence.
    println!("Part 2: {}", part2);
    println!("Took: {:?}", start_time.elapsed());
}

fn generate_secrets(seed: i64, length: usize) -> Vec<i64> {
    std::iter::successors(Some(seed), |&secret| Some(next_secret_number(secret)))
        .skip(1)
        .take(length)
        .collect()
}

fn next_secret_number(secret: i64) -> i64 {
    let mut secret = secret;
    secret = (secret ^ (secret << 6)) % MOD;
    secret = (secret ^ (secret >> 5)) % MOD;
    secret = (secret ^ (secret << 11)) % MOD;
    secret
}
