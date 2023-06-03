use std::collections::VecDeque;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let map = Map::from_str(input);
    map.ascend_to_E()
}

fn part2(input: &str) -> usize {
    let map = Map::from_str(input);
    map.descend_from_E()
}

struct Map {
    grid: Vec<Vec<char>>,
    size: (usize, usize),
}

type Pos = (usize, usize);

fn height_char_to_u8(c: char) -> u8 {
    match c {
        'S' => 'a' as u8,
        'E' => 'z' as u8,
        _ => c as u8,
    }
}

fn is_valid_ascent(from: char, to: char) -> bool {
    let from_u8 = height_char_to_u8(from);
    let to_u8 = height_char_to_u8(to);

    return from_u8 + 1 >= to_u8;
}

impl Map {
    fn from_str(input: &str) -> Map {
        let grid = input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let size = (grid.len(), grid[0].len());

        // assert that all lines are the same length
        assert!(grid.iter().all(|line| line.len() == size.1));

        Map { grid, size }
    }

    fn init_grid<T: Copy>(&self, value: T) -> Vec<Vec<T>> {
        let size = self.size;

        let mut grid = Vec::with_capacity(size.0);
        for _ in 0..size.0 {
            let mut line = Vec::with_capacity(size.1);
            line.resize_with(size.1, || value);
            grid.push(line);
        }
        grid
    }

    fn get(&self, pos: Pos) -> Option<char> {
        if pos.0 >= self.size.0 || pos.1 >= self.size.1 {
            None
        } else {
            Some(self.grid[pos.0][pos.1])
        }
    }

    fn get_neighbors(&self, pos: Pos) -> Vec<Pos> {
        let mut neighbors = Vec::new();

        if pos.0 > 0 {
            neighbors.push((pos.0 - 1, pos.1));
        }
        if pos.0 < self.size.0 - 1 {
            neighbors.push((pos.0 + 1, pos.1));
        }
        if pos.1 > 0 {
            neighbors.push((pos.0, pos.1 - 1));
        }
        if pos.1 < self.size.1 - 1 {
            neighbors.push((pos.0, pos.1 + 1));
        }

        return neighbors;
    }

    fn find(&self, c: char) -> Option<Pos> {
        for (i, line) in self.grid.iter().enumerate() {
            for (j, &field) in line.iter().enumerate() {
                if field == c {
                    return Some((i, j));
                }
            }
        }

        None
    }

    fn ascend_to_E(&self) -> usize {
        let start = self.find('S').unwrap();
        let end = self.find('E').unwrap();

        let mut open = self.init_grid(true);

        let mut queue: VecDeque<(Pos, usize)> = VecDeque::new();
        queue.push_back((start, 0));
        open[start.0][start.1] = false;

        while !queue.is_empty() {
            let (pos, current_distance) = queue.pop_front().unwrap();

            assert!(!open[pos.0][pos.1]);

            if pos == end {
                return current_distance;
            }

            let next_distance = current_distance + 1;

            let from_height = self.get(pos).unwrap();

            let neighbors = self.get_neighbors(pos);
            for neighbor in neighbors {
                let to_height = self.get(neighbor).unwrap();

                if open[neighbor.0][neighbor.1] && is_valid_ascent(from_height, to_height) {
                    queue.push_back((neighbor, next_distance));
                    open[neighbor.0][neighbor.1] = false;
                }
            }
        }

        panic!("No path found");
    }

    fn descend_from_E(&self) -> usize {
        let end = self.find('E').unwrap();

        let mut open = self.init_grid(true);

        let mut queue: VecDeque<(Pos, usize)> = VecDeque::new();
        queue.push_back((end, 0));
        open[end.0][end.1] = false;

        while !queue.is_empty() {
            let (pos, current_distance) = queue.pop_front().unwrap();

            assert!(!open[pos.0][pos.1]);

            match self.get(pos) {
                Some('S') => return current_distance,
                Some('a') => return current_distance,
                Some(_) => {}
                None => panic!("Invalid position during descent"),
            }

            let next_distance = current_distance + 1;
            let to_height = self.get(pos).unwrap();

            let neighbors = self.get_neighbors(pos);
            for neighbor in neighbors {
                let from_height = self.get(neighbor).unwrap();

                if open[neighbor.0][neighbor.1] && is_valid_ascent(from_height, to_height) {
                    queue.push_back((neighbor, next_distance));
                    open[neighbor.0][neighbor.1] = false;
                }
            }
        }

        panic!("No path found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() {
        let map = Map::from_str(INPUT);
        assert_eq!(map.size, (5, 8));

        assert_eq!(map.get((0, 0)), Some('S'));
        assert_eq!(map.get((0, 1)), Some('a'));
        assert_eq!(map.get((0, 2)), Some('b'));

        assert!(is_valid_ascent('a', 'a'));
        assert!(is_valid_ascent('a', 'b'));
        assert!(is_valid_ascent('b', 'a'));

        assert!(is_valid_ascent('S', 'b'));
        assert!(is_valid_ascent('y', 'E'));
        assert!(!is_valid_ascent('x', 'E'));

        assert!(!is_valid_ascent('a', 'c'));

        assert_eq!(map.get_neighbors((0, 0)), vec![(1, 0), (0, 1)]);
        assert_eq!(
            map.get_neighbors((1, 1)),
            vec![(0, 1), (2, 1), (1, 0), (1, 2)]
        );

        assert_eq!(map.find('S'), Some((0, 0)));
        assert_eq!(map.find('E'), Some((2, 5)));

        assert_eq!(map.ascend_to_E(), 31);
    }

    #[test]
    fn test_part2() {
        let map = Map::from_str(INPUT);
        assert_eq!(map.descend_from_E(), 29);
    }
}
