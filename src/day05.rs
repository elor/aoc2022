use std::fs;

#[allow(dead_code)]
fn main() {
    let input = fs::read_to_string("res/day05.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let (init, moves) = split_into_moves_and_init(input);
    let mut dock = dock_from_string(&init);
    apply_crate_moves_lines(&mut dock, &moves);
    return get_top_string(&dock);
}

fn part2(_input: &str) -> String {
    let (init, moves) = split_into_moves_and_init(_input);
    let mut dock = dock_from_string(&init);
    apply_crate_moves_lines_inorder(&mut dock, &moves);
    return get_top_string(&dock);
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
        .lines()
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
        .lines()
        .for_each(|line| apply_crate_moves_line_inorder(dock, line));
}

fn dock_from_string(s: &str) -> Day05Dock {
    let mut dock = Day05Dock::new();

    let mut lines = s.trim_matches('\n').lines().collect::<Vec<&str>>();

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
mod day05 {
    use super::*;

    #[test]
    fn test_part1() {
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
    fn test_part2() {
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
