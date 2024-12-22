use std::{
    collections::{HashMap, HashSet},
    iter,
};

use itertools::Itertools;
use num::ToPrimitive;

advent_of_code::solution!(22);

// How the code will be if I turn secret to its own struct?

fn mix(val: u64, secret: u64) -> u64 {
    val ^ secret
}

fn prune(val: u64) -> u64 {
    val % 16777216
}

fn next_secret(secret: u64) -> u64 {
    let secret = prune(mix(secret, secret * 64));
    let secret = prune(mix(secret, secret / 32));
    let secret = prune(mix(secret, secret * 2048));
    secret
}

fn secret_iter(mut secret: u64) -> impl Iterator<Item = u64> {
    iter::from_fn(move || {
        secret = next_secret(secret);
        Some(secret)
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let secrets = input
        .split_whitespace()
        .map(|line| line.parse::<u64>().unwrap())
        .collect_vec();
    let results = secrets
        .into_iter()
        .map(|secret| secret_iter(secret).nth(2000 - 1).unwrap())
        .collect_vec();
    Some(results.into_iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let secrets = input
        .split_whitespace()
        .map(|line| line.parse::<u64>().unwrap())
        .collect_vec();
    let buyers = secrets.into_iter().map(|secret| {
        secret_iter(secret)
            .take(2000)
            .map(|secret| (secret % 10) as i64)
            .collect_vec()
    });

    let mut bananas = HashMap::new();
    for prices in buyers {
        let mut visited = HashSet::new();
        for window in prices.windows(5) {
            let price_change = window.windows(2).map(|x| x[1] - x[0]).collect_vec();
            let price_change: [i64; 4] = price_change.try_into().unwrap();
            if visited.contains(&price_change) {
                continue;
            }
            visited.insert(price_change);
            let entry = bananas.entry(price_change).or_insert(0);
            *entry += window[4];
        }
    }
    bananas.into_values().max().unwrap().to_u64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
