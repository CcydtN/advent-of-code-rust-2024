use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(24);

#[derive(Debug, Clone)]
enum Helper {
    Value(bool),
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

impl Helper {
    fn resolve(key: String, map: &mut HashMap<String, Self>) -> Option<bool> {
        let val = match map.get(&key).cloned()? {
            Helper::Value(val) => return Some(val),
            Helper::And(a, b) => {
                let a = Self::resolve(a, map)?;
                let b = Self::resolve(b, map)?;
                a && b
            }
            Helper::Or(a, b) => {
                let a = Self::resolve(a, map)?;
                let b = Self::resolve(b, map)?;
                a || b
            }
            Helper::Xor(a, b) => {
                let a = Self::resolve(a, map)?;
                let b = Self::resolve(b, map)?;
                a ^ b
            }
        };
        map.insert(key, Self::Value(val));
        Some(val)
    }
}

fn parse<'a>(input: &'a str) -> HashMap<String, Helper> {
    let mut iter = input.split("\n\n");
    let mut map = HashMap::new();
    for line in iter.next().unwrap().lines() {
        let (name, value) = line.split(":").collect_tuple().unwrap();
        map.insert(name.trim().to_owned(), Helper::Value(value.trim() == "1"));
    }
    for line in iter.next().unwrap().lines() {
        let (a, op, b, _, name) = line.split_whitespace().collect_tuple().unwrap();
        let value = match op {
            "AND" => Helper::And(a.to_owned(), b.to_owned()),
            "OR" => Helper::Or(a.to_owned(), b.to_owned()),
            "XOR" => Helper::Xor(a.to_owned(), b.to_owned()),
            _ => unreachable!(),
        };
        map.insert(name.to_owned(), value);
    }
    map
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = parse(input);

    let mut value = 0;
    for i in 0..100 {
        let z = std::format!("z{i:02}");
        if let Some(val) = Helper::resolve(z, &mut map) {
            value += if val { 1 << i } else { 0 };
        }
    }
    Some(value)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, None);
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, None);
    }
}
