use std::{collections::hash_map, fs};

fn main() {
    let input = fs::read_to_string("res/day10.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let cpu = CPU::run_str(input);
    cpu.signal_sum()
}

fn part2(input: &str) -> i32 {
    input.len() as i32
}

enum Opcode {
    noop,
    addx,
}

type Instruction = (Opcode, i32);

fn line_to_instruction(line: &str) -> Instruction {
    let mut words = line.trim().split_whitespace();
    let opcode = match words.next() {
        Some("noop") => Opcode::noop,
        Some("addx") => Opcode::addx,
        _ => panic!("Invalid opcode for line: {}", line.trim()),
    };

    let operand = match opcode {
        Opcode::noop => 0,
        Opcode::addx => words.next().unwrap().parse::<i32>().unwrap(),
    };

    (opcode, operand)
}

fn duration(opcode: &Opcode) -> i32 {
    match opcode {
        Opcode::noop => 1,
        Opcode::addx => 2,
    }
}

struct CPU {
    X: i32,
    cycle: usize,
    history: Vec<i32>,
    program: Vec<Instruction>,
}

impl CPU {
    fn new(program: Vec<Instruction>) -> CPU {
        CPU {
            X: 1,
            cycle: 0,
            history: vec![1],
            program,
        }
    }

    fn step(&mut self) {}

    fn run(&mut self) {
        for (op, arg) in self.program.iter() {
            let duration = duration(op);
            for _ in 0..duration {
                self.history.push(self.X);
            }

            match op {
                Opcode::noop => {}
                Opcode::addx => self.X += arg,
            }
        }
    }

    fn from_str(input: &str) -> CPU {
        let mut program = Vec::new();
        for line in input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
            program.push(line_to_instruction(line));
        }
        CPU::new(program)
    }

    fn run_str(input: &str) -> CPU {
        let mut cpu = CPU::from_str(input);
        cpu.run();
        cpu
    }

    fn signal_strength(&self, cycle: usize) -> i32 {
        self.history[cycle] * cycle as i32
    }

    fn signal_sum(&self) -> i32 {
        self.history
            .iter()
            .enumerate()
            .skip(20)
            .step_by(40)
            .map(|(i, x)| x * i as i32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn test_part1() {
        let cpu = CPU::run_str("");

        assert_eq!(cpu.X, 1);
        assert_eq!(cpu.cycle, 0);

        assert_eq!(cpu.history[cpu.cycle], cpu.X);

        let cpu = CPU::run_str(TEST_INPUT);

        assert_eq!(cpu.history[20], 21);
        assert_eq!(cpu.history[60], 19);
        assert_eq!(cpu.history[100], 18);
        assert_eq!(cpu.history[140], 21);
        assert_eq!(cpu.history[180], 16);
        assert_eq!(cpu.history[220], 18);

        assert_eq!(cpu.signal_sum(), 13140);
    }

    #[test]
    fn test_part2() {}
}
