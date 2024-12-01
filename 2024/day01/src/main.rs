use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let default_path = String::from("./src/example.txt");
    let input_file = args.get(1).unwrap_or(&default_path);
    
    let input = std::fs::read_to_string(input_file).unwrap();
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();
    for line in input.lines() {
        let (left, right) = line.split_once("   ").unwrap();
        left_numbers.push(left.parse::<i32>().unwrap());
        right_numbers.push(right.parse::<i32>().unwrap());
    }

    let mut total_distance = 0;

    left_numbers.sort();
    right_numbers.sort();

    for (left, right) in left_numbers.iter().zip(right_numbers.iter()) {
        total_distance += (left - right).abs();
    }
    println!("Part 1: {}", total_distance);

    let mut frequency_map = HashMap::new(); 
    for number in right_numbers {
        *frequency_map.entry(number).or_insert(0) += 1;
    }
    let mut similarity = 0;
    for number in left_numbers {
        similarity += number * (*frequency_map.get(&number).unwrap_or(&0));
    }
    println!("Part 2: {}", similarity);
}
