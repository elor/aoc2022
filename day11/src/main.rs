use sscanf::sscanf;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);

    inspection_rounds_part1(&mut monkeys, 20);

    return monkey_business_level(&monkeys) as usize;
}

fn part2(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);

    inspection_rounds_part2(&mut monkeys, 10_000);

    return monkey_business_level(&monkeys) as usize;
}

#[derive(Debug)]
enum Operation {
    Add(i64),
    Multiply(i64),
    Divide(i64),
    Subtract(i64),
    Square(),
    Identity(),
}

impl Operation {
    fn apply(&self, input: i64) -> i64 {
        match self {
            Operation::Add(arg) => input + arg,
            Operation::Multiply(arg) => input * arg,
            Operation::Divide(arg) => input / arg,
            Operation::Subtract(arg) => input - arg,
            Operation::Square() => input * input,
            Operation::Identity() => input,
        }
    }
}

#[derive(Debug)]
enum Test {
    DivisibleBy(i64),
}

impl Test {
    fn perform(&self, input: i64) -> bool {
        match self {
            Test::DivisibleBy(divisor) => input % divisor == 0,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<i64>,
    operation: Operation,
    test: Test,
    if_true: usize,
    if_false: usize,
    items_inspected: usize,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            id: 0,
            items: Vec::new(),
            operation: Operation::Add(0),
            test: Test::DivisibleBy(1),
            if_true: 0,
            if_false: 0,
            items_inspected: 0,
        }
    }

    fn from_str(input: &str) -> Monkey {
        let mut monkey = Monkey::new();

        for line in input.lines() {
            if let Ok(id) = sscanf!(line, "Monkey {usize}:") {
                monkey.id = id;
            } else if let Ok(parsed) = sscanf!(line, "  Starting items: {str}") {
                monkey.items = parsed
                    .split(", ")
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect();
            } else if let Ok((op, arg)) = sscanf!(line, "  Operation: new = old {char} {i64}") {
                monkey.operation = match op {
                    '+' => Operation::Add(arg),
                    '*' => Operation::Multiply(arg),
                    '/' => Operation::Divide(arg),
                    '-' => Operation::Subtract(arg),
                    _ => panic!("Unknown operation: {}", op),
                };
            } else if line == "  Operation: new = old * old" {
                monkey.operation = Operation::Square();
            } else if let Ok(divisor) = sscanf!(line, "  Test: divisible by {i64}") {
                monkey.test = Test::DivisibleBy(divisor);
            } else if let Ok(target) = sscanf!(line, "    If true: throw to monkey {usize}") {
                monkey.if_true = target;
            } else if let Ok(target) = sscanf!(line, "    If false: throw to monkey {usize}") {
                monkey.if_false = target;
            } else {
                panic!("Unmatched monkey definition line:\n'{}'", line);
            }
        }

        assert_ne!(monkey.if_true, monkey.id);
        assert_ne!(monkey.if_false, monkey.id);

        return monkey;
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(|s| Monkey::from_str(s)).collect()
}

fn inspection_round(monkeys: &mut Vec<Monkey>, intermediate_operation: &Operation) {
    let mut passed_items: Vec<Vec<i64>> = monkeys.iter().map(|_| Vec::new()).collect();

    let divisors = monkeys
        .iter()
        .map(|m| match m.test {
            Test::DivisibleBy(divisor) => divisor,
        })
        .collect::<Vec<i64>>();

    let mut denominator: i64 = 1;
    for divisor in divisors.iter() {
        denominator = lcm(denominator, *divisor);
    }

    // println!("Denominator: {}", denominator);

    for (monkey_id, monkey) in monkeys.iter_mut().enumerate() {
        // move passed items to the current monkey
        if monkey_id != monkey.id {
            panic!("Monkey ID mismatch: {} != {}", monkey_id, monkey.id);
        }
        for item in passed_items[monkey_id].iter() {
            monkey.items.push(*item);
        }
        passed_items[monkey_id].clear();

        // three steps:
        // 1. inspect = apply operation
        // 2. perform intermediate operation, e.g. divide by 3
        // 3. test, throw to target

        // inspect items
        for item in monkey.items.iter() {
            monkey.items_inspected += 1;

            // 1. inspect
            let new_value = monkey.operation.apply(*item) % denominator;

            // 2. perform operation
            let divided_new_value = intermediate_operation.apply(new_value);

            // 3. test, throw to target
            let test_result = monkey.test.perform(divided_new_value);
            let target = match test_result {
                true => monkey.if_true,
                false => monkey.if_false,
            };

            passed_items[target].push(divided_new_value);
        }

        // clear items
        monkey.items.clear();
    }

    // move remaining passed items to the according monkeys
    for (monkey_id, items) in passed_items.iter().enumerate() {
        for item in items.iter() {
            monkeys[monkey_id].items.push(*item);
        }
    }
}

fn inspection_rounds_part1(monkeys: &mut Vec<Monkey>, rounds: usize) {
    let op = Operation::Divide(3);
    inspection_rounds_impl(monkeys, rounds, &op)
}

fn inspection_rounds_part2(monkeys: &mut Vec<Monkey>, rounds: usize) {
    let op = Operation::Identity();
    inspection_rounds_impl(monkeys, rounds, &op)
}

fn inspection_rounds_impl(
    monkeys: &mut Vec<Monkey>,
    rounds: usize,
    intermediate_operation: &Operation,
) {
    // print_monkeys(monkeys);
    for _ in 0..rounds {
        inspection_round(monkeys, intermediate_operation);
        // print_monkeys(monkeys);
    }
}

fn monkey_business_level(monkeys: &Vec<Monkey>) -> usize {
    let mut items_inspected = monkeys
        .iter()
        .map(|m| m.items_inspected)
        .collect::<Vec<_>>();
    items_inspected.sort();
    items_inspected.reverse();

    return items_inspected[0] * items_inspected[1];
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

fn lcm(a: i64, b: i64) -> i64 {
    return a * b / gcd(a, b);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part1() {
        let monkeys = parse_monkeys(INPUT);
        assert_eq!(monkeys.len(), 4);
        for (id, monkey) in monkeys.iter().enumerate() {
            assert_eq!(monkey.id, id);
        }

        let first_monkey = &monkeys[0];
        assert_eq!(first_monkey.items, vec![79, 98]);
        assert_eq!(first_monkey.operation.apply(1), 19);
        assert_eq!(first_monkey.operation.apply(2), 2 * 19);
        assert_eq!(first_monkey.test.perform(23), true);
        assert_eq!(first_monkey.test.perform(2), false);
        assert_eq!(first_monkey.if_true, 2);
        assert_eq!(first_monkey.if_false, 3);

        // run 20 rounds
        let mut monkeys = parse_monkeys(INPUT);
        inspection_rounds_part1(&mut monkeys, 20);
        assert_eq!(monkeys[0].items_inspected, 101);
        assert_eq!(monkeys[1].items_inspected, 95);
        assert_eq!(monkeys[2].items_inspected, 7);
        assert_eq!(monkeys[3].items_inspected, 105);

        assert_eq!(monkey_business_level(&monkeys), 101 * 105);

        assert_eq!(lcm(1, 1), 1);
        assert_eq!(lcm(2, 4), 4);
        assert_eq!(lcm(11, 13), 11 * 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(10_000, 10000);

        let mut monkeys = parse_monkeys(INPUT);
        inspection_rounds_part2(&mut monkeys, 10_000);

        assert_eq!(monkey_business_level(&monkeys), 2_713_310_158);
    }
}
