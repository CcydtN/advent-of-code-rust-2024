use num::ToPrimitive;

advent_of_code::solution!(3);

// start with "mul("
pub fn try_mul_once(input: &str) -> Option<(u64, usize)> {
    if !input.starts_with("mul(") {
        return None;
    }
    let input: &str = &input[4..];
    let closing = input.find(")")?;
    let input: &str = &input[..closing];
    // dbg!(&input);
    let mut args = [0u64, 0u64];
    let mut idx = 0;
    for c in input.chars() {
        match c {
            ',' => {
                idx += 1;
                if idx > 2 {
                    return None;
                }
            }
            '0'..='9' => {
                args[idx] *= 10u64;
                args[idx] += c.to_digit(10).unwrap().to_u64().unwrap()
            }
            _ => {
                return None;
            }
        }
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
        if let Some((val, closing)) = try_mul_once(input) {
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
    const pattern: [&str; 3] = ["mul(", "do()", "don't()"];
    while let Some(start) = pattern.into_iter().filter_map(|pat| input.find(pat)).min() {
        input = &input[start..];
        if input.starts_with(pattern[0]) {
            if is_enable {
                if let Some((val, closing)) = try_mul_once(input) {
                    sum += val;
                    input = &input[closing + 1..];
                    continue;
                }
            }
        } else if input.starts_with(pattern[1]) {
            is_enable = true;
        } else if input.starts_with(pattern[2]) {
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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
