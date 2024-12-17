use std::iter;

use itertools::Itertools;
use num::ToPrimitive;

advent_of_code::solution!(15);

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn step(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Dir::Up => (x - 1, y),
            Dir::Down => (x + 1, y),
            Dir::Left => (x, y - 1),
            Dir::Right => (x, y + 1),
        }
    }
    fn step_back(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Dir::Up => (x + 1, y),
            Dir::Down => (x - 1, y),
            Dir::Left => (x, y + 1),
            Dir::Right => (x, y - 1),
        }
    }
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            c => {
                dbg!(c);
                unreachable!()
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Cell {
    Empty,
    Wall,
    Box,
}

#[derive(Debug)]
struct Helper {
    robot: (usize, usize),
    map: Vec<Vec<Cell>>,
}

impl Helper {
    fn new(robot: (usize, usize), map: Vec<Vec<Cell>>) -> Self {
        Self { robot, map }
    }

    fn parse_grid(input: &str) -> Self {
        let m = input.lines().count();
        let n = input.lines().next().unwrap().len();
        // dbg!(m, n);
        let mut map = vec![vec![Cell::Empty; n]; m];
        let mut robot = None;
        for (i, (j, cell)) in input
            .lines()
            .enumerate()
            .flat_map(|(i, row)| iter::repeat(i).zip(row.trim().chars().enumerate()))
        {
            map[i][j] = match cell {
                '#' => Cell::Wall,
                'O' => Cell::Box,
                '.' => Cell::Empty,
                '@' => {
                    let _ = robot.insert((i, j));
                    Cell::Empty
                }
                _ => unreachable!(),
            };
        }
        Self::new(robot.unwrap(), map)
    }

    fn try_move_to(&self, pos: (usize, usize), dir: Dir) -> Option<(usize, usize)> {
        match self.map[pos.0][pos.1] {
            Cell::Empty => Some(pos),
            Cell::Wall => None,
            Cell::Box => self.try_move_to(dir.step(pos), dir),
        }
    }

    fn try_move(&mut self, dir: Dir) {
        let target = dir.step(self.robot);
        match self.map[target.0][target.1] {
            Cell::Empty => self.robot = target,
            Cell::Wall => (),
            Cell::Box => {
                if let Some(location) = self.try_move_to(dir.step(target), dir) {
                    self.map[location.0][location.1] = Cell::Box;
                    self.map[target.0][target.1] = Cell::Empty;
                    self.robot = target;
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    // dbg!(input);
    let (grid_input, moves_input) = input.split("\n\n").collect_tuple()?;
    let moves = parse_moves(moves_input);
    // dbg!(&moves);
    let mut helper = Helper::parse_grid(grid_input);
    for dir in moves {
        helper.try_move(dir);
    }
    // dbg!(&helper);
    let mut sum = 0;
    for (i, (j, cell)) in helper
        .map
        .into_iter()
        .enumerate()
        .skip(1)
        .flat_map(|(i, row)| iter::repeat(i).zip(row.into_iter().enumerate().skip(1)))
    {
        match cell {
            Cell::Empty => {}
            Cell::Wall => {}
            Cell::Box => {
                sum += i * 100 + j;
            }
        }
    }
    sum.to_u64()
}

fn parse_moves(moves_input: &str) -> Vec<Dir> {
    let moves = moves_input
        .chars()
        .filter(|c| c != &'\n')
        .map(Dir::from)
        .collect_vec();
    moves
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2028));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9021));
    }
}
