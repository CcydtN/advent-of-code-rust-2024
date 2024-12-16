use itertools::Itertools;

advent_of_code::solution!(4);

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

enum Dir {
    U,
    D,
    L,
    R,
    UL,
    DL,
    UR,
    DR,
}
const ALL_DIR: [Dir; 8] = [
    Dir::U,
    Dir::D,
    Dir::L,
    Dir::R,
    Dir::UL,
    Dir::DL,
    Dir::UR,
    Dir::DR,
];

impl Dir {
    fn advance(&self, grid: &Vec<Vec<char>>, (i, j): (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Dir::U => {
                if i == 0 {
                    None
                } else {
                    Some((i - 1, j))
                }
            }
            Dir::D => {
                if i >= grid.len() - 1 {
                    None
                } else {
                    Some((i + 1, j))
                }
            }
            Dir::L => {
                if j == 0 {
                    None
                } else {
                    Some((i, j - 1))
                }
            }
            Dir::R => {
                if j >= grid[0].len() - 1 {
                    None
                } else {
                    Some((i, j + 1))
                }
            }
            Dir::UL => Self::advance(&Self::U, grid, Self::advance(&Self::L, grid, (i, j))?),
            Dir::DL => Self::advance(&Self::D, grid, Self::advance(&Self::L, grid, (i, j))?),
            Dir::UR => Self::advance(&Self::U, grid, Self::advance(&Self::R, grid, (i, j))?),
            Dir::DR => Self::advance(&Self::D, grid, Self::advance(&Self::R, grid, (i, j))?),
        }
    }
}

pub fn check(grid: &Vec<Vec<char>>, (i, j): (usize, usize), dir: Dir, depth: usize) -> bool {
    if depth == XMAS.len() {
        return true;
    }
    if let Some((i, j)) = dir.advance(grid, (i, j)) {
        // dbg!(i, j);
        if grid[i][j] == XMAS[depth] {
            return check(grid, (i, j), dir, depth + 1);
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    // dbg!(input);
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let m = grid.len();
    let n = grid[0].len();
    dbg!(m, n);
    let mut count = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if ch != &XMAS[0] {
                continue;
            }
            // dbg!(i, j);
            for dir in ALL_DIR {
                if check(&grid, (i, j), dir, 1) {
                    count += 1;
                }
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let m = grid.len();
    let n = grid[0].len();
    dbg!(m, n);
    let mut count = 0;
    for (i, row) in grid[..m - 1].iter().enumerate().skip(1) {
        for (j, ch) in row[..n - 1].iter().enumerate().skip(1) {
            if ch != &'A' {
                continue;
            }
            match (
                (grid[i + 1][j + 1], grid[i - 1][j - 1]),
                (grid[i - 1][j + 1], grid[i + 1][j - 1]),
            ) {
                (('M', 'S'), ('S', 'M')) => {
                    count += 1;
                }
                (('M', 'S'), ('M', 'S')) => {
                    count += 1;
                }
                (('S', 'M'), ('S', 'M')) => {
                    count += 1;
                }
                (('S', 'M'), ('M', 'S')) => {
                    count += 1;
                }
                ((_, _), (_, _)) => {}
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
