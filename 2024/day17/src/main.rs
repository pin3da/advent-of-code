use input_parsing::read_input_sections;

fn run_program(registers: &mut Vec<i64>, program: &Vec<i64>) -> Vec<i64> {
    let mut output = Vec::new();
    let mut inst_ptr = 0;

    let combo = |registers: &Vec<i64>, operand: i64| -> i64 {
        match operand {
            0..=3 => operand,
            4..=6 => registers[(operand - 4) as usize],
            7 => panic!("Reserved operand"),
            _ => panic!("Invalid operand"),
        }
    };

    while inst_ptr < program.len() {
        let inst = program[inst_ptr];
        let operand = program[inst_ptr + 1];
        match inst {
            0 => registers[0] >>= combo(&registers, operand),
            1 => registers[1] ^= operand,
            2 => registers[1] = combo(&registers, operand) & 7,
            3 => {
                if registers[0] != 0 {
                    inst_ptr = operand as usize;
                    continue;
                }
            }
            4 => registers[1] ^= registers[2],
            5 => output.push(combo(&registers, operand) & 7),
            6 => registers[1] = registers[0] >> combo(&registers, operand),
            7 => registers[2] = registers[0] >> combo(&registers, operand),
            _ => panic!("Invalid instruction {}", inst),
        }
        inst_ptr += 2;
    }

    output
}

fn find_input(program: &Vec<i64>, index: usize, a: i64) -> Option<i64> {
    if index >= program.len() {
        return Some(a);
    }

    for i in 0..8 {
        let next: i64 = (a << 3) | i;
        let output = run_program(&mut vec![next, 0, 0], program)[0];
        if output == program[index] {
            let ans = find_input(&program, index - 1, next);
            if ans.is_some() {
                return Some(ans.unwrap());
            }
        }
    }

    None
}

fn main() {
    let input = read_input_sections("./src/example.txt");
    let (mut registers, program) = parse_input(&input);
    println!(
        "Part 1: {}",
        run_program(&mut registers, &program)
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    println!("Looking for {:?}", program);
    let ans = find_input(&program, program.len() - 1, 0).unwrap();
    println!("Part 2: {}", ans);
}

fn parse_input(input: &Vec<String>) -> (Vec<i64>, Vec<i64>) {
    let registers = input
        .get(0)
        .unwrap()
        .lines()
        .map(|s| s.split_once(": ").unwrap().1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let program = input
        .get(1)
        .unwrap()
        .lines()
        .map(|s| s.split_once(" ").unwrap().1)
        .flat_map(|s| s.split(",").map(|s| s.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();
    (registers, program)
}
