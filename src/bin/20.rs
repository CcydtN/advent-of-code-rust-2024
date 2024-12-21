use std::collections::HashMap;

use itertools::Itertools;
use num::ToPrimitive;

advent_of_code::solution!(20);

fn get_part_1_lower_bound() -> usize {
    if cfg!(test) {
        10
    } else {
        100
    }
}

fn get_part_2_lower_bound() -> usize {
    if cfg!(test) {
        50
    } else {
        100
    }
}

fn get_successor(pos: &(usize, usize), step: usize) -> [(usize, usize); 4] {
    [
        (pos.0 + step, pos.1),
        (pos.0.wrapping_sub(step), pos.1),
        (pos.0, pos.1 + step),
        (pos.0, pos.1.wrapping_sub(step)),
    ]
}

fn find_path(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut path = vec![start];
    while let Some(current) = path.last() {
        if current == &end {
            break;
        }
        let len = path.len();
        for successor in get_successor(current, 1) {
            if successor.0 > grid.len()
                || successor.1 > grid[0].len()
                || Some(&successor) == path.get(len.wrapping_sub(2))
            {
                continue;
            }
            if grid[successor.0][successor.1] == '.' {
                path.push(successor);
                break;
            }
        }
    }
    path
}

pub fn part_one(input: &str) -> Option<u64> {
    let (grid, start, end, expected_len) = parse_input(input);
    dbg!(&grid.len(), &grid[0].len());
    dbg!(&start, &end);

    let path = find_path(&grid, start, end);
    assert_eq!(path.len(), expected_len);

    let cheats = find_cheats(&path, 2, &grid);
    // dbg!(&cheats
    //     .iter()
    //     .map(|(key, val)| (key, val.len()))
    //     .collect_vec());
    cheats
        .into_iter()
        .filter(|(time_saved, _)| time_saved >= &get_part_1_lower_bound())
        .map(|(_, val)| val.len())
        .sum::<usize>()
        .to_u64()
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize), (usize, usize), usize) {
    let mut grid = vec![];
    let mut start = None;
    let mut end = None;
    let mut expected_len = 0;

    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (j, ch) in line.chars().enumerate() {
            match ch {
                'S' => {
                    let _ = start.insert((i, j));
                    row.push('.');
                    expected_len += 1;
                }
                'E' => {
                    let _ = end.insert((i, j));
                    row.push('.');
                    expected_len += 1;
                }
                '#' => {
                    row.push(ch);
                }
                '.' => {
                    row.push(ch);
                    expected_len += 1;
                }
                _ => {}
            }
        }
        grid.push(row);
    }
    (grid, start.unwrap(), end.unwrap(), expected_len)
}

fn manhattan_distance(point_a: &(usize, usize), point_b: &(usize, usize)) -> usize {
    point_a.0.abs_diff(point_b.0) + point_a.1.abs_diff(point_b.1)
}

fn find_cheats(
    path: &[(usize, usize)],
    allowance: usize,
    grid: &Vec<Vec<char>>,
) -> HashMap<usize, Vec<((usize, usize), (usize, usize))>> {
    let mut cheats = HashMap::new();
    for (i, end_point) in path.iter().enumerate().skip(2) {
        for (j, start_point) in path[..i - 2].iter().enumerate() {
            let dist = manhattan_distance(start_point, end_point);
            if dist <= allowance {
                let entry = cheats.entry(i - j - 2).or_insert(vec![]);
                entry.push((*start_point, *end_point));
            }
        }
    }
    cheats
}

pub fn part_two(input: &str) -> Option<u64> {
    let (grid, start, end, expected_len) = parse_input(input);
    dbg!(&grid.len(), &grid[0].len());
    dbg!(&start, &end);

    let path = find_path(&grid, start, end);
    assert_eq!(path.len(), expected_len);

    let cheats = find_cheats(&path, 20, &grid);
    // dbg!(&cheats
    //     .iter()
    //     .map(|(key, val)| (key, val.len()))
    //     .collect_vec());
    cheats
        .into_iter()
        .filter(|(time_saved, _)| time_saved >= &get_part_2_lower_bound())
        .map(|(_, val)| val.len())
        .sum::<usize>()
        .to_u64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            result,
            Some(32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3)
        );
    }
}
