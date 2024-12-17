use std::{
    collections::{HashSet, VecDeque},
    iter,
};

use itertools::Itertools;
use num::ToPrimitive;

advent_of_code::solution!(15);

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Robot,
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
                    // Cell::Empty
                    Cell::Robot
                }
                _ => unreachable!(),
            };
        }
        Self::new(robot.unwrap(), map)
    }
    fn parse_wide_grid(input: &str) -> Self {
        let m = input.lines().count();
        let n = input.lines().next().unwrap().len() * 2;
        // dbg!(m, n);
        let mut map = vec![vec![Cell::Empty; n]; m];
        let mut robot = None;
        for (i, (j, cell)) in input
            .lines()
            .enumerate()
            .flat_map(|(i, row)| iter::repeat(i).zip(row.trim().chars().enumerate()))
        {
            match cell {
                '#' => {
                    map[i][2 * j] = Cell::Wall;
                    map[i][2 * j + 1] = Cell::Wall;
                }
                'O' => {
                    map[i][2 * j] = Cell::BoxLeft;
                    map[i][2 * j + 1] = Cell::BoxRight;
                }
                '.' => {
                    map[i][2 * j] = Cell::Empty;
                    map[i][2 * j + 1] = Cell::Empty;
                }
                '@' => {
                    let _ = robot.insert((i, 2 * j));
                    map[i][2 * j] = Cell::Robot;
                }
                _ => unreachable!(),
            };
        }
        Self::new(robot.unwrap(), map)
    }

    // In wide case, always return BoxLeft position
    fn normalize(&self, pos: (usize, usize)) -> (usize, usize) {
        match self.map[pos.0][pos.1] {
            Cell::Empty | Cell::Wall | Cell::Box | Cell::BoxLeft => pos,
            Cell::BoxRight => (pos.0, pos.1 - 1),
            Cell::Robot => unreachable!(),
        }
    }

    fn get_movable(&self, current: (usize, usize), dir: &Dir) -> Option<Vec<(usize, usize)>> {
        assert_eq!(self.map[current.0][current.1], Cell::Robot);
        let mut movable = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(current);
        let mut visited = HashSet::new();

        while let Some(pos) = queue.pop_front() {
            match self.map[pos.0][pos.1] {
                Cell::Empty => {}
                Cell::Wall => return None,
                Cell::Box | Cell::Robot => {
                    let next = self.normalize(dir.step(pos));
                    queue.push_back(next);
                    movable.push(pos)
                }
                Cell::BoxLeft => {
                    match dir {
                        Dir::Up | Dir::Down => {
                            for item in vec![pos, (pos.0, pos.1 + 1)] {
                                let next = self.normalize(dir.step(item));
                                if visited.insert(next) {
                                    queue.push_back(next);
                                }
                            }
                        }
                        Dir::Left => {
                            let next = self.normalize(dir.step(pos));
                            queue.push_back(next);
                        }
                        Dir::Right => {
                            let next = self.normalize(dir.step(dir.step(pos)));
                            queue.push_back(next);
                        }
                    }
                    movable.push(pos)
                }
                Cell::BoxRight => unreachable!(),
            }
        }
        Some(movable)
    }

    fn try_move(&mut self, dir: Dir) {
        if let Some(movable) = self.get_movable(self.robot, &dir) {
            // dbg!(&movable, &dir);
            for pos in movable.into_iter().rev() {
                match self.map[pos.0][pos.1] {
                    Cell::Empty | Cell::Wall | Cell::BoxRight => {
                        dbg!(pos);
                        dbg!(&self.map[pos.0][pos.1]);
                        unreachable!()
                    }
                    Cell::Box | Cell::Robot => {
                        let next_pos = dir.step(pos);
                        self.map[next_pos.0][next_pos.1] = self.map[pos.0][pos.1].clone();
                        self.map[pos.0][pos.1] = Cell::Empty;
                    }
                    Cell::BoxLeft => {
                        let left = pos;
                        let right = (pos.0, pos.1 + 1);
                        let next_left = dir.step(left);
                        let next_right = dir.step(right);
                        self.map[next_left.0][next_left.1] = Cell::BoxLeft;
                        self.map[next_right.0][next_right.1] = Cell::BoxRight;
                        match dir {
                            Dir::Up | Dir::Down => {
                                self.map[left.0][left.1] = Cell::Empty;
                                self.map[right.0][right.1] = Cell::Empty;
                            }
                            Dir::Left => {
                                self.map[right.0][right.1] = Cell::Empty;
                            }
                            Dir::Right => {
                                self.map[left.0][left.1] = Cell::Empty;
                            }
                        }
                    }
                };
            }
            self.robot = dir.step(self.robot);
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
            Cell::Empty | Cell::Wall | Cell::Robot => {}
            Cell::Box => {
                sum += i * 100 + j;
            }
            Cell::BoxLeft => unreachable!(),
            Cell::BoxRight => unreachable!(),
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
    // dbg!(input);
    let (grid_input, moves_input) = input.split("\n\n").collect_tuple()?;
    let moves = parse_moves(moves_input);
    // dbg!(&moves);
    let mut helper = Helper::parse_wide_grid(grid_input);
    for dir in moves {
        helper.try_move(dir);
        // dbg!(&helper);
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
            Cell::Empty | Cell::Wall | Cell::Robot | Cell::BoxRight => {}
            Cell::BoxLeft => {
                sum += i * 100 + j;
            }
            Cell::Box => unreachable!(),
        }
    }
    sum.to_u64()
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
