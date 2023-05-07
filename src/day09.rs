use itertools::Itertools;
use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("res/day09.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    count_visited(input, 1) as i32
}

fn part2(input: &str) -> i32 {
    count_visited(input, 9) as i32
}

// a pair of coordinates
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

type Movement = (char, i32);

// a set of visited points, as a rust typedef
type Visited = HashSet<Point>;

struct Grid {
    visited: Visited,
    head_visited: Visited,
    rope: Vec<Point>,
    print_boundary: Boundary,
}

impl Grid {
    fn new(length: usize) -> Grid {
        let mut grid = Grid {
            visited: Visited::new(),
            head_visited: Visited::new(),
            rope: Vec::new(),
            print_boundary: Boundary::new(),
        };

        assert!(length > 0);

        for _ in 0..=length {
            grid.rope.push(Point { x: 0, y: 0 });
        }

        grid.visit();

        grid
    }

    fn visit(&mut self) {
        for point in self.rope.iter() {
            self.print_boundary.add(point);
        }
        self.head_visited.insert(self.rope.first().unwrap().clone());
        self.visited.insert(self.rope.last().unwrap().clone());
    }

    fn move_single(&mut self, direction: char) {
        let head = self.rope.first_mut().unwrap();
        match direction {
            'U' => head.y += 1,
            'D' => head.y -= 1,
            'R' => head.x += 1,
            'L' => head.x -= 1,
            _ => panic!("Invalid direction: {}", direction),
        };

        self.fix_rope();

        self.visit();
    }

    fn fix_rope_pair(leading: &Point, trailing: &mut Point) {
        let dx = leading.x - trailing.x;
        let dy = leading.y - trailing.y;

        if dx > 1 {
            trailing.x = leading.x - 1;
            if dy == dx {
                trailing.y = leading.y - 1;
            } else if dy == -dx {
                trailing.y = leading.y + 1;
            } else {
                trailing.y = leading.y;
            }
        } else if dx < -1 {
            trailing.x = leading.x + 1;
            if dy == dx {
                trailing.y = leading.y + 1;
            } else if dy == -dx {
                trailing.y = leading.y - 1;
            } else {
                trailing.y = leading.y;
            }
        } else if dy > 1 {
            trailing.y = leading.y - 1;
            if dx == dy {
                trailing.x = leading.x - 1;
            } else if dx == -dy {
                trailing.x = leading.x + 1;
            } else {
                trailing.x = leading.x;
            }
        } else if dy < -1 {
            trailing.y = leading.y + 1;
            if dx == dy {
                trailing.x = leading.x + 1;
            } else if dx == -dy {
                trailing.x = leading.x - 1;
            } else {
                trailing.x = leading.x;
            }
        }
    }

    fn fix_rope(&mut self) {
        for i in 0..self.rope.len() - 1 {
            let leading = self.rope[i];
            let mut trailing = self.rope.get_mut(i + 1).unwrap();
            Grid::fix_rope_pair(&leading, &mut trailing)
        }
    }

    fn move_amount(&mut self, direction: char, amount: i32) {
        for _ in 0..amount {
            self.move_single(direction);
        }
    }

    fn count(&self) -> usize {
        self.visited.len()
    }

    fn split_movements(list: &str) -> Vec<Movement> {
        list.split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                let (direction, amount) = line.split(' ').next_tuple().unwrap();

                (
                    direction.chars().next().unwrap(),
                    amount.parse::<i32>().unwrap(),
                )
            })
            .collect()
    }

    fn move_list(&mut self, list: &str) {
        let movements = Grid::split_movements(list);

        for (direction, amount) in movements {
            self.move_amount(direction, amount);
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // 2D grid of '.' with size of the boundary
        let mut grid =
            vec![
                vec!['.'; (self.print_boundary.max.y - self.print_boundary.min.y + 2) as usize];
                (self.print_boundary.max.x - self.print_boundary.min.x + 1) as usize
            ];

        let off = Point {
            x: -self.print_boundary.min.x,
            y: -self.print_boundary.min.y,
        };

        // mark (0,0) with 's', mind the min_x and min_y
        grid[0 + off.y as usize][0 + off.x as usize] = 's';

        // mark the visited points with 'H', '1', '2', ...
        for (i, point) in self.rope.iter().enumerate().rev() {
            let c = match i {
                0 => 'H',
                1..=9 => (i as u8 + b'0') as char,
                _ => '#',
            };
            grid[(point.y + off.y) as usize][(point.x + off.x) as usize] = c;
        }

        writeln!(f, "").unwrap();
        for row in grid.iter().rev() {
            for cell in row {
                write!(f, "{}", cell).unwrap();
            }
            writeln!(f, "").unwrap();
        }

        Ok(())
    }
}

fn count_visited(list: &str, rope_length: usize) -> usize {
    let mut grid = Grid::new(rope_length);

    grid.move_list(list);

    grid.count()
}

struct Boundary {
    min: Point,
    max: Point,
}

impl Boundary {
    fn new() -> Boundary {
        Boundary {
            min: Point { x: 0, y: 0 },
            max: Point { x: 0, y: 0 },
        }
    }

    fn add(&mut self, point: &Point) {
        if point.x < self.min.x {
            self.min.x = point.x;
        }
        if point.x > self.max.x {
            self.max.x = point.x;
        }
        if point.y < self.min.y {
            self.min.y = point.y;
        }
        if point.y > self.max.y {
            self.max.y = point.y;
        }
    }
}

#[cfg(test)]
fn debug_visited(list: &str, rope_length: usize, filename: &str) {
    use std::io::Write;

    let mut file = fs::File::create(filename).unwrap();

    let mut grid_for_boundary = Grid::new(rope_length);
    grid_for_boundary.move_list(list);

    let mut grid = Grid::new(rope_length);
    grid.print_boundary = grid_for_boundary.print_boundary;

    writeln!(file, "== {} ==", "Initial State").unwrap();
    writeln!(file, "{}", grid).unwrap();

    let movements = Grid::split_movements(list);

    for (direction, amount) in movements {
        writeln!(file, "== {} {} ==", direction, amount).unwrap();

        for _ in 0..amount {
            grid.move_single(direction);
            writeln!(file, "{}", grid).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_part1() {
        assert_eq!(count_visited("", 1), 1);
        assert_eq!(count_visited("R 1", 1), 1);
        assert_eq!(count_visited("R 2", 1), 2);
        assert_eq!(count_visited("R 2\nL 3", 1), 2);
        assert_eq!(count_visited("R 2\nL 4", 1), 3);

        assert_eq!(count_visited("R 2\nU 2\nL 2", 1), 4);

        assert_eq!(count_visited(TEST_INPUT_1, 1), 13);
    }

    #[test]
    fn test_part2() {
        const TEST_INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        assert_eq!(count_visited("", 9), 1);
        assert_eq!(count_visited("R 10", 9), 2);
        assert_eq!(count_visited("R 100", 9), 92);
        assert_eq!(count_visited("R 10\nU 10", 9), 7);

        assert_eq!(count_visited(TEST_INPUT_1, 9), 1);

        debug_visited(TEST_INPUT_1, 9, "debug_1-9.log");

        assert_eq!(count_visited(TEST_INPUT_2, 9), 36);
    }
}
