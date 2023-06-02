use std::collections::HashSet;
use std::fs;

#[allow(dead_code)]
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn day03_priority(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 'a' as i32 + 1,
        'A'..='Z' => c as i32 - 'A' as i32 + 27,
        _ => -1,
    }
}

fn split_string_in_two(s: &str) -> (String, String) {
    let string = s.to_string();

    let top_half = string.get(0..string.len() / 2).unwrap();
    let bottom_half = string.get(string.len() / 2..).unwrap();

    return (top_half.to_string(), bottom_half.to_string());
}

fn find_duplicate_in_string_halves(s: &str) -> char {
    let (top_half, bottom_half) = split_string_in_two(s);

    let top_hash: HashSet<char> = HashSet::from_iter(top_half.chars());
    let bottom_hash: HashSet<char> = HashSet::from_iter(bottom_half.chars());

    let mut intersection: Vec<char> = top_hash.intersection(&bottom_hash).cloned().collect();

    if intersection.len() == 1 {
        return intersection.pop().unwrap();
    }

    return '\0';
}

fn find_triplicate_in_string_thirds(a: &str, b: &str, c: &str) -> char {
    let a_hash: HashSet<char> = HashSet::from_iter(a.chars());
    let b_hash: HashSet<char> = HashSet::from_iter(b.chars());
    let c_hash: HashSet<char> = HashSet::from_iter(c.chars());

    let mut intersection: Vec<char> = a_hash.intersection(&b_hash).cloned().collect();

    intersection.retain(|c| c_hash.contains(c));

    if intersection.len() == 1 {
        return intersection.pop().unwrap();
    }

    return '\0';
}

fn part1(s: &str) -> i32 {
    s.trim()
        .lines()
        .map(|line| find_duplicate_in_string_halves(line))
        .map(|c| day03_priority(c))
        .sum()
}

fn part2(s: &str) -> i32 {
    let mut lines = s.trim().lines().collect::<Vec<&str>>();

    let mut prio_sum = 0;

    while lines.len() != 0 {
        let a = lines.pop().unwrap();
        let b = lines.pop().unwrap();
        let c = lines.pop().unwrap();

        prio_sum += day03_priority(find_triplicate_in_string_thirds(a, b, c));
    }

    return prio_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day03() {
        assert_eq!(day03_priority('a'), 1 as i32);
        assert_eq!(day03_priority('b'), 2 as i32);
        assert_eq!(day03_priority('Z'), 52 as i32);

        assert_eq!(find_duplicate_in_string_halves("aa"), 'a');
        assert_eq!(find_duplicate_in_string_halves("bsb"), 'b');

        assert_eq!(
            find_duplicate_in_string_halves("vJrwpWtwJgWrhcsFMMfFFhFp"),
            'p'
        );
        assert_eq!(day03_priority('p'), 16 as i32);
        assert_eq!(day03_priority('P'), 42 as i32);

        let test_data = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

        assert_eq!(part1(test_data), 157 as i32);
    }

    #[test]
    fn test_day03_part2() {
        assert_eq!(find_triplicate_in_string_thirds("a", "a", "a"), 'a');

        let test_data = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;
        assert_eq!(part2(test_data), 70 as i32);
    }
}
