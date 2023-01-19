use std::{env, fs};

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
}
