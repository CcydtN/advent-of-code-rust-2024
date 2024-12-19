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

#[derive(Debug, Clone)]
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
        numerator >> self.get_combo_operand(operand)
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

pub fn test_factor() -> usize {
    if cfg!(test) {
        3
    } else {
        0
    }
}

fn dfs(value: usize, targets: &[usize], computer: &Computer) -> Option<usize> {
    if targets.len() == 0 {
        return Some(value >> 3 - test_factor());
    }
    for guess in 0..8 {
        let mut clone = computer.clone();
        clone.registers[0] = (value + guess) << test_factor();
        let output = clone.run_until_halts();
        if output[0] == targets[0] {
            if let Some(ret) = dfs((value + guess) << 3, &targets[1..], computer) {
                return Some(ret);
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    // dbg!(input);
    let computer = Computer::parse_from(input)?;
    let mut program = computer.program.clone();
    // for values in program.chunks(2) {
    //     let opcode = values[0];
    //     let operand = values[1];
    //     dbg!(Opcode::from(opcode), operand);
    // }

    // Every value is only affected by the last 3 bit
    // use dfs to loop over those possibility
    program.reverse();
    dfs(0, &program, &computer)?.to_u64()
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
        assert_eq!(result, Some(117440));
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

    // #[test]
    // fn instruction_expample_06() {
    //     let mut computer = Computer::new(
    //         [34640, 0, 0],
    //         [2, 4, 1, 2, 7, 5, 4, 1, 1, 3, 5, 5, 0, 3, 3, 0].to_vec(),
    //     );
    //     let output = computer.run_until_halts();
    //     // dbg!(&computer);
    //     assert_eq!(output, &[2, 4, 1, 2, 7, 5, 4, 1, 1, 3, 5, 5, 0, 3, 3, 0]);
    // }
}
