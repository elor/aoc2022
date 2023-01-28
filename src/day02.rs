mod day03;

use std::fs;

#[allow(dead_code)]
fn main() {
    let input = fs::read_to_string("res/day02.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
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

fn part2(input: &str) -> i32 {
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
    fn test_day02() {
        let s = "A X";
        assert_eq!(part1(s), 4 as i32);
    }
}
