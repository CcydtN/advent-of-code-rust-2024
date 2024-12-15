use std::{collections::HashMap, iter};

use itertools::Itertools;
use num::ToPrimitive;

advent_of_code::solution!(11);

fn count_digit(val: u64) -> u32 {
    for i in 1..64 {
        if val < 10u64.pow(i) {
            return i;
        }
    }
    unreachable!()
}

fn apply_rules(val: u64) -> impl IntoIterator<Item = u64> {
    if val == 0 {
        return vec![1].into_iter();
    }
    let digit_count = count_digit(val);
    if digit_count % 2 == 0 {
        let mask = 10u64.pow(digit_count / 2);
        return vec![val / mask, val % mask].into_iter();
    }
    vec![val * 2024].into_iter()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();
    // dbg!(&stones);
    for _ in 0..25 {
        stones = stones.into_iter().flat_map(apply_rules).collect_vec();
        // dbg!(&stones);
    }
    stones.len().to_u64()
}

fn helper(depth: usize, val: u64, dp: &mut HashMap<(usize, u64), u64>) -> u64 {
    if depth == 0 {
        return 1;
    }
    if let Some(result) = dp.get(&(depth, val)) {
        return result.clone();
    }
    if val == 0 {
        let result = helper(depth - 1, 1, dp);
        dp.entry((depth, val)).or_insert(result);
        return result;
    }
    let digit_count = count_digit(val);
    if digit_count % 2 == 0 {
        let mask = 10u64.pow(digit_count / 2);
        let result = [val / mask, val % mask]
            .into_iter()
            .map(|next| helper(depth - 1, next, dp))
            .sum();
        dp.entry((depth, val)).or_insert(result);
        return result;
    }
    let result = helper(depth - 1, val * 2024, dp);
    dp.entry((depth, val)).or_insert(result);
    return result;
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();
    // dbg!(&stones);
    let mut dp = HashMap::new();
    let count = stones
        .into_iter()
        .map(|stone| helper(75, stone, &mut dp))
        .sum();
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
