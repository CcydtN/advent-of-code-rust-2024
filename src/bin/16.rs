use itertools::iproduct;
use num::{traits::WrappingSub, ToPrimitive};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

advent_of_code::solution!(16);

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Visited(usize),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Dir {
    North,
    East,
    South,
    West,
}

const DIRECTIONS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];

impl Dir {
    fn move_from(&self, pos: &(usize, usize)) -> (usize, usize) {
        let mut pos = pos.clone();
        match self {
            Dir::North => pos.0 -= 1,
            Dir::East => pos.1 += 1,
            Dir::South => pos.0 += 1,
            Dir::West => pos.1 -= 1,
        };
        pos
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Reindeer {
    pos: Vec<(usize, usize)>,
    facing: Dir,
    rotate: usize,
}

impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.score().cmp(&other.score()))
    }
}

impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.score()).cmp(&other.score())
    }
}

impl Reindeer {
    fn new(pos: (usize, usize), facing: Dir, rotate: usize) -> Self {
        Self {
            pos: vec![pos],
            facing,
            rotate,
        }
    }

    fn init(pos: (usize, usize)) -> Self {
        Self::new(pos, Dir::East, 0)
    }

    fn move_next(self) -> [Self; 4] {
        let mut next = std::array::from_fn(|_| self.clone());
        for (item, dir) in next.iter_mut().zip(DIRECTIONS) {
            item.pos.push(dir.move_from(item.position()));
            item.rotate += if dir == item.facing { 0 } else { 1 };
            item.facing = dir;
        }
        next
    }

    #[inline]
    fn score(&self) -> usize {
        self.rotate * 1000 + (self.pos.len() - 1)
    }

    #[inline]
    fn position(&self) -> &(usize, usize) {
        self.pos.last().unwrap()
    }
}

fn parse_input(input: &str) -> (Vec<Vec<Cell>>, (usize, usize), (usize, usize)) {
    let m = input.lines().count();
    let n = input.lines().next().unwrap().len();
    let mut grid = vec![];
    grid.reserve(m);
    let mut start = None;
    let mut end = None;
    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        row.reserve(n);
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => row.push(Cell::Wall),
                '.' => row.push(Cell::Empty),
                'S' => {
                    let _ = start.insert((i, j));
                    row.push(Cell::Empty)
                }
                'E' => {
                    let _ = end.insert((i, j));
                    row.push(Cell::Empty)
                }
                _ => unreachable!(),
            }
        }
        grid.push(row);
    }
    (grid, start.unwrap(), end.unwrap())
}

pub fn part_one(input: &str) -> Option<u64> {
    // dbg!(input);
    let (mut grid, start, end) = parse_input(input);
    // dbg!(start, end);
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse(Reindeer::init(start)));
    grid[start.0][start.1] = Cell::Visited(0);

    while let Some(Reverse(reindeer)) = priority_queue.pop() {
        let current_pos = reindeer.position();
        if current_pos == &end {
            return reindeer.score().to_u64();
        }
        match grid[current_pos.0][current_pos.1] {
            Cell::Visited(score) if score != reindeer.score() => continue,
            Cell::Visited(_) => {}
            _ => unreachable!(),
        }
        for next in reindeer.move_next() {
            let next_pos = next.position();
            match grid[next_pos.0][next_pos.1] {
                Cell::Empty => {
                    grid[next_pos.0][next_pos.1] = Cell::Visited(next.score());
                    priority_queue.push(Reverse(next));
                }
                Cell::Visited(ref mut score) if *score > next.score() => {
                    *score = next.score();
                    priority_queue.push(Reverse(next));
                }
                Cell::Wall | Cell::Visited(_) => {}
            }
        }
    }
    unreachable!()
}

// 475 too low
pub fn part_two(input: &str) -> Option<u64> {
    let (mut grid, start, end) = parse_input(input);
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse(Reindeer::init(start)));
    grid[start.0][start.1] = Cell::Visited(0);
    let mut min_score = None;
    let mut nice_tile = HashSet::new();

    while let Some(Reverse(reindeer)) = priority_queue.pop() {
        let current_pos = reindeer.position();
        if min_score.is_some() && reindeer.score() > min_score.unwrap() {
            return nice_tile.len().to_u64();
        }
        if current_pos == &end {
            let _ = min_score.insert(reindeer.score());
            reindeer.pos.into_iter().for_each(|val| {
                nice_tile.insert(val);
            });
            continue;
        }
        for next in reindeer.move_next() {
            let next_pos = next.position();
            match grid[next_pos.0][next_pos.1] {
                Cell::Empty => {
                    grid[next_pos.0][next_pos.1] = Cell::Visited(next.pos.len());
                    priority_queue.push(Reverse(next));
                }
                Cell::Visited(ref mut step) if *step >= next.pos.len() => {
                    *step = next.pos.len();
                    priority_queue.push(Reverse(next));
                }
                Cell::Wall | Cell::Visited(_) => {}
            }
        }
    }
    nice_tile.len().to_u64()
}

fn debug(grid: &[Vec<Cell>]) {
    for row in grid {
        for cell in row {
            let s = match cell {
                Cell::Empty => ".".to_owned(),
                Cell::Wall => "#".to_owned(),
                Cell::Visited(score) => score.to_string(),
            };
            print!("{:7}", s);
        }
        println!("");
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(7036));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(45));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
