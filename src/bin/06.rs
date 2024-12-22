use std::{collections::HashSet, marker::PhantomData};

use itertools::Itertools;
use num::ToPrimitive;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Eq, PartialEq, Default)]
enum Cell {
    #[default]
    Empty,
    Visited([bool; 4]),
    Obstacle,
}

const UP: (usize, usize) = (usize::MAX, 0);
const DOWN: (usize, usize) = (1, 0);
const LEFT: (usize, usize) = (0, usize::MAX);
const RIGHT: (usize, usize) = (0, 1);

fn dir2idx(dir: &(usize, usize)) -> usize {
    [UP, DOWN, LEFT, RIGHT]
        .iter()
        .position(|x| x == dir)
        .unwrap()
}

#[derive(Debug, Clone, Default)]
struct Helper {
    guard: (usize, usize),
    dir: (usize, usize),
    map: Vec<Vec<Cell>>,
    has_loop: bool,
}

impl Helper {
    fn rotate(&self) -> (usize, usize) {
        match self.dir {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
            (_, _) => unreachable!(),
        }
    }

    fn next_step(&self) -> (usize, usize) {
        (
            self.guard.0.wrapping_add(self.dir.0),
            self.guard.1.wrapping_add(self.dir.1),
        )
    }

    fn new(input: &str) -> Self {
        let mut guard = None;
        let dir = (usize::MAX, 0);
        let rows = input.lines().collect_vec();
        assert!(rows.len() < usize::MAX && rows[0].len() < usize::MAX);
        let mut map = vec![vec![Cell::Empty; rows[0].len()]; rows.len()];
        for (i, row) in rows.into_iter().enumerate() {
            for (j, cell) in row.chars().enumerate() {
                match cell {
                    '.' => (),
                    '#' => map[i][j] = Cell::Obstacle,
                    '^' => {
                        map[i][j] = Cell::Visited([true, false, false, false]);
                        let _ = guard.insert((i, j));
                    }
                    _ => unreachable!(),
                }
            }
        }
        Self {
            guard: guard.unwrap(),
            dir,
            map,
            ..Default::default()
        }
    }

    fn update(&mut self) -> bool {
        let next = self.next_step();
        if next.0 >= self.map.len() || next.1 >= self.map[0].len() {
            return false;
        }
        match self.map[next.0][next.1] {
            Cell::Obstacle => {
                self.dir = self.rotate();
            }
            Cell::Empty => {
                let mut visited = [false; 4];
                visited[dir2idx(&self.dir)] = true;
                self.map[next.0][next.1] = Cell::Visited(visited);
                self.guard = next;
            }
            Cell::Visited(ref mut visited) => {
                if visited[dir2idx(&self.dir)] {
                    self.has_loop = true;
                    return false;
                }
                visited[dir2idx(&self.dir)] = true;
                self.guard = next;
            }
        }
        true
    }

    fn count(self) -> usize {
        self.map
            .into_iter()
            .flatten()
            .filter(|cell| match cell {
                Cell::Visited(_) => true,
                _ => false,
            })
            .count()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    // dbg!(input);
    let mut helper = Helper::new(input);
    while helper.update() {}
    helper.count().to_u64()
}

// Brute force
pub fn part_two(input: &str) -> Option<u64> {
    let mut helper = Helper::new(input);
    while helper.update() {}

    let mut count = 0;
    for (i, row) in helper.map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            match cell {
                Cell::Visited(_) => {}
                _ => continue,
            }
            let mut tmp = Helper::new(input);
            tmp.map[i][j] = Cell::Obstacle;
            while tmp.update() {}
            if tmp.has_loop == true {
                count += 1;
            }
        }
    }
    count.to_u64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
