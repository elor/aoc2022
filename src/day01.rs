use std::fs;

#[allow(dead_code)]
fn main() {
    let input = fs::read_to_string("res/day01.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let sums = input
        .trim()
        .split("\n\n")
        .map(|s| s.lines().map(|v| v.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();

    // max of sums
    sums.iter().max().unwrap().clone()
}

fn part2(input: &str) -> i32 {
    let mut sums = input
        .trim()
        .split("\n\n")
        .map(|s| s.lines().map(|v| v.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();

    sums.sort();
    sums.reverse();

    sums[0] + sums[1] + sums[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() {
        let s = "1";
        assert_eq!(part1(s), 1 as i32);

        let s = "1\n";
        assert_eq!(part1(s), 1 as i32);

        let s = "1\n2";
        assert_eq!(part1(s), 3 as i32);

        let s = "1\n\n2";
        assert_eq!(part1(s), 2 as i32);

        let s = "1\n\n2";
        assert_eq!(part1(s), 2 as i32);
    }
}
