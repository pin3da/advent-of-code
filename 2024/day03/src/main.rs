use regex;

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Mult(i32, i32),
    Do,
    Dont,
}

fn next_token(input: &mut &str) -> Option<Token> {
    // Order matters.
    let token_patterns: Vec<(&str, fn(&regex::Captures) -> Token)> = vec![
        (r"^do\(\)", |_| Token::Do),
        (r"^don't\(\)", |_| Token::Dont),
        (
            r"^mul\((\d{1,3}),(\d{1,3})\)",
            |c: &regex::Captures| -> Token {
                Token::Mult(c[1].parse::<i32>().unwrap(), c[2].parse::<i32>().unwrap())
            },
        ),
    ];

    while !input.is_empty() {
        for (pattern, token_fn) in &token_patterns {
            let re = regex::Regex::new(pattern).unwrap();
            if let Some(captured) = re.captures(input) {
                let end = captured.get(0).unwrap().end();

                let token = token_fn(&captured);

                // Advance the input past the match
                *input = &input[end..];
                return Some(token);
            }
        }

        *input = &input[1..];
    }

    None
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let default_path = String::from("./src/example.txt");
    let input_file = args.get(1).unwrap_or(&default_path);

    let input_string = std::fs::read_to_string(input_file).unwrap();
    let mut input: &str = &input_string;

    let mut part1 = 0;
    let mut part2 = 0;
    let mut enabled = true;
    while let Some(token) = next_token(&mut input) {
        match token {
            Token::Mult(a, b) => {
                part1 += a * b;
                if enabled {
                    part2 += a * b;
                }
            }
            Token::Do => {
                enabled = true;
            }
            Token::Dont => {
                enabled = false;
            }
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
