use input_parsing::read_input;
use std::collections::{HashMap, HashSet};

const MOD: i64 = 16777216;

fn gen_sequence() -> Vec<Vec<i64>> {
    fn int_to_vec(n: i64) -> Vec<i64> {
        let mut digits = vec![];
        let mut n = n;
        while n > 0 {
            digits.push((n % 10) as i64);
            n /= 10;
        }
        digits.reverse();
        while digits.len() < 5 {
            digits.insert(0, 0);
        }
        digits
    }

    let mut sequence = vec![];
    for i in 0..=99999 {
        sequence.push(int_to_vec(i));
    }
    sequence
}

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

    let mut best_per_seed: Vec<HashMap<Vec<i64>, i64>> = vec![];
    for seed in &input {
        let mut value_by_diff = HashMap::new();
        let secret = generate_secrets(*seed, 2000).iter().map(|v| v % 10).collect::<Vec<i64>>();
        let windows = secret.windows(5);
        for w in windows {
            let diff = w.windows(2).map(|d| d[1] - d[0]).collect::<Vec<i64>>();
            if value_by_diff.contains_key(&diff) {
                continue;
            }
            value_by_diff.insert(diff, w[4]);
        }
        best_per_seed.push(value_by_diff);
    }

    let sequence: Vec<Vec<i64>> = gen_sequence()
        .iter()
        .map(|v| v.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i64>>())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let part2 = sequence.iter()
        .map(|s| {
            best_per_seed.iter()
                .filter_map(|best| best.get(s))
                .sum::<i64>()
        })
        .max()
        .unwrap_or(0);

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
