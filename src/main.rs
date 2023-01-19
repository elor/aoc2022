use std::{collections::HashSet, env, fs};

fn main() {
    // parse './main day input.txt'
    let args: Vec<String> = env::args().collect();
    let day = args[1].as_str();
    let input_filename = args[2].as_str();

    // read input file
    let input = fs::read_to_string(input_filename).expect("Something went wrong reading the file");

    match day {
        "day01" => {
            let result = day01(&input);
            println!("Result of day01: {}", result);
        }
        "day01_2" => {
            let result = day01_part2(&input);
            println!("Result of day01_part2: {}", result);
        }
        "day02" => {
            let result = day02(&input);
            println!("Result of day02: {}", result);
        }
        "day02_2" => {
            let result = day02_part2(&input);
            println!("Result of day02_part2: {}", result);
        }
        "day03" => {
            let result = day03(&input);
            println!("Result of day03: {}", result);
        }
        "day03_2" => {
            let result = day03_part2(&input);
            println!("Result of day03_part2: {}", result);
        }

        _ => panic!("Day not implemented"),
    };
}

fn day01(input: &str) -> i32 {
    let sums = input
        .trim()
        .split("\n\n")
        .map(|s| s.split("\n").map(|v| v.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();

    // max of sums
    sums.iter().max().unwrap().clone()
}

fn day01_part2(input: &str) -> i32 {
    let mut sums = input
        .trim()
        .split("\n\n")
        .map(|s| s.split("\n").map(|v| v.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();

    sums.sort();
    sums.reverse();

    sums[0] + sums[1] + sums[2]
}

fn day02(input: &str) -> i32 {
    enum Hand {
        Rock,
        Paper,
        Scissors,
    }

    fn to_hand(s: char) -> Hand {
        match s {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissors,
            'X' => Hand::Rock,
            'Y' => Hand::Paper,
            'Z' => Hand::Scissors,
            _ => panic!("Invalid hand"),
        }
    }

    fn score(h1: &Hand, h2: &Hand) -> i32 {
        match (h1, h2) {
            (Hand::Rock, Hand::Rock) => 3,
            (Hand::Rock, Hand::Paper) => 6,
            (Hand::Rock, Hand::Scissors) => 0,
            (Hand::Paper, Hand::Rock) => 0,
            (Hand::Paper, Hand::Paper) => 3,
            (Hand::Paper, Hand::Scissors) => 6,
            (Hand::Scissors, Hand::Rock) => 6,
            (Hand::Scissors, Hand::Paper) => 0,
            (Hand::Scissors, Hand::Scissors) => 3,
        }
    }

    fn points(h: &Hand) -> i32 {
        match h {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    input
        .trim()
        .split("\n")
        .map(|s| {
            let opponent = to_hand(s.chars().nth(0).unwrap());
            let me = to_hand(s.chars().nth(2).unwrap());

            points(&me) + score(&opponent, &me)
        })
        .sum()
}

fn day02_part2(input: &str) -> i32 {
    enum Hand {
        Rock,
        Paper,
        Scissors,
    }

    fn to_hand(s: char) -> Hand {
        match s {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissors,
            'X' => Hand::Rock,
            'Y' => Hand::Paper,
            'Z' => Hand::Scissors,
            _ => panic!("Invalid hand"),
        }
    }

    fn score(h1: &Hand, h2: &Hand) -> i32 {
        match (h1, h2) {
            (Hand::Rock, Hand::Rock) => 3,
            (Hand::Rock, Hand::Paper) => 6,
            (Hand::Rock, Hand::Scissors) => 0,
            (Hand::Paper, Hand::Rock) => 0,
            (Hand::Paper, Hand::Paper) => 3,
            (Hand::Paper, Hand::Scissors) => 6,
            (Hand::Scissors, Hand::Rock) => 6,
            (Hand::Scissors, Hand::Paper) => 0,
            (Hand::Scissors, Hand::Scissors) => 3,
        }
    }

    fn points(h: &Hand) -> i32 {
        match h {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn get_strategy(opponent: &Hand, strategy: char) -> Hand {
        match strategy {
            'X' => match opponent {
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
            },
            'Y' => match opponent {
                Hand::Rock => Hand::Rock,
                Hand::Paper => Hand::Paper,
                Hand::Scissors => Hand::Scissors,
            },
            'Z' => match opponent {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
            },
            _ => panic!("Invalid strategy"),
        }
    }

    input
        .trim()
        .split("\n")
        .map(|s| {
            let opponent = to_hand(s.chars().nth(0).unwrap());
            let strategy = s.chars().nth(2).unwrap();

            let me = get_strategy(&opponent, strategy);

            points(&me) + score(&opponent, &me)
        })
        .sum()
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

    let mut intersection: Vec<char> = a_hash
        .intersection(&b_hash)
        .cloned()
        .collect();

    intersection.retain(|c| c_hash.contains(c));

    if intersection.len() == 1 {
        return intersection.pop().unwrap();
    }

    return '\0';
}

fn day03(s: &str) -> i32 {
    s.trim()
        .split("\n")
        .map(|line| find_duplicate_in_string_halves(line))
        .map(|c| day03_priority(c))
        .sum()
}

fn day03_part2(s: &str) -> i32 {
    let mut lines = s.trim()
    .split("\n").collect::<Vec<&str>>();

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
    fn test_day01() {
        let s = "1";
        assert_eq!(day01(s), 1 as i32);

        let s = "1\n";
        assert_eq!(day01(s), 1 as i32);

        let s = "1\n2";
        assert_eq!(day01(s), 3 as i32);

        let s = "1\n\n2";
        assert_eq!(day01(s), 2 as i32);

        let s = "1\n\n2";
        assert_eq!(day01(s), 2 as i32);
    }

    #[test]
    fn test_day01_part2() {
        let s = "1\n\n1\n\n1";
        assert_eq!(day01_part2(s), 3 as i32);

        let s = "1\n\n2\n\n3\n\n4";
        assert_eq!(day01_part2(s), 9 as i32);
    }

    #[test]
    fn test_day02() {
        let s = "A X";
        assert_eq!(day02(s), 4 as i32);
    }

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

        assert_eq!(day03(test_data), 157 as i32);
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
        assert_eq!(day03_part2(test_data), 70 as i32);
    }
}
