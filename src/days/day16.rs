use crate::AOCDay;

/*
 * Day 16: Packet Decoder
 *
 * Parse a encoded packet and evaluate its contents.
 */

pub struct Day16();

impl AOCDay for Day16 {
    fn part1(&self, _input: &str) -> Option<String> { Some(part1(_input)) }
    fn part2(&self, _input: &str) -> Option<String> { Some(part2(_input)) }
    fn get_num(&self) -> u32 { 16 }
}

pub fn get() -> Day16 {Day16()}

#[derive(Debug)]
enum PType {
    Literal(u64, u64, u64), // version, type ID, value
    Operator(u64, u64, bool, u64) //version, type ID, length type ID, length
}

#[derive(Debug)]
struct Packet {
    ptype: PType,
    subpackets: Vec<Packet> // empty for Literal
}

impl Packet {
    fn version_sum(&self) -> u64 {
        match self.ptype {
            PType::Literal(v,_,_) => {v},
            PType::Operator(v,_,_,_) => {
                let sub_sum: u64 = self.subpackets.iter().map(|p| p.version_sum()).sum();
                return v + sub_sum;
            },
        }
    }
}

fn part1(input: &str) -> String {
    let binary = hex_to_binary(&input);
    let (_, p) = parse_packet(&binary);
    p.version_sum().to_string()
}

fn part2(input: &str) -> String {
    let binary = hex_to_binary(&input);
    let (_, p) = parse_packet(&binary);
    evaluate_packet(&p).to_string()
}

fn evaluate_packet(p: &Packet) -> u64 {
    match p.ptype {
        PType::Literal(_,_,val) => {val},
        PType::Operator(_,t_id,_,_) => {
            match t_id {
                0 => {p.subpackets.iter().map(evaluate_packet).sum()},
                1 => {p.subpackets.iter().map(evaluate_packet).product()},
                2 => {p.subpackets.iter().map(evaluate_packet).min().unwrap()},
                3 => {p.subpackets.iter().map(evaluate_packet).max().unwrap()},
                5 => {if evaluate_packet(&p.subpackets[0]) > evaluate_packet(&p.subpackets[1]) {1} else {0}},
                6 => {if evaluate_packet(&p.subpackets[0]) < evaluate_packet(&p.subpackets[1]) {1} else {0}},
                7 => {if evaluate_packet(&p.subpackets[0]) == evaluate_packet(&p.subpackets[1]) {1} else {0}},
                _ => {panic!("Invalid Type ID");}
            }
        },
    }
}

fn parse_literal(input: &str) -> (&str, Packet) {
    let version_s: &str = &input[0..3];
    let type_id_s: &str = &input[3..6];
    let version = u64::from_str_radix(version_s, 2).unwrap();
    let type_id = u64::from_str_radix(type_id_s, 2).unwrap();
    assert_eq!(type_id, 4, "Packet is not of type_id 4, in parsing literal");

    let mut num: u64 = 0;
    let chars = &input[6..];
    let mut char_iter = chars.chars();
    while let Some(b) = char_iter.next() {
        let num_s: String = char_iter.clone().take(4).collect();
        char_iter.nth(3); // skip 4 elements
        num = (num << 4) | u64::from_str_radix(&num_s, 2).unwrap();
        match b {
            '1' => {continue;},
            '0' => {break;},
            _ => panic!("Invalid binary in parse literal")
        }
    }

    let packet = Packet {
        ptype: PType::Literal(version, type_id, num),
        subpackets: Vec::new()
    };
    let result = (char_iter.as_str(), packet);
    result
}

fn parse_operator(input: &str) -> (&str, Packet) {
    let type_id_s: &str = &input[3..6];
    let type_id = u64::from_str_radix(type_id_s, 2).unwrap();
    assert!(type_id != 4, "Packet is of type_id 4, in parsing operator");
    let length_type = &input[6..7].chars().next().unwrap();

    match length_type {
        '1' => {
            return parse_subpackets_type1(&input);
        },
        '0' => {
            return parse_subpackets_type0(&input);
        },
        _ => panic!("Invalid length type: {}", length_type),
    }
}

fn parse_subpackets_type0(input: &str) -> (&str, Packet) {
    let version_s: &str = &input[0..3];
    let version = u64::from_str_radix(version_s, 2).unwrap();
    let type_id_s: &str = &input[3..6];
    let type_id = u64::from_str_radix(type_id_s, 2).unwrap();
    let length_s = &input[7..22];
    let length = u64::from_str_radix(length_s, 2).unwrap();
    // parse subpackets
    let rest = &input[22..];
    let mut r = rest;
    let mut subpackets: Vec<Packet> = Vec::new();
    loop {
        let (rr, p) = parse_packet(r);
        r = rr;
        subpackets.push(p);
        let diff = (rest.len() - r.len()) as u64;
        if diff < length {
            continue;
        }else if diff == length {
            break; // we have parsed the length in packets
        }else {
            panic!("Parsed more bits than specified");
        }
    }

    (r, Packet {
        ptype: PType::Operator(version, type_id, false, length),
        subpackets
    })
}

fn parse_subpackets_type1(input: &str) -> (&str, Packet) {
    let version_s: &str = &input[0..3];
    let version = u64::from_str_radix(version_s, 2).unwrap();
    let type_id_s: &str = &input[3..6];
    let type_id = u64::from_str_radix(type_id_s, 2).unwrap();
    let length_s = &input[7..18];
    let length = u64::from_str_radix(length_s, 2).unwrap();
    // parse subpackets
    let mut r = &input[18..];
    let mut subpackets: Vec<Packet> = Vec::new();
    for _ in 0..length {
        let (rr, p) = parse_packet(r);
        r = rr;
        subpackets.push(p);
    }

    (r, Packet {
        ptype: PType::Operator(version, type_id, true, length),
        subpackets
    })
}

fn parse_packet(input: &str) -> (&str, Packet) {
    let type_id_s = &input[3..6];
    let type_id = u64::from_str_radix(type_id_s, 2).unwrap();
    match type_id {
        4 => {return parse_literal(input);},
        _ => {return parse_operator(input);},
    }
}

pub fn hex_to_binary(input: &str) -> String {
    let mut result = String::new();
    for c in input.trim().chars() {
        let n = u32::from_str_radix(&c.to_string(), 16).unwrap();
        result.push_str(&format!("{:04b}", n));
    }
    result
}

fn test_input_literal() -> String {
    String::from("D2FE28")
}

fn test_input_operator_type0() -> String {
    String::from("38006F45291200")
}

fn test_input_operator_type1() -> String {
    String::from("EE00D40C823060")
}

fn test_input_nested1() -> String {
    String::from("8A004A801A8002F478")
}

fn test_input_nested2() -> String {
    String::from("620080001611562C8802118E34")
}

fn test_input_nested3() -> String {
    String::from("C0015000016115A2E0802F182340")
}

fn test_input_nested4() -> String {
    String::from("A0016C880162017C3686B18A3D4780")
}
