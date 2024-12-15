use std::collections::HashSet;

use itertools::{iproduct, Itertools};
use num::ToPrimitive;

advent_of_code::solution!(10);

pub fn parse_grid(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|item| item.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    // dbg!(input);
    let grid = parse_grid(input);
    let m = grid.len();
    let n = grid[0].len();

    let mut count = 0;
    for (i, j) in iproduct!(0..m, 0..n) {
        if grid[i][j] != 0 {
            continue;
        }
        let mut stack = vec![(i, j)];
        let mut visited = HashSet::new();
        visited.insert((i, j));
        while let Some((i, j)) = stack.pop() {
            if grid[i][j] == 9 {
                count += 1;
            }
            for offset in [1, usize::MAX] {
                let x = i.wrapping_add(offset);
                if x < m && grid[x][j] == grid[i][j] + 1 {
                    if visited.insert((x, j)) {
                        stack.push((x, j));
                    }
                }
                let y = j.wrapping_add(offset);
                if y < n && grid[i][y] == grid[i][j] + 1 {
                    if visited.insert((i, y)) {
                        stack.push((i, y));
                    }
                }
            }
        }
    }
    count.to_u64()
}

pub fn part_two_helper(
    target: usize,
    i: usize,
    j: usize,
    grid: &Vec<Vec<usize>>,
    dp: &mut Vec<Vec<Option<u64>>>,
) -> Option<u64> {
    if i >= grid.len() || j >= grid[0].len() || target != grid[i][j] {
        return None;
    }
    if dp[i][j].is_some() {
        return dp[i][j];
    }
    let mut sum = 0;
    if target == grid[i][j] {
        sum += part_two_helper(target + 1, i.wrapping_add(1), j, grid, dp).unwrap_or(0);
        sum += part_two_helper(target + 1, i.wrapping_sub(1), j, grid, dp).unwrap_or(0);
        sum += part_two_helper(target + 1, i, j.wrapping_add(1), grid, dp).unwrap_or(0);
        sum += part_two_helper(target + 1, i, j.wrapping_sub(1), grid, dp).unwrap_or(0);
    }
    let _ = dp[i][j].insert(sum);
    return dp[i][j];
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_grid(input);
    let m = grid.len();
    let n = grid[0].len();
    let mut dp: Vec<Vec<Option<u64>>> = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|val| if val == &9 { Some(1) } else { None })
                .collect_vec()
        })
        .collect_vec();

    let mut count = 0;
    for (i, j) in iproduct!(0..m, 0..n) {
        count += part_two_helper(0, i, j, &grid, &mut dp).unwrap_or(0);
    }
    // dbg!(dp);
    count.to_u64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
