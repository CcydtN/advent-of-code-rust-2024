use itertools::Itertools;

advent_of_code::solution!(22);

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

pub fn part_one(input: &str) -> Option<u64> {
    let secrets = input
        .split_whitespace()
        .map(|line| line.parse::<u64>().unwrap())
        .collect_vec();
    let results = secrets
        .into_iter()
        .map(|mut secret| {
            for _ in 0..2000 {
                secret = next_secret(secret);
            }
            secret
        })
        .collect_vec();
    Some(results.into_iter().sum())
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
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
