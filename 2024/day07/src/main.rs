use input_parsing::read_input;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

fn generate_operation_combinations(length: usize, ops: Vec<Operation>) -> Vec<Vec<Operation>> {
    let mut result = Vec::new();

    fn generate_recursive(
        current: Vec<Operation>,
        length: usize,
        ops: &[Operation],
        result: &mut Vec<Vec<Operation>>,
    ) {
        if current.len() == length {
            result.push(current);
            return;
        }

        for &op in ops {
            let mut new_combination = current.clone();
            new_combination.push(op);
            generate_recursive(new_combination, length, ops, result);
        }
    }

    generate_recursive(Vec::new(), length, &ops, &mut result);
    result
}

fn concat(a: usize, b: usize) -> usize {
    let mut result = a;
    let mut c = b;
    while c > 0 {
        result *= 10;
        c /= 10;
    }
    result + b
}

fn can_build(target: usize, vals: &[usize], ops: Vec<Operation>) -> bool {
    let op_len = vals.len() - 1;
    let combinations = generate_operation_combinations(op_len, ops);
    for combination in combinations {
        let mut result = vals[0];
        for i in 0..op_len {
            match combination[i] {
                Operation::Add => result += vals[i + 1],
                Operation::Multiply => result *= vals[i + 1],
                Operation::Concatenate => result = concat(result, vals[i + 1]),
            }
        }
        if result == target {
            return true;
        }
    }
    false
}

fn main() {
    let input = read_input("./src/example.txt");
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let (target, vals) = line.split_once(":").unwrap();
        let target = target.parse::<usize>().unwrap();
        let vals = vals
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        if can_build(target, &vals, vec![Operation::Add, Operation::Multiply]) {
            part1 += target;
            part2 += target;
        } else if can_build(
            target,
            &vals,
            vec![Operation::Add, Operation::Multiply, Operation::Concatenate],
        ) {
            part2 += target;
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
