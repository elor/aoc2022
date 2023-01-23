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
        "day04" => {
            let result = day04(&input);
            println!("Result of day04: {}", result);
        }
        "day04_2" => {
            let result = day04_part2(&input);
            println!("Result of day04_part2: {}", result);
        }
        "day05" => {
            let result = day05(&input);
            println!("Result of day05: {}", result);
        }
        "day05_2" => {
            let result = day05_part2(&input);
            println!("Result of day05_part2: {}", result);
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

    let mut intersection: Vec<char> = a_hash.intersection(&b_hash).cloned().collect();

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
    let mut lines = s.trim().split("\n").collect::<Vec<&str>>();

    let mut prio_sum = 0;

    while lines.len() != 0 {
        let a = lines.pop().unwrap();
        let b = lines.pop().unwrap();
        let c = lines.pop().unwrap();

        prio_sum += day03_priority(find_triplicate_in_string_thirds(a, b, c));
    }

    return prio_sum;
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

fn day04(input: &str) -> i32 {
    input
        .trim()
        .split("\n")
        .map(|line| split_ranges_line(line))
        .filter(|(left, right)| is_fully_contained(left, right))
        .count() as i32
}

fn day05(input: &str) -> String {
    let (init, moves) = split_into_moves_and_init(input);
    let mut dock = dock_from_string(&init);
    apply_crate_moves_lines(&mut dock, &moves);
    return get_top_string(&dock);
}

fn day05_part2(_input: &str) -> String {
    let (init, moves) = split_into_moves_and_init(_input);
    let mut dock = dock_from_string(&init);
    apply_crate_moves_lines_inorder(&mut dock, &moves);
    return get_top_string(&dock);
}

fn day04_part2(input: &str) -> i32 {
    input
        .trim()
        .split("\n")
        .map(|line| split_ranges_line(line))
        .filter(|(left, right)| left.overlaps(right))
        .count() as i32
}

type Day05Stack = String;
type Day05Dock = Vec<Day05Stack>;

fn get_top_string(dock: &Day05Dock) -> String {
    dock.iter()
        .map(|stack| stack.chars().last().unwrap())
        .collect()
}

fn move_crates(dock: &mut Day05Dock, count: usize, from: usize, to: usize) {
    for _ in 0..count {
        if dock[from - 1].len() <= 1 {
            return;
        }

        let top = dock[from - 1].pop().unwrap();
        dock[to - 1].push(top);
    }
}

fn apply_crate_moves_line(dock: &mut Day05Dock, instruction: &str) {
    let mut split = instruction.split(" ");

    let _move = split.next().unwrap();
    let count = split.next().unwrap().parse::<usize>().unwrap();
    let _from = split.next().unwrap();
    let from = split.next().unwrap().parse::<usize>().unwrap();
    let _to = split.next().unwrap();
    let to = split.next().unwrap().parse::<usize>().unwrap();

    move_crates(dock, count, from, to);
}

fn apply_crate_moves_lines(dock: &mut Day05Dock, input: &str) {
    input
        .trim()
        .split("\n")
        .for_each(|line| apply_crate_moves_line(dock, line));
}

fn move_crates_inorder(dock: &mut Day05Dock, count: usize, from: usize, to: usize) {
    assert_ne!(from, to);

    let mut split_pos = dock[from - 1].len() - count;
    if split_pos <= 0 {
        split_pos = 1;
    }

    let binding = dock[from - 1].clone();
    let (front, back) = binding.split_at(split_pos);
    
    dock[from - 1] = front.to_string();
    dock[to - 1].push_str(back);
}

fn apply_crate_moves_line_inorder(dock: &mut Day05Dock, instruction: &str) {
    let mut split = instruction.split(" ");

    let _move = split.next().unwrap();
    let count = split.next().unwrap().parse::<usize>().unwrap();
    let _from = split.next().unwrap();
    let from = split.next().unwrap().parse::<usize>().unwrap();
    let _to = split.next().unwrap();
    let to = split.next().unwrap().parse::<usize>().unwrap();

    move_crates_inorder(dock, count, from, to);
}

fn apply_crate_moves_lines_inorder(dock: &mut Day05Dock, input: &str) {
    input
        .trim()
        .split("\n")
        .for_each(|line| apply_crate_moves_line_inorder(dock, line));
}

fn dock_from_string(s: &str) -> Day05Dock {
    let mut dock = Day05Dock::new();

    let mut lines = s.trim_matches('\n').split("\n").collect::<Vec<&str>>();

    let numberline = lines.pop().unwrap();
    let number_of_stacks = numberline.split(" ").filter(|s| s.len() > 0).count();
    dock.resize(number_of_stacks, " ".to_string());

    lines.reverse();
    for line in lines {
        let chars = line.chars().skip(1).step_by(4);

        for (i, c) in chars.enumerate() {
            if c != ' ' {
                dock[i].push(c);
            }
        }
    }

    return dock;
}

fn split_into_moves_and_init(s: &str) -> (String, String) {
    let mut split = s.trim_matches('\n').split("\n\n");

    let moves = split.next().unwrap();
    let init = split.next().unwrap();

    return (moves.to_string(), init.to_string());
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

        assert_eq!(day04(test_data), 2 as i32);
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

    #[test]
    fn test_day05() {
        let dock = Day05Dock::new();
        assert_eq!(dock.len(), 0 as usize);

        let dock = dock_from_string("[A]\n1");
        assert_eq!(dock.len(), 1 as usize);
        assert_eq!(get_top_string(&dock), "A");

        let test_data = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        let (init, moves) = split_into_moves_and_init(test_data);

        let mut dock = dock_from_string(&init);
        assert_eq!(dock.len(), 3 as usize);
        assert_eq!(get_top_string(&dock), "NDP");

        move_crates(&mut dock, 1, 2, 1);
        assert_eq!(get_top_string(&dock), "DCP");

        let mut dock = dock_from_string(&init);
        apply_crate_moves_line(&mut dock, "move 1 from 2 to 1");
        assert_eq!(get_top_string(&dock), "DCP");
        apply_crate_moves_line(&mut dock, "move 3 from 1 to 3");
        assert_eq!(get_top_string(&dock), " CZ");
        apply_crate_moves_line(&mut dock, "move 2 from 2 to 1");
        assert_eq!(get_top_string(&dock), "M Z");
        apply_crate_moves_line(&mut dock, "move 1 from 1 to 2");
        assert_eq!(get_top_string(&dock), "CMZ");

        let mut dock = dock_from_string(&init);
        apply_crate_moves_lines(&mut dock, &moves);
        assert_eq!(get_top_string(&dock), "CMZ");
    }

    #[test]
    fn test_day05_part2() {
        let test_data = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
        let (init, moves) = split_into_moves_and_init(test_data);
        let mut dock = dock_from_string(&init);
        apply_crate_moves_lines_inorder(&mut dock, &moves);
        assert_eq!(get_top_string(&dock), "MCD");
    }
}
