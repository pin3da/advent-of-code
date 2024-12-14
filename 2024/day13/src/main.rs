use input_parsing::read_input_sections;

fn main() {
    let input = parse_input();
    let mut part1 = 0;
    let mut part2 = 0;
    for (a, b, prize) in input {
        let (x_1, y_1) = a;
        let (x_2, y_2) = b;
        let (x_prize, y_prize) = prize;
        if let Some((times_a, times_b)) = find_cost(x_1, y_1, x_2, y_2, x_prize, y_prize) {
            if times_a <= 100 && times_b <= 100 {
                part1 += times_a * 3 + times_b;
            }
        }
        let extra = 10000000000000;
        if let Some((times_a, times_b)) =
            find_cost(x_1, y_1, x_2, y_2, x_prize + extra, y_prize + extra)
        {
            part2 += times_a * 3 + times_b;
        }
    }
    println!("{}", part1);
    println!("{}", part2);
}

fn find_cost(x_1: i64, y_1: i64, x_2: i64, y_2: i64, x: i64, y: i64) -> Option<(i64, i64)> {
    let det = x_1 * y_2 - x_2 * y_1;
    if det == 0 {
        return None;
    }
    let a = x * y_2 - x_2 * y;
    let b = x_1 * y - x * y_1;
    if a % det != 0 || b % det != 0 {
        return None;
    }
    Some(( a / det, b / det))
}

fn parse_input() -> Vec<((i64, i64), (i64, i64), (i64, i64))> {
    let input = read_input_sections("./src/example.txt");
    let mut result = vec![];
    for section in input {
        fn parse_coords(line: &str, delimiter: &str) -> (i64, i64) {
            if let Some((_, coords)) = line.split_once(": ") {
                let mut nums = coords.split(", ");
                let x = nums
                    .next()
                    .unwrap()
                    .trim_start_matches("X")
                    .trim_start_matches(delimiter);
                let y = nums
                    .next()
                    .unwrap()
                    .trim_start_matches("Y")
                    .trim_start_matches(delimiter);
                (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
            } else {
                panic!("Invalid input format")
            }
        }
        let mut lines = section.lines();
        let (x_a, y_a) = parse_coords(lines.next().unwrap(), "+");
        let (x_b, y_b) = parse_coords(lines.next().unwrap(), "+");
        let (x_prize, y_prize) = parse_coords(lines.next().unwrap(), "=");
        result.push(((x_a, y_a), (x_b, y_b), (x_prize, y_prize)));
    }
    result
}
