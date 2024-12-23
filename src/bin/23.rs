use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use num::ToPrimitive;

advent_of_code::solution!(23);

// Challenge for future me:
// Try rewire the second part with iterator or yield, like the python example in
// https://www.geeksforgeeks.org/maximal-clique-problem-recursive-solution/

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

fn bron_kerbosch<'a>(
    r: HashSet<&'a str>,
    mut p: HashSet<&'a str>,
    mut x: HashSet<&'a str>,
    neighbors: &HashMap<&'a str, HashSet<&'a str>>,
) -> Vec<HashSet<&'a str>> {
    let mut cliques = vec![];
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
    }
    while !p.is_empty() {
        let value = p.iter().next().cloned().unwrap();
        let mut new_r = r.clone();
        new_r.insert(value);
        let new_p = p.intersection(&neighbors[&value]).cloned().collect();
        let new_x = x.intersection(&neighbors[&value]).cloned().collect();
        cliques.append(&mut bron_kerbosch(new_r, new_p, new_x, neighbors));
        p.remove(value);
        x.insert(value);
    }
    return cliques;
}

// Maximal Clique Problem
pub fn part_two(input: &str) -> Option<String> {
    let pairs = parse(input).unwrap();
    let mut neighbors = HashMap::new();
    for &(a, b) in pairs.iter() {
        neighbors.entry(a).or_insert(HashSet::new()).insert(b);
        neighbors.entry(b).or_insert(HashSet::new()).insert(a);
    }

    let mut largest_set = bron_kerbosch(
        HashSet::new(),
        neighbors.keys().cloned().collect(),
        HashSet::new(),
        &neighbors,
    )
    .into_iter()
    .max_by_key(|v| v.len())
    .unwrap()
    .into_iter()
    .collect_vec();

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
