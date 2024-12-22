use itertools::Itertools;
use num::ToPrimitive;

advent_of_code::solution!(2);

fn report_check(report: &[i64]) -> bool {
    // First condition: Every element must be sorted in ascending or desending order and at least differ by 1
    (report.is_sorted_by(|a, b| a > b) || report.is_sorted_by(|a, b| a < b))
    // Second condition: Every element differ by at Most 3
        && report.windows(2).all(|r| r[0].abs_diff(r[1]) <= 3)
}

fn parse_reports(input: &str) -> Vec<Vec<i64>> {
    input
        .split_whitespace()
        .map(|data| data.parse().unwrap())
        .chunks(5)
        .into_iter()
        .map(|chunk| chunk.collect_vec())
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    // dbg!(input);
    let reports = parse_reports(input);

    reports
        .into_iter()
        .filter(|report| report_check(report.as_slice()))
        .count()
        .to_u64()
}

fn report_check_with_tolerance(report: &[i64]) -> bool {
    // Brute force
    for i in 0..report.len() {
        let mut clone = report.to_vec();
        clone.remove(i);
        if report_check(&clone) {
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let reports = parse_reports(input);

    reports
        .into_iter()
        .filter(|report| report_check_with_tolerance(&report))
        .count()
        .to_u64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
