fn is_safe(numbers: &Vec<i32>) -> bool {
    // vector with the difference between each number and the next one
    let diffs = numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .map(|(a, b)| (b - a))
        .collect::<Vec<i32>>();
    let all_decreasing = diffs.iter().all(|&x| x < 0);
    let all_increasing = diffs.iter().all(|&x| x > 0);
    let diffs_in_range = diffs
        .iter()
        .all(|&x| x.abs() >= 1 && x.abs() <= 3);
    diffs_in_range && (all_decreasing || all_increasing)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let default_path = String::from("./src/example.txt");
    let input_file = args.get(1).unwrap_or(&default_path);
    
    let input = std::fs::read_to_string(input_file).unwrap();
    let mut part_1 = 0;
    let mut part_2 = 0;
    for line in input.lines() {
        let numbers = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if is_safe(&numbers) {
            part_1 += 1;
        }
        for i in 0..numbers.len() {
            let mut modified = numbers.clone();
            modified.remove(i);
            if is_safe(&modified) {
                part_2 += 1;
                break;
            }
        }
    }
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
