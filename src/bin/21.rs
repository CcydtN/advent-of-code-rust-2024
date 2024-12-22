use itertools::{iproduct, Itertools};
use num::ToPrimitive;
use std::{collections::HashMap, sync::LazyLock};

advent_of_code::solution!(21);

type Mapping = HashMap<char, (usize, usize)>;

#[derive(Debug, Clone, Copy)]
enum KeypadType {
    Numeric,
    Directional,
}

impl KeypadType {
    fn get_mapping(&self) -> &'static Mapping {
        match self {
            KeypadType::Numeric => get_numeric_keypad(),
            KeypadType::Directional => get_directional_keypad(),
        }
    }

    fn is_valid(&self, pos: (usize, usize)) -> bool {
        self.get_mapping().values().find(|x| x == &&pos).is_some()
    }
}

fn get_numeric_keypad() -> &'static Mapping {
    const LAYOUT: [(char, (usize, usize)); 11] = [
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ];
    static KEYPAD: LazyLock<Mapping> = LazyLock::new(|| LAYOUT.into_iter().collect::<Mapping>());
    &KEYPAD
}

fn get_directional_keypad() -> &'static Mapping {
    const LAYOUT: [(char, (usize, usize)); 5] = [
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ];
    static KEYPAD: LazyLock<Mapping> = LazyLock::new(|| LAYOUT.into_iter().collect::<Mapping>());
    &KEYPAD
}

fn get_possible_sequence(
    keypad_type: KeypadType,
    start: &(usize, usize),
    end: &(usize, usize),
) -> Vec<Vec<char>> {
    let vertical_char = if start.0 > end.0 { '^' } else { 'v' };
    let horizontal_char = if start.1 > end.1 { '<' } else { '>' };

    let vertical = vec![vertical_char; start.0.abs_diff(end.0)];
    let horizontal = vec![horizontal_char; start.1.abs_diff(end.1)];

    let len = start.0.abs_diff(end.0) + start.1.abs_diff(end.1);

    [vertical, horizontal]
        .into_iter()
        .flatten()
        .permutations(len)
        .unique()
        .filter(|path| check_path(keypad_type, start, path))
        .map(|mut v| {
            v.push('A');
            v
        })
        .collect_vec()
}

fn check_path(keypad_type: KeypadType, start: &(usize, usize), path: &[char]) -> bool {
    let mut pos = start.clone();
    for c in path {
        match c {
            '^' => pos.0 -= 1,
            'v' => pos.0 += 1,
            '<' => pos.1 -= 1,
            '>' => pos.1 += 1,
            _ => unreachable!(),
        }
        if !keypad_type.is_valid(pos) {
            return false;
        }
    }
    true
}

fn helper(input: &[char], mapping: &HashMap<(char, char), usize>) -> usize {
    let mut last_key = 'A';
    let mut sum = 0;
    for &key in input {
        sum += mapping[&(last_key, key)];
        last_key = key;
    }
    sum
}

fn generate_seq_map(middle_layer_count: usize) -> HashMap<(char, char), usize> {
    let mut keypad_types = vec![KeypadType::Directional; middle_layer_count];
    keypad_types.push(KeypadType::Numeric);

    let mut seq_length = HashMap::new();
    let keypad = keypad_types[0].get_mapping();
    for (&start, &end) in iproduct!(keypad.keys(), keypad.keys()) {
        let start_pos = keypad[&start];
        let end_pos = keypad[&end];
        let cost = start_pos.0.abs_diff(end_pos.0) + start_pos.1.abs_diff(end_pos.1) + 1;
        seq_length.insert((start, end), cost);
    }

    for &keypad_type in keypad_types.iter().skip(1) {
        let mut next_seq_length = HashMap::new();
        let keypad = keypad_type.get_mapping();
        for (&start, &end) in iproduct!(keypad.keys(), keypad.keys()) {
            let sequences = get_possible_sequence(keypad_type, &keypad[&start], &keypad[&end]);
            let result = sequences
                .into_iter()
                .map(|seq| helper(&seq, &seq_length))
                .min()
                .unwrap();
            next_seq_length.insert((start, end), result);
        }
        seq_length = next_seq_length;
    }
    seq_length
}

pub fn part_one(input: &str) -> Option<u64> {
    let codes = input.split_whitespace().collect_vec();
    let numeric_part = codes
        .iter()
        .map(|&code| code[..3].parse::<usize>().unwrap())
        .collect_vec();
    let mapping = generate_seq_map(2);

    let sequence = codes
        .iter()
        .map(|code| code.chars().collect_vec())
        .map(|code| helper(&code, &mapping))
        .collect_vec();

    sequence
        .into_iter()
        .zip(numeric_part)
        .map(|(a, b)| dbg!(a) * dbg!(b))
        .sum::<usize>()
        .to_u64()
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
