use std::collections::HashMap;

use itertools::Itertools;
use num::ToPrimitive;

advent_of_code::solution!(19);

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut iter = input.lines();
    let building_blocks = iter.next().unwrap().split(", ").collect_vec();
    let _ = iter.next();
    let targets = iter.collect_vec();
    (building_blocks, targets)
}

// Ugly approach, but it works
fn try_build_target<'a>(
    building_blocks: &[&'a str],
    target: &'a str,
    dp: &mut HashMap<&'a str, u64>,
) -> u64 {
    if dp.contains_key(target) {
        return *dp.get(target).unwrap();
    }
    let mut count = 0;
    for block in building_blocks {
        if target.starts_with(block) {
            count += try_build_target(building_blocks, &target[block.len()..], dp)
        }
    }
    dp.insert(target, count);
    return count;
}

pub fn part_one(input: &str) -> Option<u64> {
    let (building_blocks, targets) = parse_input(input);
    // dbg!(&building_blocks, &targets);
    let mut dp = HashMap::new();
    dp.insert("", 1u64);
    targets
        .into_iter()
        .map(|target| try_build_target(&building_blocks, target, &mut dp))
        .filter(|val| val != &0)
        .count()
        .to_u64()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (building_blocks, targets) = parse_input(input);
    // dbg!(&building_blocks, &targets);
    let mut dp = HashMap::new();
    dp.insert("", 1u64);
    targets
        .into_iter()
        .map(|target| try_build_target(&building_blocks, target, &mut dp))
        .sum::<u64>()
        .to_u64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
