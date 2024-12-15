use itertools::Itertools;
use num::ToPrimitive;

advent_of_code::solution!(7);

fn part_one_check(target: u64, values: &[u64]) -> bool {
    let len = values.len();
    let tmp = 1 << len;
    // dbg!(len, tmp);
    for i in 0..tmp {
        let mut result = values[0];
        for (j, val) in values.iter().skip(1).enumerate() {
            if (i >> j) % 2 == 1 {
                result += val;
            } else {
                result *= val;
            }
        }
        if result == target {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    // dbg!(input);
    let mut sum = 0;
    for line in input.lines() {
        let (target, values) = line.split_at(line.find(":").unwrap());
        let target = target.parse::<u64>().unwrap();
        let values = values[1..]
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect_vec();
        // dbg!(&target, &values);
        if part_one_check(target, &values) {
            sum += target;
        }
    }
    sum.to_u64()
}

fn min_10_pow(val: &u64) -> u64 {
    let mut tmp = 1;
    while &tmp <= val {
        tmp *= 10;
    }
    tmp
}

fn part_two_check(target: u64, values: &[u64]) -> bool {
    let len = values.len();
    let tmp = 3usize.pow(len.to_u32().unwrap());
    // dbg!(len, tmp);
    for mut i in 0..tmp {
        let mut result = values[0];
        for val in values.iter().skip(1) {
            match i % 3 {
                0 => result += val,
                1 => result *= val,
                2 => result = result * min_10_pow(val) + val,
                _ => unreachable!(),
            }
            i /= 3;
        }
        if result == target {
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let (target, values) = line.split_at(line.find(":").unwrap());
        let target = target.parse::<u64>().unwrap();
        let values = values[1..]
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect_vec();
        // dbg!(&target, &values);
        if part_two_check(target, &values) {
            sum += target;
        }
    }
    sum.to_u64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
