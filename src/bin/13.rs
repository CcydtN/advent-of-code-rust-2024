use itertools::Itertools;
use num::{integer::gcd, ToPrimitive};

advent_of_code::solution!(13);

#[derive(Debug)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    price: (i64, i64),
}

impl Machine {
    fn new(button_a: (i64, i64), button_b: (i64, i64), price: (i64, i64)) -> Self {
        Self {
            button_a,
            button_b,
            price,
        }
    }

    fn parse(input: &str) -> Self {
        // dbg!(input);
        let [button_a, button_b, price] = input
            .lines()
            .map(Self::parse_x_y)
            .collect_vec()
            .try_into()
            .unwrap();
        Self::new(button_a, button_b, price)
    }

    fn parse_x_y(line: &str) -> (i64, i64) {
        let x_start = line.find('X').unwrap() + 2;
        let x_end = line.find(',').unwrap();
        let y_start = line.find('Y').unwrap() + 2;
        (
            line[x_start..x_end].parse().unwrap(),
            line[y_start..].parse().unwrap(),
        )
    }

    fn solve_press_count(&self) -> Option<(i64, i64)> {
        let a = self.button_a.0;
        let b = self.button_b.0;
        let c = self.button_a.1;
        let d = self.button_b.1;
        let e = self.price.0;
        let f = self.price.1;
        // [[a,b]][x]=[e]
        // [[c,d]][y]=[f]
        let determinant = a * d - b * c;
        // Check if the determinant is zero (matrix is singular, no unique solution)
        // dbg!(determinant);
        if determinant == 0 {
            return None;
        }

        // Multiply the inverse matrix by the result matrix [e, f]
        let x = d * e - b * f;
        let y = -c * e + a * f;
        // dbg!(x, y);
        if gcd(x, determinant) == determinant.abs() && gcd(y, determinant) == determinant.abs() {
            Some((x / determinant, y / determinant))
        } else {
            None
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    // dbg!(input);
    let machines = input.split("\n\n").map(Machine::parse).collect_vec();
    // dbg!(&machines);
    let mut sum = 0;
    for machine in machines {
        // dbg!(&machine);
        let solution = machine.solve_press_count();
        // dbg!(solution);
        if let Some((a_press, b_press)) = solution {
            if a_press > 100 || b_press > 100 {
                continue;
            }
            sum += 3 * a_press + b_press;
        }
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
