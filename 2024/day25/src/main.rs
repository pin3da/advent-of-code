use input_parsing::read_input_sections;

fn main() {
    let input = read_input_sections("./src/example.txt");

    let mut locks = vec![];
    let mut keys = vec![];
    for section in input {
        let lines = section.lines().collect::<Vec<_>>();
        if lines[0].starts_with(".") {
            keys.push(parse_key(lines));
        } else {
            locks.push(parse_lock(lines));
        }
    }

    let mut ans = 0;
    for key in &keys {
        for lock in &locks {
            if fits(&key, &lock) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn fits(key: &Vec<i32>, lock: &Vec<i32>) -> bool {
    for i in 0..key.len() {
        if key[i] + lock[i] > 5 {
            return false;
        }
    }
    true
}

fn parse_lock(lines: Vec<&str>) -> Vec<i32> {
    let matrix = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let cols = matrix[0].len();

    let mut ans = vec![];
    for c in 0..cols {
        let mut h = 0;
        for r in 0..matrix.len() {
            if matrix[r][c] == '#' {
                h += 1;
            } else {
                break;
            }
        }
        ans.push(h - 1);
    }

    ans
}

fn parse_key(lines: Vec<&str>) -> Vec<i32> {
    let mut lines = lines.clone();
    lines.reverse();
    parse_lock(lines)
}
