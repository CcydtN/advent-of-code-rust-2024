use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(5);

fn part_one_check_order(updates: &[u64], rules: &HashMap<u64, HashSet<u64>>) -> bool {
    let mut table = rules.get(&updates[0]).cloned().unwrap_or(HashSet::new());
    for update in updates.into_iter().skip(1) {
        if !table.contains(update) {
            return false;
        }
        table = table.intersection(&rules[update]).cloned().collect();
    }
    true
}

pub fn part_one(input: &str) -> Option<u64> {
    // dbg!(input);
    let mut parts = input.split("\n\n");
    let mut rules = HashMap::new();
    for line in parts.next().unwrap().lines() {
        let sep = line.find('|').unwrap();
        let before = line[..sep].parse::<u64>().unwrap();
        let after = line[sep + 1..].parse::<u64>().unwrap();
        rules.entry(before).or_insert(HashSet::new()).insert(after);
    }
    // dbg!(&rules);

    let mut sum = 0;
    for line in parts.next().unwrap().lines() {
        let items = line
            .split(",")
            .map(|item| item.parse::<u64>().unwrap())
            .collect_vec();
        if part_one_check_order(&items, &rules) {
            sum += items[items.len() / 2];
        }
    }
    Some(sum)
}

fn part_two_helper(updates: Vec<u64>, rules: &HashMap<u64, HashSet<u64>>) -> Vec<u64> {
    let updates = updates.into_iter().collect::<HashSet<u64>>();
    let purned_rules = rules
        .into_iter()
        .filter(|(key, _)| updates.contains(key))
        .map(|(&key, val)| (key, val.intersection(&updates).count()))
        .collect::<HashMap<u64, _>>();
    let mut tmp = purned_rules.into_iter().collect_vec();
    tmp.sort_by(|a, b| b.1.cmp(&a.1));
    // dbg!(&tmp);
    tmp.into_iter().map(|x| x.0).collect_vec()
}

pub fn part_two(input: &str) -> Option<u64> {
    // dbg!(input);
    let mut parts = input.split("\n\n");
    let mut rules = HashMap::new();
    for line in parts.next().unwrap().lines() {
        let sep = line.find('|').unwrap();
        let before = line[..sep].parse::<u64>().unwrap();
        let after = line[sep + 1..].parse::<u64>().unwrap();
        rules.entry(before).or_insert(HashSet::new()).insert(after);
    }
    // dbg!(&rules);

    let mut sum = 0;
    for line in parts.next().unwrap().lines() {
        let items = line
            .split(",")
            .map(|item| item.parse::<u64>().unwrap())
            .collect_vec();
        let new_items = part_two_helper(items.clone(), &rules);
        if new_items != items {
            sum += new_items[new_items.len() / 2];
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
