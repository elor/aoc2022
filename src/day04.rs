use std::fs;

#[allow(dead_code)]
fn main() {
    let input = fs::read_to_string("res/day04.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

#[derive(Debug)]
struct Range(i32, i32);

// PartialEq trait
impl PartialEq for Range {
    fn eq(&self, other: &Range) -> bool {
        return self.0 == other.0 && self.1 == other.1;
    }
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        return other.0 >= self.0 && other.1 <= self.1;
    }

    fn overlaps(&self, other: &Range) -> bool {
        return (self.0 <= other.1 && self.1 >= other.0)
            || (self.0 >= other.1 && self.1 <= other.1);
    }
}

fn split_into_range(s: &str) -> Range {
    let mut split = s.split("-");

    let min = split.next().unwrap().parse::<i32>().unwrap();
    let max = split.next().unwrap().parse::<i32>().unwrap();

    return Range(min, max);
}

fn split_ranges_line(s: &str) -> (Range, Range) {
    let mut split = s.split(",");

    let first = split.next().unwrap();
    let second = split.next().unwrap();

    return (split_into_range(first), split_into_range(second));
}

fn is_fully_contained(left: &Range, right: &Range) -> bool {
    left.contains(&right) || right.contains(&left)
}

fn part1(input: &str) -> i32 {
    input
        .trim()
        .split("\n")
        .map(|line| split_ranges_line(line))
        .filter(|(left, right)| is_fully_contained(left, right))
        .count() as i32
}

fn part2(input: &str) -> i32 {
    input
        .trim()
        .split("\n")
        .map(|line| split_ranges_line(line))
        .filter(|(left, right)| left.overlaps(right))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day04() {
        assert_eq!(split_into_range("6-6"), Range(6, 6));

        assert_eq!(split_ranges_line("1-3,6-6"), (Range(1, 3), Range(6, 6)));
        assert_eq!(split_ranges_line("1-2,3-4"), (Range(1, 2), Range(3, 4)));

        assert_eq!(is_fully_contained(&Range(1, 7), &Range(2, 2)), true);
        assert_eq!(is_fully_contained(&Range(1, 1), &Range(2, 2)), false);

        let test_data = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

        assert_eq!(part1(test_data), 2 as i32);
    }

    #[test]
    fn test_day04_part2() {
        assert_eq!(Range(1, 5).overlaps(&Range(5, 10)), true);
        assert_eq!(Range(1, 5).overlaps(&Range(6, 10)), false);

        assert_eq!(Range(1, 1).overlaps(&Range(1, 2)), true);
        assert_eq!(Range(10, 10).overlaps(&Range(1, 2)), false);

        assert_eq!(Range(5, 5).overlaps(&Range(1, 9)), true);
        assert_eq!(Range(1, 9).overlaps(&Range(5, 5)), true);
    }
}
