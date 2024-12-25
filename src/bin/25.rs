use itertools::iproduct;

advent_of_code::solution!(25);

fn parse(input: &str) -> Option<(Vec<[i64; 5]>, Vec<[i64; 5]>)> {
    let mut keys = vec![];
    let mut locks = vec![];
    let mut buffer = [0; 5];
    for grid in input.split("\n\n") {
        for row in grid.split_whitespace() {
            for (i, cell) in row.trim().chars().enumerate() {
                buffer[i] += if cell == '#' { 1 } else { -1 };
            }
        }
        // as top row is filled
        // if grid.chars().take(5).all(|c| c == '#') {
        if grid.chars().next() == Some('#') {
            locks.push(buffer);
        } else {
            keys.push(buffer);
        }
        buffer.fill(0);
    }
    Some((keys, locks))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (keys, locks) = dbg!(parse(input))?;

    let mut count = 0;
    for (key, lock) in iproduct!(keys, locks) {
        if key.iter().zip(lock).all(|(a, b)| a + b <= 0) {
            count += 1;
        }
    }
    Some(count)
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
