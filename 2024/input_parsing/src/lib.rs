use std::fs;

pub fn read_input(default_path: &str) -> String {
    let args: Vec<String> = std::env::args().collect();
    let default = String::from(default_path);
    let input_file = args.get(1).unwrap_or(&default);
    fs::read_to_string(input_file).unwrap()
}

pub fn read_input_sections(default_path: &str) -> Vec<String> {
    read_input(default_path)
        .split("\n\n")
        .map(|s| s.to_string())
        .collect()
}
