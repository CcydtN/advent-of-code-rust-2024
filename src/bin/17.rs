use itertools::Itertools;
use num::ToPrimitive;

advent_of_code::solution!(17);

#[derive(Debug, PartialEq, Eq)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<usize> for Opcode {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Computer {
    registers: [usize; 3],
    program: Vec<usize>,
    instruction_pointer: usize,
    output: Vec<usize>,
}

impl Computer {
    fn new(registers: [usize; 3], program: Vec<usize>) -> Self {
        Self {
            registers,
            program,
            instruction_pointer: 0,
            output: vec![],
        }
    }

    fn get_combo_operand(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4..=6 => self.registers[operand - 4],
            _ => unreachable!(),
        }
    }

    fn do_division(&self, operand: usize) -> usize {
        let numerator = self.registers[0];
        let denominator = 2usize.pow(self.get_combo_operand(operand).to_u32().unwrap());
        numerator / denominator
    }

    fn run_until_halts(&mut self) -> Vec<usize> {
        while self.run_once() {}
        self.output.clone()
    }

    fn run_once(&mut self) -> bool {
        if self.instruction_pointer >= self.program.len() {
            return false;
        }
        let opcode: Opcode = self.program[self.instruction_pointer].try_into().unwrap();
        let operand = self.program[self.instruction_pointer + 1];
        match opcode {
            Opcode::Adv => self.registers[0] = self.do_division(operand),
            Opcode::Bxl => self.registers[1] ^= operand,
            Opcode::Bst => self.registers[1] = self.get_combo_operand(operand) % 8,
            Opcode::Jnz => {
                if self.registers[0] != 0 {
                    assert!(operand < self.instruction_pointer);
                    self.instruction_pointer = operand;
                    return true;
                }
            }
            Opcode::Bxc => self.registers[1] ^= self.registers[2],
            Opcode::Out => self.output.push(self.get_combo_operand(operand) % 8),
            Opcode::Bdv => self.registers[1] = self.do_division(operand),
            Opcode::Cdv => self.registers[2] = self.do_division(operand),
        }
        self.instruction_pointer += 2;
        true
    }

    fn parse_from(input: &str) -> Option<Self> {
        let mut iter = input.lines();
        let mut register = [0; 3];
        for i in 0..3 {
            let line = iter.next().unwrap().trim();
            let pattern = line.find(": ").unwrap();
            register[i] = line[pattern + 2..].parse::<usize>().ok()?;
        }
        iter.next();
        let line = iter.next()?.trim();
        let pattern = line.find(": ")?;
        let program: Result<Vec<_>, _> = line[pattern + 2..]
            .split(",")
            .map(|val| val.parse())
            .collect();
        Some(Self::new(register, program.ok()?))
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = Computer::parse_from(input)?;
    let output = computer.run_until_halts();
    Some(output.into_iter().map(|val| val.to_string()).join(","))
}

pub fn part_two(input: &str) -> Option<u64> {
    // Check the following two things,
    // 1. Will the input to any odd idx
    // 2. Is the jump going forward or backward
    {
        let computer = Computer::parse_from(input)?;
        computer
            .program
            .chunks(2)
            .enumerate()
            .for_each(|(i, values)| {
                let opcode = values[0];
                let operand = values[1];
                if opcode == Opcode::Jnz as usize {
                    assert!(operand < (2 * i));
                }
            })
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        // assert_eq!(result, Some(117440));
    }

    #[test]
    fn instruction_expample_01() {
        let mut computer = Computer::new([0, 0, 9], [2, 6].to_vec());
        computer.run_until_halts();
        dbg!(&computer);
        assert_eq!(computer.registers[1], 1);
    }

    #[test]
    fn instruction_expample_02() {
        let mut computer = Computer::new([10, 0, 0], [5, 0, 5, 1, 5, 4].to_vec());
        let output = computer.run_until_halts();
        dbg!(&computer);
        assert_eq!(&output, &[0, 1, 2]);
    }

    #[test]
    fn instruction_expample_03() {
        let mut computer = Computer::new([2024, 0, 0], [0, 1, 5, 4, 3, 0].to_vec());
        let output = computer.run_until_halts();
        dbg!(&computer);
        assert_eq!(&output, &[4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.registers[0], 0);
    }

    #[test]
    fn instruction_expample_04() {
        let mut computer = Computer::new([0, 29, 0], [1, 7].to_vec());
        computer.run_until_halts();
        dbg!(&computer);
        assert_eq!(computer.registers[1], 26);
    }

    #[test]
    fn instruction_expample_05() {
        let mut computer = Computer::new([0, 2024, 43690], [4, 0].to_vec());
        computer.run_until_halts();
        dbg!(&computer);
        assert_eq!(computer.registers[1], 44354);
    }
}
