use itertools::{iproduct, Itertools};
use num::ToPrimitive;
use std::collections::{BTreeMap, BTreeSet, HashSet};

advent_of_code::solution!(12);

fn get_neighbor((i, j): (usize, usize), grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut result = vec![];
    if i != 0 {
        result.push((i - 1, j));
    }
    if i != grid.len() - 1 {
        result.push((i + 1, j));
    }
    if j != 0 {
        result.push((i, j - 1));
    }
    if j != grid[0].len() - 1 {
        result.push((i, j + 1));
    }
    result
}

fn dfs(root: (usize, usize), grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut stack = vec![root];
    let mut visited = HashSet::new();
    visited.insert(root);
    while let Some((i, j)) = stack.pop() {
        let neighbors = get_neighbor((i, j), grid);
        for (x, y) in neighbors {
            if grid[x][y] == grid[i][j] {
                if visited.insert((x, y)) {
                    stack.push((x, y));
                }
            }
        }
    }
    visited
}

fn count_edge(regions: &HashSet<(usize, usize)>) -> u64 {
    let mut rows = BTreeMap::new();
    let mut columns = BTreeMap::new();
    for (i, j) in regions {
        rows.entry(i).or_insert(BTreeSet::new()).insert(j);
        columns.entry(j).or_insert(BTreeSet::new()).insert(i);
    }
    let mut count = 0;
    for tmp in [rows, columns] {
        count += tmp.first_key_value().unwrap().1.len();
        count += tmp.last_key_value().unwrap().1.len();
        for (a, b) in tmp.into_values().tuple_windows() {
            count += a.symmetric_difference(&b).count()
        }
    }
    count.to_u64().unwrap()
}

fn count_group<'a>(iter: impl IntoIterator<Item = &'a usize>) -> usize {
    let value = iter.into_iter().collect_vec();
    if value.len() == 0 {
        return 0;
    }
    value
        .into_iter()
        .tuple_windows::<(_, _)>()
        .fold(1, |acc, (a, b)| acc + (b - a != 1) as usize)
}

fn count_edge_group(regions: &HashSet<(usize, usize)>) -> u64 {
    let mut rows = BTreeMap::new();
    let mut columns = BTreeMap::new();
    for &(i, j) in regions {
        rows.entry(i).or_insert(BTreeSet::new()).insert(j);
        columns.entry(j).or_insert(BTreeSet::new()).insert(i);
    }
    // dbg!(&rows, &columns);
    let mut count = 0;
    for tmp in [rows, columns] {
        count += count_group(tmp.first_key_value().unwrap().1);
        count += count_group(tmp.last_key_value().unwrap().1);
        for (a, b) in tmp.into_values().tuple_windows() {
            count += count_group(a.difference(&b));
            count += count_group(b.difference(&a));
        }
    }
    count.to_u64().unwrap()
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
        let region = dfs((i, j), &grid);
        let area = region.len() as u64;
        let edge = count_edge(&region);
        dbg!(grid[i][j], area, edge, area * edge);
        sum += area * edge;
        for (x, y) in region {
            grid[x][y] = '.';
        }
    }
    Some(sum)
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
        let region = dfs((i, j), &grid);
        let area = region.len() as u64;
        let edge = count_edge_group(&region);
        dbg!(grid[i][j], area, edge, area * edge);
        sum += area * edge;
        for (x, y) in region {
            grid[x][y] = '.';
        }
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
