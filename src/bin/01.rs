advent_of_code::solution!(1);
use itertools::Itertools;
use num::ToPrimitive;

fn parse_input(input: &str) -> [Vec<usize>; 2] {
    let mut lists = [vec![], vec![]];
    for (i, item) in input.split_whitespace().enumerate() {
        lists[i % 2].push(item.parse().unwrap());
    }
    lists
}

pub fn part_one(input: &str) -> Option<u64> {
    // dbg!(lists);
    let [mut left, mut right] = parse_input(input);
    left.sort();
    right.sort();
    let result = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum::<usize>();
    result.to_u64()
}

pub fn part_two(input: &str) -> Option<u64> {
    // dbg!(lists);
    let [left, right] = parse_input(input);
    let freq = right.into_iter().counts();
    let result = left
        .into_iter()
        .map(|val| val * freq.get(&val).unwrap_or(&0))
        .sum::<usize>();
    result.to_u64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
