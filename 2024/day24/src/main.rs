use input_parsing::read_input_sections;
use std::collections::BTreeMap;

#[derive(Debug)]
enum GateType<'a> {
    AND(&'a str, &'a str),
    OR(&'a str, &'a str),
    XOR(&'a str, &'a str),
    LITERAL(bool),
}

#[derive(Debug)]
struct Gate<'a> {
    typ: GateType<'a>,
    value: Option<bool>,
}

impl<'a> Gate<'a> {
    fn eval(&self, gates: &BTreeMap<&str, Gate>) -> bool {
        if let Some(value) = self.value {
            return value;
        }

        let result = match self.typ {
            GateType::AND(left, right) => {
                let gate1 = gates.get(left).unwrap();
                let gate2 = gates.get(right).unwrap();
                gate1.eval(gates) && gate2.eval(gates)
            }
            GateType::OR(left, right) => {
                let gate1 = gates.get(left).unwrap();
                let gate2 = gates.get(right).unwrap();
                gate1.eval(gates) || gate2.eval(gates)
            }
            GateType::XOR(left, right) => {
                let gate1 = gates.get(left).unwrap();
                let gate2 = gates.get(right).unwrap();
                gate1.eval(gates) ^ gate2.eval(gates)
            }
            GateType::LITERAL(value) => value,
        };

        result
    }
}

fn main() {
    let sections = read_input_sections("./src/example.txt");

    let mut gates: BTreeMap<&str, Gate> = BTreeMap::new();
    for gate in sections[0].lines() {
        let (name, value) = gate.split_once(": ").unwrap();
        let value = value == "1";
        let gate = Gate {
            typ: GateType::LITERAL(value),
            value: None,
        };
        gates.insert(name, gate);
    }

    for gate in sections[1].lines() {
        let (input, name) = gate.split_once(" -> ").unwrap();
        let parts: Vec<&str> = input.split_whitespace().collect();
        let gate_type = match parts[1] {
            "AND" => GateType::AND(parts[0], parts[2]),
            "OR" => GateType::OR(parts[0], parts[2]),
            "XOR" => GateType::XOR(parts[0], parts[2]),
            _ => panic!("Unknown gate type"),
        };
        let new_gate = Gate {
            typ: gate_type,
            value: None,
        };
        gates.insert(name, new_gate);
    }

    println!("There are {} gates", gates.len());

    let mut result = vec![];
    for (name, gate) in &gates {
        if name.starts_with("z") {
            result.push(gate.eval(&gates));
        }
    }
    result.reverse();

    let mut part1 : usize = 0;
    for b in &result {
        part1 <<= 1;
        if *b {
            part1 |= 1;
        }
    }

    // 412579686 -> Too low. Was due to integer overflow.
    println!("part1: {}", part1);

    // I renedered the graph (plot.rs) and found the candidates visually.
    let mut candidates = vec!["rts", "z07", "z26", "kgj", "chv", "vvw", "z12", "jpj"];
    candidates.sort();

    let candidates_str = candidates.join(",");
    println!("Part 2: {}", candidates_str);

}
