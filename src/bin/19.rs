use itertools::Itertools;
use num::ToPrimitive;

advent_of_code::solution!(19);

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut iter = input.lines();
    let building_blocks = iter.next().unwrap().split(", ").collect_vec();
    let _ = iter.next();
    let targets = iter.collect_vec();
    (building_blocks, targets)
}

// Ugly approach, but it works
fn try_build_target<'a>(building_blocks: &[&'a str], target: &'a str) -> Option<Vec<&'a str>> {
    if target.len() == 0 {
        return Some(vec![]);
    }
    for block in building_blocks {
        if target.starts_with(block) {
            if let Some(mut result) = try_build_target(building_blocks, &target[block.len()..]) {
                result.push(block);
                return Some(result);
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let (building_blocks, targets) = parse_input(input);
    // dbg!(&building_blocks, &targets);
    targets
        .into_iter()
        .filter_map(|target| try_build_target(&building_blocks, target))
        .count()
        .to_u64()
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
