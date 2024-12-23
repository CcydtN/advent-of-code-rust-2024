use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use num::ToPrimitive;

advent_of_code::solution!(23);

fn parse<'a>(input: &'a str) -> Option<Vec<(&'a str, &'a str)>> {
    input
        .split_whitespace()
        .map(|x| x.split("-").collect_tuple())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let pairs = parse(input).unwrap();
    let mut neighbors = HashMap::new();
    for (a, b) in pairs {
        neighbors.entry(a).or_insert(HashSet::new()).insert(b);
        neighbors.entry(b).or_insert(HashSet::new()).insert(a);
    }

    let mut inter_connected_with_t = HashSet::new();
    for (t_name, t_list) in neighbors.iter().filter(|(name, _)| name.starts_with("t")) {
        for nei_name in t_list {
            for item in t_list.intersection(&neighbors[nei_name]) {
                let mut tmp = [item, nei_name, t_name];
                tmp.sort();
                inter_connected_with_t.insert(tmp);
            }
        }
    }
    inter_connected_with_t.len().to_u64()
}

fn find_upper_bound(pair_count: usize) -> usize {
    let mut product = 1 * 2 * 3;
    for i in 4..pair_count {
        product *= i;
        if pair_count < product {
            return i;
        }
    }
    unreachable!()
}

fn find_set_with_size<'a>(
    neighbors: &HashMap<&'a str, HashSet<&'a str>>,
    size: usize,
) -> Option<Vec<&'a str>> {
    'outer: for combination in neighbors.keys().cloned().combinations(size) {
        let set = combination.iter().cloned().collect::<HashSet<_>>();
        for item in &combination {
            let interset_count = neighbors[item].intersection(&set).count();
            if interset_count != set.len() - 1 {
                continue 'outer;
            }
        }
        return Some(combination);
    }
    None
}

pub fn part_two(input: &str) -> Option<String> {
    let pairs = parse(input).unwrap();
    let mut neighbors = HashMap::new();
    for &(a, b) in pairs.iter() {
        neighbors.entry(a).or_insert(HashSet::new()).insert(b);
        neighbors.entry(b).or_insert(HashSet::new()).insert(a);
    }

    let mut left = 3;
    let mut right = find_upper_bound(pairs.len());
    while dbg!(left) < dbg!(right) {
        let mid = (left + right) / 2;
        if find_set_with_size(&neighbors, mid).is_some() {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    let mut largest_set = find_set_with_size(&neighbors, left - 1).unwrap();
    largest_set.sort();
    let password = largest_set.join(",");
    Some(password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_owned()));
    }
}
