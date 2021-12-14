use std::collections::HashMap;

type Rule = (char, char);

fn main() {
    let original = include_str!("input.txt");
    let first_line = original.lines().next().unwrap();
    let input = parse_input(original);
    let mut templ = input.0;
    for i in 0..40 {
        templ = insert(templ, &input.1);
        if i == 9 {
            println!("Part 1 {}", get_diff(&templ, first_line));
        }
    }
    println!("Part 2 {}", get_diff(&templ, first_line));
}

fn get_diff(templ: &HashMap<Rule, i64>, original: &str) -> i64 {
    let mut freq: HashMap<char, i64> = HashMap::new();
    for (key, val) in templ {
        *freq.entry(key.0).or_default() += val;
        *freq.entry(key.1).or_default() += val;
    }

    let first = original.chars().nth(0).unwrap();
    let last = original.chars().last().unwrap();
    *freq.entry(first).or_default() += 1;
    *freq.entry(last).or_default() += 1;

    let mut ans: Vec<i64> = freq.into_values().map(|it| it / 2).collect();
    ans.sort();
    ans.last().unwrap() - ans.first().unwrap()
}

fn parse_input(s: &str) -> (HashMap<Rule, i64>, HashMap<Rule, char>) {
    let mut lines = s.lines();
    let tmp: Vec<char> = lines.next().unwrap().chars().collect();
    let mut template = HashMap::new();
    for i in 0..tmp.len() - 1 {
        *template.entry((tmp[i], tmp[i + 1])).or_default() += 1;
    }
    assert!(lines.next().unwrap().is_empty());

    let mut rules = HashMap::new();
    for i in lines {
        rules.insert(
            (i.chars().nth(0).unwrap(), i.chars().nth(1).unwrap()),
            i.chars().nth(6).unwrap(),
        );
    }

    (template, rules)
}

fn insert(templ: HashMap<Rule, i64>, rules: &HashMap<Rule, char>) -> HashMap<Rule, i64> {
    let mut ans: HashMap<Rule, i64> = HashMap::new();

    for (key, val) in templ {
        if let Some(c) = rules.get(&key) {
            let a = (key.0, *c);
            let b = (*c, key.1);
            *ans.entry(a).or_default() += val;
            *ans.entry(b).or_default() += val;
        }
    }

    ans
}
