use std::collections::HashSet;

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

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
