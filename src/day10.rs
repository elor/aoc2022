use std::fs;

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
    let cpu = CPU::run_str(input);

    println!("{}", cpu.output);

    return -1;
}

enum Opcode {
    Noop,
    Addx,
}

type Instruction = (Opcode, i32);

fn line_to_instruction(line: &str) -> Instruction {
    let mut words = line.trim().split_whitespace();
    let opcode = match words.next() {
        Some("noop") => Opcode::Noop,
        Some("addx") => Opcode::Addx,
        _ => panic!("Invalid opcode for line: {}", line.trim()),
    };

    let operand = match opcode {
        Opcode::Noop => 0,
        Opcode::Addx => words.next().unwrap().parse::<i32>().unwrap(),
    };

    (opcode, operand)
}

fn duration(opcode: &Opcode) -> i32 {
    match opcode {
        Opcode::Noop => 1,
        Opcode::Addx => 2,
    }
}

struct CPU {
    x: i32,
    cycle: usize,
    history: Vec<i32>,
    program: Vec<Instruction>,
    output: String,
}

impl CPU {
    fn new(program: Vec<Instruction>) -> CPU {
        CPU {
            x: 1,
            cycle: 0,
            history: vec![1],
            program,
            output: String::new(),
        }
    }

    fn screen_x(&self) -> usize {
        self.cycle % 40
    }

    fn is_lit(&self) -> bool {
        (self.screen_x() as i32 - self.x).abs() <= 1
    }

    fn run(&mut self) {
        for (op, arg) in self.program.iter() {
            let duration = duration(op);
            for _ in 0..duration {
                /* before cycle */

                /* during cycle */
                self.history.push(self.x);

                let c = if self.is_lit() { '#' } else { ' ' };
                self.output.push(c);

                self.cycle += 1;
                if self.screen_x() == 0 {
                    self.output.push('\n');
                }
            }

            /* after cycle */
            match op {
                Opcode::Noop => {}
                Opcode::Addx => self.x += arg,
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
mod day10 {
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

        assert_eq!(cpu.x, 1);
        assert_eq!(cpu.cycle, 0);

        assert_eq!(cpu.history[cpu.cycle], cpu.x);

        let cpu = CPU::run_str(TEST_INPUT);

        assert_eq!(cpu.history[20], 21);
        assert_eq!(cpu.history[60], 19);
        assert_eq!(cpu.history[100], 18);
        assert_eq!(cpu.history[140], 21);
        assert_eq!(cpu.history[180], 16);
        assert_eq!(cpu.history[220], 18);

        assert_eq!(cpu.signal_sum(), 13140);
    }

    const REFERENCE_OUTPUT: &str = "##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     \n";

    #[test]
    fn test_part2() {
        let cpu = CPU::run_str(TEST_INPUT);

        assert_eq!(cpu.output, REFERENCE_OUTPUT);
    }
}
