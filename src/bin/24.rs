use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
    usize,
};

use itertools::Itertools;

advent_of_code::solution!(24);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Gate {
    Value(bool),
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

impl Gate {
    fn and(a: String, b: String) -> Gate {
        if a < b {
            Gate::And(a, b)
        } else {
            Gate::And(b, a)
        }
    }

    fn or(a: String, b: String) -> Gate {
        if a < b {
            Gate::Or(a, b)
        } else {
            Gate::Or(b, a)
        }
    }

    fn xor(a: String, b: String) -> Gate {
        if a < b {
            Gate::Xor(a, b)
        } else {
            Gate::Xor(b, a)
        }
    }

    fn is_or(&self) -> bool {
        match self {
            Gate::Or(_, _) => true,
            _ => false,
        }
    }

    fn is_xor(&self) -> bool {
        match self {
            Gate::Xor(_, _) => true,
            _ => false,
        }
    }

    fn is_and(&self) -> bool {
        match self {
            Gate::And(_, _) => true,
            _ => false,
        }
    }

    fn get_val(&self) -> (&str, &str) {
        match self {
            Gate::Value(_) => unreachable!(),
            Gate::And(a, b) | Gate::Or(a, b) | Gate::Xor(a, b) => return (a, b),
        }
    }

    fn is_base_xor(&self) -> Option<usize> {
        match self {
            Gate::Xor(a, b) => {
                let a = a[1..].parse::<usize>().ok()?;
                let b = b[1..].parse::<usize>().ok()?;
                if a == b {
                    Some(a)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn is_base_and(&self) -> bool {
        match self {
            Gate::And(a, b) => {
                let a = a[1..].parse::<usize>();
                let b = b[1..].parse::<usize>();
                if a.is_err() || b.is_err() {
                    return false;
                }
                if a.unwrap() == b.unwrap() {
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    fn resolve(key: String, map: &mut HashMap<String, Self>) -> Option<bool> {
        let val = match map.get(&key).cloned()? {
            Gate::Value(val) => return Some(val),
            Gate::And(a, b) => {
                let a = Self::resolve(a, map)?;
                let b = Self::resolve(b, map)?;
                a && b
            }
            Gate::Or(a, b) => {
                let a = Self::resolve(a, map)?;
                let b = Self::resolve(b, map)?;
                a || b
            }
            Gate::Xor(a, b) => {
                let a = Self::resolve(a, map)?;
                let b = Self::resolve(b, map)?;
                a ^ b
            }
        };
        map.insert(key, Self::Value(val));
        Some(val)
    }
}

fn parse<'a>(input: &'a str) -> HashMap<String, Gate> {
    let mut iter = input.split("\n\n");
    let mut map = HashMap::new();
    for line in iter.next().unwrap().lines() {
        let (name, value) = line.split(":").collect_tuple().unwrap();
        map.insert(name.trim().to_owned(), Gate::Value(value.trim() == "1"));
    }
    for line in iter.next().unwrap().lines() {
        let (a, op, b, _, name) = line.split_whitespace().collect_tuple().unwrap();
        let value = match op {
            "AND" => Gate::and(a.to_owned(), b.to_owned()),
            "OR" => Gate::or(a.to_owned(), b.to_owned()),
            "XOR" => Gate::xor(a.to_owned(), b.to_owned()),
            _ => unreachable!(),
        };
        map.insert(name.to_owned(), value);
    }
    map
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = parse(input);

    let mut z_val = 0;
    for i in 0..100 {
        let key = std::format!("z{i:02}");
        if let Some(val) = Gate::resolve(key, &mut map) {
            z_val += if val { 1 << i } else { 0 };
        }
    }
    Some(z_val)
}

pub fn part_two(input: &str) -> Option<String> {
    // z{i} should be the result of the xor result of two part,
    // (x{i} ^ y{i}) and the Carry bit from i-1
    // Or gate is a camouflage to hide the detail.
    //
    // ** z00 should only have (x{i} ^ y{i}), and it is correct

    // y00 XOR x00 -> z00
    //
    // tcd XOR bwv -> z01
    // y01 XOR x01 -> tcd, adding part
    // y00 AND x00 -> bwv, direct carry bit from z00
    //
    // frj XOR hqq -> z02
    // y02 XOR x02 -> frj, adding part
    // sgv OR wqt -> hqq, carry bit from z01
    // x01 AND y01 -> wqt, direct carry bit i-1
    // bwv AND tcd -> sgv, indirect carry bit
    //
    // ckv XOR bbh -> z03
    // bkc OR wsq -> ckv, carry bit from z02
    // y02 AND x02 -> wsq, carry bit from z02 directly
    // hqq AND frj -> bkc, carry bit from z01 indirectly
    // y03 XOR x03 -> bbh, adding part

    let mut map = parse(input);
    let mut inverse_map = map
        .iter()
        .map(|(k, v)| (v.clone(), k.clone()))
        .collect::<HashMap<_, _>>();

    let z_msb = map
        .keys()
        .filter_map(|val| val[1..].parse::<usize>().ok())
        .max()
        .unwrap();

    // manually swap
    swap(
        "qff".to_owned(),
        "qnw".to_owned(),
        &mut map,
        &mut inverse_map,
    );
    swap(
        "z23".to_owned(),
        "qqp".to_owned(),
        &mut map,
        &mut inverse_map,
    );
    swap(
        "z36".to_owned(),
        "fbq".to_owned(),
        &mut map,
        &mut inverse_map,
    );
    swap(
        "z16".to_owned(),
        "pbv".to_owned(),
        &mut map,
        &mut inverse_map,
    );

    let xor_gates = map.iter().filter(|(_, val)| val.is_xor()).collect_vec();
    let non_base_xor_gates = xor_gates
        .iter()
        .filter(|(_, val)| val.is_base_xor().is_none())
        .cloned()
        .collect_vec();
    // xor_gates should be either (x{i} XOR y{i} => _) or (_ XOR _) => z{i}
    let failed_xor = non_base_xor_gates
        .iter()
        .filter(|(key, val)| {
            let target = key[1..].parse::<usize>();
            if target.is_err() {
                return true;
            } else {
                let (a, b) = val.get_val();
                let x = map[a].is_base_xor();
                let y = map[b].is_base_xor();
                match (x, y) {
                    (Some(i), None) if Ok(i) == target => {
                        return i != 1 && !map[b].is_or();
                    }
                    (None, Some(i)) if Ok(i) == target => {
                        return i != 1 && !map[a].is_or();
                    }
                    _ => {
                        // dbg!(key, &map[a], &map[b]);
                        true
                    }
                }
            }
        })
        .cloned()
        .map(|(k, v)| (k.clone(), v))
        .collect_vec();
    dbg!(&failed_xor);

    let failed_z = (0..z_msb)
        .map(|key| format!("z{key:02}"))
        .filter(|(key)| !map[key].is_xor())
        .map(|key| (key.clone(), &map[&key]))
        .collect_vec();
    dbg!(&failed_z);

    let mut x_val = 0u64;
    for i in 0..=z_msb {
        let key = std::format!("x{i:02}");
        if let Some(val) = Gate::resolve(key, &mut map) {
            x_val += if val { 1 << i } else { 0 };
        }
    }
    let mut y_val = 0u64;
    for i in 0..=z_msb {
        let key = std::format!("y{i:02}");
        if let Some(val) = Gate::resolve(key, &mut map) {
            y_val += if val { 1 << i } else { 0 };
        }
    }
    let mut z_val = 0u64;
    for i in 0..=z_msb {
        let key = std::format!("z{i:02}");
        if let Some(val) = Gate::resolve(key, &mut map) {
            z_val += if val { 1 << i } else { 0 };
        }
    }
    assert_eq!(x_val + y_val, z_val);
    let mut result = vec!["qff", "qnw", "z23", "qqp", "z36", "fbq", "z16", "pbv"];
    result.sort();
    Some(result.into_iter().join(","))
}

fn swap(
    key_a: String,
    key_b: String,
    map: &mut HashMap<String, Gate>,
    inverse_map: &mut HashMap<Gate, String>,
) {
    let a = map.remove(&key_a).unwrap();
    let b = map.remove(&key_b).unwrap();
    let _ = inverse_map.insert(b.to_owned(), key_a.clone()).unwrap();
    let _ = inverse_map.insert(a.to_owned(), key_b.clone()).unwrap();
    map.insert(key_a, b);
    map.insert(key_b, a);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        // both example is not directly related to solution
    }
}
