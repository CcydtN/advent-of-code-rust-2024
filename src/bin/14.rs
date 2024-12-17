advent_of_code::solution!(14);

use std::collections::HashSet;

use itertools::Itertools;
use nalgebra::{vector, Vector2, VectorView2};
use num::ToPrimitive;

#[derive(Debug)]
struct Robot {
    pos: Vector2<i64>,
    vel: Vector2<i64>,
}

impl Robot {
    fn new(pos: Vector2<i64>, vel: Vector2<i64>) -> Self {
        Self { pos, vel }
    }

    fn from_input(input: &str) -> Option<Self> {
        // dbg!(input);
        let mut iter = input.split_whitespace();
        let pos_input = iter.next()?;
        // dbg!(pos_input);
        assert!(pos_input.starts_with("p="));
        let pos: [i64; 2] = pos_input[2..]
            .split(',')
            .filter_map(|val| val.parse::<i64>().ok())
            .collect_vec()
            .try_into()
            .ok()?;
        // dbg!(pos);

        let vel_input = iter.next()?;
        assert!(vel_input.starts_with("v="));
        let vel: [i64; 2] = vel_input[2..]
            .split(',')
            .filter_map(|val| val.parse::<i64>().ok())
            .collect_vec()
            .try_into()
            .ok()?;
        // dbg!(vel);

        Some(Robot::new(Vector2::from(pos), Vector2::from(vel)))
    }

    fn predict(&self, time: i64, space: VectorView2<i64>) -> Vector2<i64> {
        let tmp = self.pos + (time * self.vel);
        let tmp: [i64; 2] = tmp
            .into_iter()
            .zip(space)
            .map(|(a, b)| ((a % b) + b) % b)
            .collect_vec()
            .try_into()
            .unwrap();
        tmp.into()
    }

    fn advance(&mut self, space: VectorView2<i64>) {
        self.pos += self.vel;
        self.pos.iter_mut().zip(space).for_each(|(a, b)| {
            *a %= b;
            *a += b;
            *a %= b;
        });
    }
}

fn parse_input(input: &str) -> Option<Vec<Robot>> {
    input.lines().map(|line| Robot::from_input(line)).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let robots = parse_input(input).unwrap();
    // dbg!(&robots);
    let space = get_space();
    // dbg!(space);
    let mut count = [0u64; 4];
    for robot in robots {
        let pos = robot.predict(100, space.as_view());
        // dbg!(pos);
        let x = pos.x;
        let y = pos.y;
        match (x, y) {
            (i, j) if i > space.x / 2 && j > space.y / 2 => {
                count[0] += 1;
            }
            (i, j) if i < space.x / 2 && j > space.y / 2 => {
                count[1] += 1;
            }
            (i, j) if i < space.x / 2 && j < space.y / 2 => {
                count[2] += 1;
            }
            (i, j) if i > space.x / 2 && j < space.y / 2 => {
                count[3] += 1;
            }
            (_, _) => {}
        };
    }
    // dbg!(count);
    Some(count.into_iter().product::<u64>())
}

pub fn part_two(input: &str) -> Option<u64> {
    // If it display a picture, none of them should stack on each other?
    let mut robots = parse_input(input).unwrap();
    let space = get_space();
    let mut count = 0usize;
    loop {
        robots
            .iter_mut()
            .for_each(|robot| robot.advance(space.as_view()));
        count += 1;

        let mut visited = HashSet::new();
        if robots.iter().all(|robot| visited.insert(robot.pos)) {
            break;
        }
    }
    // print the grid

    // let mut grid = vec![vec!['.'; space.y as usize]; space.x as usize];
    // for robot in robots {
    //     grid[robot.pos.x as usize][robot.pos.y as usize] = '#';
    // }
    // for row in grid {
    //     for cell in row {
    //         print!("{}", cell);
    //     }
    //     println!("");
    // }
    count.to_u64()
}

fn get_space() -> Vector2<i64> {
    if cfg!(test) {
        vector![11, 7]
    } else {
        vector![101, 103]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
