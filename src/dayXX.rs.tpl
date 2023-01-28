use std::fs;

fn main() {
    let input = fs::read_to_string("res/day01.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    -1
}

fn part2(input: &str) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
    }

    #[test]
    fn test_part2() {
    }
}
