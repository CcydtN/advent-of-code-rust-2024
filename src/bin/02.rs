use itertools::Itertools;
use num::ToPrimitive;

advent_of_code::solution!(2);

fn part_one_report_check(report: &[i64]) -> bool {
    let check = if report[0] > report[1] {
        |a, b| a - b >= 1 && a - b <= 3
    } else {
        |a, b| b - a >= 1 && b - a <= 3
    };
    report.windows(2).all(|r| check(r[0], r[1]))
}

pub fn part_one(input: &str) -> Option<u64> {
    // dbg!(input);
    let reports = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|level| level.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    reports
        .into_iter()
        .filter(|report| part_one_report_check(&report))
        .count()
        .to_u64()
}

fn part_two_report_check(report: &[i64]) -> bool {
    // Brute force
    for i in 0..report.len() {
        let mut clone = report.to_vec();
        clone.remove(i);
        if part_one_report_check(&clone) {
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let reports = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|level| level.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    reports
        .into_iter()
        .filter(|report| part_two_report_check(&report))
        .count()
        .to_u64()
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
