fn main() {
    let args: Vec<String> = std::env::args().collect();
    let default_path = String::from("./src/example.txt");
    let input_file = args.get(1).unwrap_or(&default_path);

    let input_string = std::fs::read_to_string(input_file).unwrap();

    let ordering_rules = input_string
        .lines()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let max_label = 100;
    let mut goes_before = vec![vec![false; max_label]; max_label];
    for rule in &ordering_rules {
        let parts = rule.split("|").collect::<Vec<&str>>();
        let left = parts[0].parse::<usize>().unwrap();
        let right = parts[1].parse::<usize>().unwrap();
        goes_before[left][right] = true;
    }

    println!("Number of ordering rules: {}", ordering_rules.len());

    let updates = input_string
        .lines()
        .skip(ordering_rules.len() + 1)
        .collect::<Vec<&str>>();
    let mut part1 = 0;
    let mut part2 = 0;
    for update in &updates {
        let parts = update
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut in_order = true;
        for i in 0..parts.len() - 1 {
            for j in i + 1..parts.len() {
                if !goes_before[parts[i]][parts[j]] {
                    in_order = false;
                    break;
                }
            }
        }
        if in_order {
            let middle = parts.len() / 2;
            part1 += parts[middle];
        } else {
            let mut sorted = parts.clone();
            sorted.sort_by(|a, b| {
                if goes_before[*a][*b] {
                    std::cmp::Ordering::Less
                } else if goes_before[*b][*a] {
                    std::cmp::Ordering::Greater
                } else {
                    a.cmp(b)
                }
            });
            let middle = sorted.len() / 2;
            part2 += sorted[middle];
        }
    }
    // wrong attempts part 1:
    //    9449 -> too high. Note: It was not a transitive ordering. Checking the explicit ordering rules.
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
