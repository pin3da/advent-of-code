fn main() {
    let input = parse_input(include_str!("input.txt"));
    let example = parse_input(include_str!("example.txt"));
    println!("part 1");
    part1(&example);
    part1(&input);
    println!("part 2");
    part2(&example);
    part2(&input);
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    return input
        .lines()
        .map(|it| it.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
}

fn part1(input: &Vec<Vec<u32>>) {
    let freq = get_freq(input);
    let mut gamma = 0;
    let half = (input.len() / 2) as u32;
    for val in freq.iter() {
        gamma *= 2;
        gamma += if *val > half { 1 } else { 0 };
    }
    let epsilon = ((1 << freq.len()) - 1) ^ gamma;
    println!("{:?}", epsilon * gamma);
}

fn part2(input: &Vec<Vec<u32>>) {
    let oxygen = generate_rating(input, |freq, half| if freq >= half { 1 } else { 0 });
    let scrubber = generate_rating(input, |freq, half| if freq < half { 1 } else { 0 });
    println!("{},{} = {}", oxygen, scrubber, oxygen * scrubber);
}

fn generate_rating(input: &Vec<Vec<u32>>, target_fn: fn(u32, u32) -> u32) -> u32 {
    let mut index = 0;
    let mut current = input.clone();
    while current.len() > 1 {
        let freq = get_freq(&current);
        let half = ((current.len() + 1) / 2) as u32;
        let target = target_fn(freq[index], half);
        let mut next: Vec<Vec<u32>> = Vec::new();
        for row in current {
            if row[index] == target {
                next.push(row);
            }
        }
        current = next;
        index += 1;
    }
    let mut ans: u32 = 0;
    for i in current[0].iter() {
        ans *= 2;
        ans += i;
    }
    return ans;
}

fn get_freq(input: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut freq = input[0].clone();
    for i in 1..input.len() {
        for j in 0..freq.len() {
            freq[j] += input[i][j];
        }
    }
    freq
}
