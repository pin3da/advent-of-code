use std::collections::VecDeque;

fn main() {
    println!("part 1 {}", solve_1(include_str!("input.txt")));
    println!("part 2 {}", solve_2(include_str!("input.txt")));
}

#[derive(Debug)]
struct Packet {
    version: i64,
    type_id: i64,
    sum_versions: i64,
    sum_bits: i64,
    value: i64,
}

impl Packet {
    fn parse(input: &mut VecDeque<bool>) -> Packet {
        let version = consume_n(input, 3);
        let type_id = consume_n(input, 3);
        let mut sum_versions = version;
        let mut sum_bits: i64 = 6;

        let value = if type_id == 4 {
            let (constant, len_bits) = Packet::parse_constant(input);
            sum_bits += len_bits as i64;
            constant
        } else {
            let len_type_id = input.pop_front().unwrap();
            sum_bits += 1;
            let mut sub_packets = Vec::new();
            if len_type_id {
                let num_sub_packets = consume_n(input, 11);
                sum_bits += 11;
                for _ in 0..num_sub_packets {
                    sub_packets.push(Packet::parse(input));
                }
            } else {
                let total_length = consume_n(input, 15);
                sum_bits += 15;
                let mut bits_seen = 0;

                while bits_seen < total_length {
                    let sub = Packet::parse(input);
                    bits_seen += sub.sum_bits;
                    sub_packets.push(sub);
                }
                assert_eq!(bits_seen, total_length);
            }

            for sub in sub_packets.iter() {
                sum_versions += sub.sum_versions;
                sum_bits += sub.sum_bits;
            }

            apply_op(&sub_packets, type_id)
        };

        let ans = Packet {
            version,
            type_id,
            sum_versions,
            sum_bits,
            value,
        };
        ans
    }

    fn parse_constant(input: &mut VecDeque<bool>) -> (i64, i32) {
        let mut buffer = VecDeque::new();
        let mut len_bits = 0;
        while input.len() >= 5 {
            let should_stop = !input.pop_front().unwrap();
            for _ in 0..4 {
                buffer.push_back(input.pop_front().unwrap());
            }
            len_bits += 5;
            if should_stop {
                break;
            }
        }
        let len_buffer = buffer.len() as i32;
        (consume_n(&mut buffer, len_buffer), len_bits)
    }
}

fn solve_1(s: &str) -> i64 {
    let mut input = parse_input(s);
    let packet = Packet::parse(&mut input);
    packet.sum_versions
}

fn solve_2(s: &str) -> i64 {
    let mut input = parse_input(s);
    let packet = Packet::parse(&mut input);
    packet.value
}

fn parse_input(s: &str) -> VecDeque<bool> {
    let mut input = VecDeque::new();
    for c in s.chars() {
        let val = c.to_digit(16).unwrap();
        for i in 0..4 {
            input.push_back((val & (1 << (3 - i))) != 0);
        }
    }

    input
}

fn consume_n(input: &mut VecDeque<bool>, n: i32) -> i64 {
    assert!(n < 64);
    let mut ans = 0;
    for _ in 0..n {
        ans *= 2;
        ans += if input.pop_front().unwrap() { 1 } else { 0 };
    }
    ans
}

fn apply_op(packets: &Vec<Packet>, inst: i64) -> i64 {
    let values = packets.iter().map(|p| p.value);
    match inst {
        0 => values.sum(),
        1 => values.fold(1, |prod, val| prod * val),
        2 => values.reduce(i64::min).unwrap(),
        3 => values.reduce(i64::max).unwrap(),
        5 => {
            if packets[0].value > packets[1].value {
                1
            } else {
                0
            }
        }
        6 => {
            if packets[0].value < packets[1].value {
                1
            } else {
                0
            }
        }
        _ => {
            if packets[0].value == packets[1].value {
                1
            } else {
                0
            }
        }
    }
}

#[test]
fn example_1() {
    assert_eq!(solve_1("D2FE28"), 6);
    assert_eq!(solve_1("8A004A801A8002F478"), 16);
    assert_eq!(solve_1("620080001611562C8802118E34"), 12);
    assert_eq!(solve_1("C0015000016115A2E0802F182340"), 23);
    assert_eq!(solve_1("A0016C880162017C3686B18A3D4780"), 31);
}

#[test]
fn example_2() {
    assert_eq!(solve_2("C200B40A82"), 3);
    assert_eq!(solve_2("04005AC33890"), 54);
    assert_eq!(solve_2("880086C3E88112"), 7);
    assert_eq!(solve_2("9C0141080250320F1802104A08"), 1);
}
