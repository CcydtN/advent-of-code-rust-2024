use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use itertools::{iproduct, Itertools};

advent_of_code::solution!(12);

pub fn part_one_dfs(root: (usize, usize), grid: &mut Vec<Vec<char>>) -> (u64, u64) {
    let mut stack = vec![root];
    let mut visited = HashSet::new();
    visited.insert(root);
    let mut edge = HashSet::new();
    while let Some((i, j)) = stack.pop() {
        for offset in [1, usize::MAX] {
            {
                let x = i.wrapping_add(offset);
                if x < grid.len() && grid[x][j] == grid[i][j] {
                    edge.insert(((i, j), (x, j)));
                    if visited.insert((x, j)) {
                        stack.push((x, j));
                    }
                }
            }
            {
                let y = j.wrapping_add(offset);
                if y < grid.len() && grid[i][y] == grid[i][j] {
                    edge.insert(((i, j), (i, y)));
                    if visited.insert((i, y)) {
                        stack.push((i, y));
                    }
                }
            }
        }
    }
    let area = visited.len() as u64;
    for (i, j) in visited {
        grid[i][j] = '.'
    }
    (area, area * 4 - edge.len() as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let m = grid.len();
    let n = grid[0].len();
    let mut sum = 0;
    for (i, j) in iproduct!(0..m, 0..n) {
        if grid[i][j] == '.' {
            continue;
        }
        let dbg_info = grid[i][j];
        let (area, edge) = part_one_dfs((i, j), &mut grid);
        // dbg!(dbg_info, area, edge, area * edge);
        sum += area * edge;
    }
    Some(sum)
}

pub fn part_two_dfs(root: (usize, usize), grid: &mut Vec<Vec<char>>) -> (u64, u64) {
    let mut stack = vec![root];
    let mut visited = HashSet::new();
    visited.insert(root);
    let mut vertical_edge = BTreeMap::new();
    let mut horizontal_edge = BTreeMap::new();
    while let Some((i, j)) = stack.pop() {
        for offset in [1, usize::MAX] {
            {
                let x = i.wrapping_add(offset);
                if x < grid.len() && grid[x][j] == grid[i][j] {
                    if visited.insert((x, j)) {
                        stack.push((x, j));
                    }
                }
            }
            {
                let y = j.wrapping_add(offset);
                if y < grid.len() && grid[i][y] == grid[i][j] {
                    if visited.insert((i, y)) {
                        stack.push((i, y));
                    }
                }
            }
        }
        horizontal_edge
            .entry(j)
            .or_insert(BTreeSet::new())
            .insert(i);
        vertical_edge.entry(i).or_insert(BTreeSet::new()).insert(j);
    }
    let area = visited.len() as u64;
    let mut edge = 0;
    let mut last = BTreeSet::new();

    // Should be cleaner if we use rangeset, just count the group of difference.
    for item in horizontal_edge.into_values().chain([BTreeSet::new()]) {
        let mut last_val = usize::MAX;
        for val in last.difference(&item) {
            edge += (&last_val != val) as u64;
            last_val = val.to_owned() + 1;
        }
        let mut last_val = usize::MAX;
        for val in item.difference(&last) {
            edge += (&last_val != val) as u64;
            last_val = val.to_owned() + 1;
        }
        last = item;
    }
    let mut last = BTreeSet::new();
    for item in vertical_edge.into_values().chain([BTreeSet::new()]) {
        let mut last_val = usize::MAX;
        for val in last.difference(&item) {
            edge += (&last_val != val) as u64;
            last_val = val.to_owned() + 1;
        }
        let mut last_val = usize::MAX;
        for val in item.difference(&last) {
            edge += (&last_val != val) as u64;
            last_val = val.to_owned() + 1;
        }
        last = item;
    }

    for (i, j) in visited {
        grid[i][j] = '.'
    }
    (area, edge as u64)
}

// 907046 X
pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let m = grid.len();
    let n = grid[0].len();
    let mut sum = 0;
    for (i, j) in iproduct!(0..m, 0..n) {
        if grid[i][j] == '.' {
            continue;
        }
        let dbg_info = grid[i][j];
        let (area, edge) = part_two_dfs((i, j), &mut grid);
        // dbg!(dbg_info, area, edge, area * edge);
        sum += area * edge;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 0,
        ));
        assert_eq!(result, Some(1930));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 0,
        ));
        assert_eq!(result, Some(1206));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(436));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(236));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(368));
    }
}
