use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use itertools::Itertools;
use nalgebra::distance;
use num::ToPrimitive;

advent_of_code::solution!(18);

fn get_grid_size() -> (usize, usize) {
    if cfg!(test) {
        (7, 7)
    } else {
        (71, 71)
    }
}

fn get_take_amount() -> usize {
    if cfg!(test) {
        12
    } else {
        1024
    }
}

fn parse_line(line: &str) -> Option<(usize, usize)> {
    let line = line.trim();
    let values: [usize; 2] = line
        .split(",")
        .map(|item| item.parse().unwrap())
        .collect_vec()
        .try_into()
        .ok()?;
    return Some(values.into());
}

fn parse_input(input: &str) -> Option<Vec<(usize, usize)>> {
    input.lines().map(parse_line).collect()
}

fn get_successors(pos: &(usize, usize)) -> [(usize, usize); 4] {
    [
        (pos.0 + 1, pos.1),
        (pos.0.wrapping_sub(1), pos.1),
        (pos.0, pos.1 + 1),
        (pos.0, pos.1.wrapping_sub(1)),
    ]
}

// 314 to high
pub fn part_one(input: &str) -> Option<u64> {
    let positions = parse_input(input)?;
    dbg!(positions.len());
    let grid_size = get_grid_size();
    let corrupted = positions
        .into_iter()
        .take(get_take_amount())
        .collect::<HashSet<_>>();

    let start = (0, 0);
    let end = (grid_size.0 - 1, grid_size.1 - 1);
    let total_step = 0;

    let mut prioity_queue = BinaryHeap::new();
    prioity_queue.push((Reverse(total_step), vec![start]));
    let mut visited = vec![vec![usize::MAX; grid_size.1]; grid_size.0];
    visited[0][0] = total_step;

    while let Some((Reverse(total_step), path)) = prioity_queue.pop() {
        let current = path.last().unwrap();
        // dbg!(current);
        if current == &end {
            debug(grid_size, &path, &corrupted);
            // return None;
            return (path.len() - 1).to_u64();
        }
        if visited[current.0][current.1] != total_step {
            continue;
        }
        for next in get_successors(current) {
            if next.0 >= grid_size.0 || next.1 >= grid_size.1 || corrupted.contains(&next) {
                continue;
            }
            let next_total_step = total_step + 1;
            if next_total_step < visited[next.0][next.1] {
                visited[next.0][next.1] = next_total_step;
                let mut next_path = path.clone();
                next_path.push(next);
                prioity_queue.push((Reverse(next_total_step), next_path));
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn debug(
    grid_size: (usize, usize),
    path: &Vec<(usize, usize)>,
    corrupted: &HashSet<(usize, usize)>,
) {
    println!("---");
    let mut grid = vec![vec!['.'; grid_size.1]; grid_size.0];
    for &(i, j) in corrupted {
        grid[j][i] = '#';
    }
    for &(i, j) in path {
        grid[j][i] = 'O';
    }
    for row in grid {
        for cell in row {
            print!("{cell}");
        }
        println!("");
    }
    println!("---");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
