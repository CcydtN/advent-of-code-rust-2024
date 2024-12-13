advent_of_code::solution!(1);
use itertools::Itertools;
use num::ToPrimitive;

pub fn part_one(input: &str) -> Option<u64> {
    let mut lists = [vec![], vec![]];
    for (i, item) in input.split_whitespace().enumerate() {
        lists[i % 2].push(item.parse::<u64>().ok()?);
    }
    // dbg!(lists);
    let [mut left, mut right] = lists;
    left.sort();
    right.sort();
    let result = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(a, b)| if a > b { a - b } else { b - a })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lists = [vec![], vec![]];
    for (i, item) in input.split_whitespace().enumerate() {
        lists[i % 2].push(item.parse::<u64>().ok()?);
    }
    // dbg!(lists);
    let [left, right] = lists;
    let freq = right.into_iter().counts();
    let result = left
        .into_iter()
        .map(|val| val * freq.get(&val).unwrap_or(&0).to_u64().unwrap())
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
