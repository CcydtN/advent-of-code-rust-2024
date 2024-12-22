use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(5);

fn part_one_check_order(update: &[u64], rules: &HashMap<u64, HashSet<u64>>) -> bool {
    let mut table = rules.get(&update[0]).cloned().unwrap_or(HashSet::new());
    // Skipping the first and last item
    let mut iter = update.into_iter().skip(1);
    let last = iter.next_back().unwrap();

    for update in iter {
        if !table.contains(update) {
            return false;
        }
        if let Some(rule) = rules.get(update) {
            table = table.intersection(&rule).cloned().collect();
        } else {
            return false;
        }
    }
    table.contains(last)
}

fn parse(input: &str) -> Option<(HashMap<u64, HashSet<u64>>, Vec<Vec<u64>>)> {
    let (rules_input, updates_input) = input.split("\n\n").collect_tuple()?;

    let mut rules = HashMap::new();
    for line in rules_input.lines().take_while(|line| line != &"") {
        let (before, after) = dbg!(line)
            .split("|")
            .filter_map(|val| val.parse::<u64>().ok())
            .collect_tuple()?;
        rules.entry(before).or_insert(HashSet::new()).insert(after);
    }

    let mut updates = vec![];
    for line in updates_input.lines() {
        let update: Result<Vec<u64>, _> = line.split(",").map(|item| item.parse::<u64>()).collect();
        updates.push(update.ok()?)
    }
    Some((rules, updates))
}

pub fn part_one(input: &str) -> Option<u64> {
    // dbg!(input);
    let (rules, updates) = parse(input)?;

    let mut sum = 0;
    for update in updates {
        if part_one_check_order(&update, &rules) {
            dbg!(&update);
            sum += update[update.len() / 2];
        }
    }
    Some(sum)
}

fn part_two_helper(update: Vec<u64>, rules: &HashMap<u64, HashSet<u64>>) -> Vec<u64> {
    let updates = update.into_iter().collect::<HashSet<u64>>();
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
    let (rules, updates) = parse(input)?;

    let mut sum = 0;
    for update in updates {
        if part_one_check_order(&update, &rules) {
            continue;
        }
        let new_items = part_two_helper(update, &rules);
        dbg!(&new_items);
        sum += new_items[new_items.len() / 2];
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
