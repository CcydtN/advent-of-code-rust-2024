use std::collections::{HashMap, HashSet};
mod util;

use num::ToPrimitive;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    // dbg!(input);
    let mut map: HashMap<char, Vec<_>> = HashMap::new();
    let mut antinodes: HashSet<_> = HashSet::new();
    let grid_size = (input.lines().count(), input.lines().next().unwrap().len());
    for (i, line) in input.lines().enumerate() {
        for (j, cell) in line.chars().enumerate() {
            if cell == '.' {
                continue;
            }
            map.entry(cell)
                .or_default()
                .push(util::SimpleVector::new(i, j));
        }
    }
    // dbg!(map);
    for (_, nodes) in map {
        for (i, x) in nodes.iter().enumerate() {
            for y in nodes[..i].iter() {
                let xy = y - x;
                for point in [x - &xy, y + &xy] {
                    if point.x < grid_size.0 && point.y < grid_size.1 {
                        antinodes.insert(point);
                    }
                }
            }
        }
    }
    antinodes.len().to_u64()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map: HashMap<char, Vec<_>> = HashMap::new();
    let mut antinodes: HashSet<_> = HashSet::new();
    let grid_size = (input.lines().count(), input.lines().next().unwrap().len());
    for (i, line) in input.lines().enumerate() {
        for (j, cell) in line.chars().enumerate() {
            if cell == '.' {
                continue;
            }
            map.entry(cell)
                .or_default()
                .push(util::SimpleVector::new(i, j));
        }
    }
    // dbg!(map);
    for (_, nodes) in map {
        for (i, x) in nodes.iter().enumerate() {
            for y in nodes[..i].iter() {
                let xy = y - x;
                let mut tmp = x.clone();
                while tmp.x < grid_size.0 && tmp.y < grid_size.1 {
                    antinodes.insert(tmp.clone());
                    tmp = &tmp - &xy;
                }
                let mut tmp = y.clone();
                while tmp.x < grid_size.0 && tmp.y < grid_size.1 {
                    antinodes.insert(tmp.clone());
                    tmp = &tmp + &xy;
                }
            }
        }
    }
    antinodes.len().to_u64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
