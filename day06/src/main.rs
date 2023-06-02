use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn sliding_window_first_appearance(input: &str, window_size: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .enumerate()
        .filter(|(_, w)| {
            let mut chars = w.iter().map(|c| c.to_string()).collect::<Vec<String>>();
            chars.sort();
            chars.dedup();
            chars.len() == window_size
        })
        .map(|(i, _)| i)
        .next()
        .unwrap()
        + window_size
}

fn part1(input: &str) -> usize {
    sliding_window_first_appearance(input, 4)
}

fn part2(input: &str) -> usize {
    sliding_window_first_appearance(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // detect four completely different characters in the stream
        // Specifically, it needs to report the number of characters from the beginning of the buffer to the end of the first such four-character marker.
        let test_data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part1(test_data), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part2() {}
}
