advent_of_code::solution!(3);

// start with "mul("
pub fn try_mul(input: &str) -> Option<(u64, usize)> {
    if !input.starts_with("mul(") {
        return None;
    }
    let input: &str = &input[4..];

    // We can add more check in here, make sure closing is within the rage
    // X,Y are each 1-3 digit, mean closing is in [3, 7]
    // But it seem not necessary
    let closing = input.find(")")?;
    let input: &str = &input[..closing];

    let args: Result<Vec<u64>, _> = input.split(",").map(|arg| arg.parse::<u64>()).collect();
    let args = args.ok()?;
    if args.len() > 2 {
        return None;
    }
    Some((args.into_iter().product(), closing))
}

pub fn part_one(input: &str) -> Option<u64> {
    // dbg!(input);
    // 175015740
    let mut input = input;
    let mut sum = 0;
    while let Some(start) = input.find("mul(") {
        // dbg!(args);
        input = &input[start..];
        if let Some((val, closing)) = try_mul(input) {
            sum += val;
            input = &input[closing + 1..];
        } else {
            input = &input[1..];
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = input;
    let mut sum = 0;
    let mut is_enable = true;
    const PATTERN: [&str; 3] = ["mul(", "do()", "don't()"];
    while let Some(start) = PATTERN.into_iter().filter_map(|pat| input.find(pat)).min() {
        input = &input[start..];
        if input.starts_with(PATTERN[0]) {
            if is_enable {
                if let Some((val, closing)) = try_mul(input) {
                    sum += val;
                    input = &input[closing + 1..];
                    continue;
                }
            }
        } else if input.starts_with(PATTERN[1]) {
            is_enable = true;
        } else if input.starts_with(PATTERN[2]) {
            is_enable = false;
        } else {
            unreachable!()
        }
        input = &input[1..];
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
