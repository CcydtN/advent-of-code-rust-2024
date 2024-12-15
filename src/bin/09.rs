use std::collections::VecDeque;

use itertools::Itertools;
use num::ToPrimitive;

advent_of_code::solution!(9);

#[derive(Debug, Default)]
enum State {
    #[default]
    Files,
    Space,
}

#[derive(Debug, Default)]
struct Helper {
    state: State,
    base_id: u64,
    files: VecDeque<u64>,
    spaces: VecDeque<u64>,
}

impl Helper {
    fn new(files: VecDeque<u64>, spaces: VecDeque<u64>) -> Self {
        Self {
            files,
            spaces,
            ..Default::default()
        }
    }

    fn next(&mut self) -> Option<u64> {
        match self.state {
            State::Files => self.next_file(),
            State::Space => self.next_space(),
        }
    }

    fn next_file(&mut self) -> Option<u64> {
        let ret = self.base_id;
        let count: &mut u64 = self.files.front_mut()?;
        if count == &0 {
            let _ = self.files.pop_front();
            self.base_id += 1;
            self.state = State::Space;
            return self.next();
        } else {
            *count -= 1;
        }
        Some(ret)
    }

    fn next_space(&mut self) -> Option<u64> {
        let ret = self.base_id + self.files.len().to_u64().unwrap() - 1;
        {
            let count = self.spaces.front_mut()?;
            if count == &0 {
                let _ = self.spaces.pop_front();
                self.state = State::Files;
                return self.next();
            } else {
                *count -= 1;
            }
        }
        {
            // take file from back
            let count = self.files.back_mut()?;
            *count -= 1;
            if count == &0 {
                let _ = self.files.pop_back();
                let _ = self.spaces.pop_back();
            }
        }
        Some(ret)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    // dbg!(input);
    let mut tmp = [VecDeque::new(), VecDeque::new()];
    tmp.iter_mut().for_each(|x| x.reserve(input.len() / 2));
    for (i, c) in input.trim().chars().enumerate() {
        tmp[i % 2].push_back(c.to_digit(10).unwrap().to_u64().unwrap());
    }
    let [files, spaces] = tmp;

    let mut helper = Helper::new(files, spaces);
    let mut pos = 0;
    let mut sum = 0;
    while let Some(i) = helper.next() {
        sum += i * pos;
        pos += 1;
    }
    sum.to_u64()
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
