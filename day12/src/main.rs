use std::collections::VecDeque;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let map = Map::from_str(input);
    map.find_path()
}

fn part2(_input: &str) -> usize {
    0
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

fn is_valid_move(from: char, to: char) -> bool {
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

    fn viable_neighbors(&self, pos: Pos) -> Vec<Pos> {
        let mut neighbors = Vec::new();

        let height = self.get(pos);
        if height.is_none() {
            return neighbors;
        }

        let from = height.unwrap();

        if pos.0 > 0 {
            if let Some(to) = self.get((pos.0 - 1, pos.1)) {
                if is_valid_move(from, to) {
                    neighbors.push((pos.0 - 1, pos.1));
                }
            }
        }
        if let Some(to) = self.get((pos.0 + 1, pos.1)) {
            if is_valid_move(from, to) {
                neighbors.push((pos.0 + 1, pos.1));
            }
        }
        if pos.1 > 0 {
            if let Some(to) = self.get((pos.0, pos.1 - 1)) {
                if is_valid_move(from, to) {
                    neighbors.push((pos.0, pos.1 - 1));
                }
            }
        }
        if let Some(to) = self.get((pos.0, pos.1 + 1)) {
            if is_valid_move(from, to) {
                neighbors.push((pos.0, pos.1 + 1));
            }
        }

        neighbors
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

    fn find_path(&self) -> usize {
        let start = self.find('S').unwrap();
        let end = self.find('E').unwrap();

        let max_distance = self.size.0 * self.size.1 + 1;

        let mut distance = self.init_grid(max_distance);
        distance[start.0][start.1] = 0;

        let mut queue: VecDeque<Pos> = VecDeque::new();
        queue.push_back(start);

        while !queue.is_empty() {
            let pos = queue.pop_front().unwrap();

            let current_distance = distance[pos.0][pos.1];
            if pos == end {
                return current_distance;
            }

            let next_distance = current_distance + 1;

            let neighbors = self.viable_neighbors(pos);
            for neighbor in neighbors {
                if distance[neighbor.0][neighbor.1] > next_distance {
                    distance[neighbor.0][neighbor.1] = next_distance;
                    queue.push_back(neighbor);
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

        assert!(is_valid_move('a', 'a'));
        assert!(is_valid_move('a', 'b'));
        assert!(is_valid_move('b', 'a'));

        assert!(is_valid_move('S', 'b'));
        assert!(is_valid_move('y', 'E'));
        assert!(!is_valid_move('x', 'E'));

        assert!(!is_valid_move('a', 'c'));

        assert_eq!(map.viable_neighbors((0, 0)), vec![(1, 0), (0, 1)]);
        assert_eq!(
            map.viable_neighbors((1, 1)),
            vec![(0, 1), (2, 1), (1, 0), (1, 2)]
        );

        assert_eq!(map.find('S'), Some((0, 0)));
        assert_eq!(map.find('E'), Some((2, 5)));

        assert_eq!(map.find_path(), 31);
    }

    #[test]
    fn test_part2() {}
}
