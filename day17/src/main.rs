use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut field = Field::new(input);

    field.continue_until_rocks_locked(2022);

    field.number_of_filled_rows()
}

fn part2(_input: &str) -> usize {
    0
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Action {
    Creation,
    Rock,
    Drop,
    Move(Direction),
    Lock,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

type Rock = [u8; 4];

fn rock_height(rock: &Rock) -> usize {
    for (i, row) in rock.iter().enumerate().rev() {
        if *row != 0 {
            return i + 1;
        }
    }

    0
}

const ROCKS: [Rock; 5] = [
    //
    [
        0b0011110, // ####
        0b0000000, // ....
        0b0000000, // ....
        0b0000000, // ....
    ],
    [
        0b0001000, // .#..
        0b0011100, // ###.
        0b0001000, // .#..
        0b0000000, // ....
    ],
    [
        0b0011100, // ###.
        0b0000100, // ..#.
        0b0000100, // ..#.
        0b0000000, // ....
    ],
    [
        0b0010000, // #...
        0b0010000, // #...
        0b0010000, // #...
        0b0010000, // #...
    ],
    [
        0b0011000, // ##..
        0b0011000, // ##..
        0b0000000, // ....
        0b0000000, // ....
    ], //
];

use std::iter::{Copied, Cycle, Iterator};

type RockCycle = Cycle<Copied<std::slice::Iter<'static, Rock>>>;

fn rock_cycle() -> RockCycle {
    ROCKS.iter().copied().cycle()
}

struct MoveCycle {
    data: Vec<Direction>,
    current: usize,
}

impl MoveCycle {
    fn new(input: &str) -> Self {
        Self {
            data: input
                .trim()
                .chars()
                .map(|c| match c {
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => panic!("Invalid input character: {c}"),
                })
                .collect(),
            current: 0,
        }
    }

    fn next(&mut self) -> Direction {
        let result = self.data[self.current];
        self.current = (self.current + 1) % self.data.len();
        result
    }
}

const MAX_SIZE: usize = 4096;

enum DropResult {
    Ground,
    Rock,
    Dropped,
}

struct Field {
    data: [u8; MAX_SIZE],
    current_rock: Option<Rock>,
    rocks: RockCycle,
    moves: MoveCycle,
    rock_position: (isize, usize),
    last_action: Action,
    rocks_locked: usize,
}

impl Field {
    fn new(input: &str) -> Self {
        Self {
            data: [0; MAX_SIZE],
            current_rock: None,

            // x,y offset:
            // x - distance to the left border
            // y - distance to the bottom
            rock_position: (0, 0),
            last_action: Action::Creation,
            rocks: rock_cycle(),
            moves: MoveCycle::new(input),
            rocks_locked: 0,
        }
    }

    fn next_rock(&mut self) {
        let rock = self.rocks.next().unwrap();

        let x = 0; // offset of 2 is already included in ROCK
        let y = self.number_of_filled_rows() + 3;

        self.current_rock = Some(rock);
        self.rock_position = (x, y);

        self.last_action = Action::Rock;
    }

    fn lock_rock(&mut self) {
        if let Some(rock) = self.current_rock {
            let (x, y) = self.rock_position;
            for (i, rock_row) in rock.iter().enumerate() {
                let row = y + i;
                self.data[row] |= shift(rock_row, x);
            }
            self.current_rock = None;
            self.rocks_locked += 1;
            self.last_action = Action::Lock;
        }
    }

    fn number_of_filled_rows(&self) -> usize {
        let rock_row = if let Some(rock) = self.current_rock {
            self.rock_position.1 + rock_height(&rock)
        } else {
            0
        };
        for row in (rock_row..MAX_SIZE).rev() {
            if self.data[row] != 0 {
                return row + 1;
            }
        }
        rock_row
    }

    fn fall_once(&mut self) -> DropResult {
        if self.rock_position.1 == 0 {
            self.lock_rock();
            return DropResult::Ground;
        }

        self.rock_position.1 -= 1;

        if self.collides() {
            self.rock_position.1 += 1;
            self.lock_rock();
            return DropResult::Rock;
        }

        self.last_action = Action::Drop;
        DropResult::Dropped
    }

    fn collides(&self) -> bool {
        if let Some(rock) = self.current_rock {
            let (x, y) = self.rock_position;

            (0..4).any(|i| {
                let row = y + i;
                self.data[row] & shift(&rock[i], x) != 0
            })
        } else {
            false
        }
    }

    fn one_action(&mut self) -> Action {
        // order:
        // 0. Creation - None --> New
        // 1. insert rock - New, Lock --> Rock
        // 2. Move rock - Rock, Drop --> Move
        // 3. Drop rock - Move --> Drop, Lock
        match self.last_action {
            Action::Creation => {
                self.next_rock();
            }
            Action::Rock => {
                self.next_move();
            }
            Action::Move(_) => {
                self.fall_once();
            }
            Action::Drop => {
                self.next_move();
            }
            Action::Lock => {
                self.next_rock();
            }
        }

        self.last_action
    }

    fn move_left(&mut self) {
        self.last_action = Action::Move(Direction::Left);

        if let Some(rock) = self.current_rock {
            if rock
                .iter()
                .any(|row| shift(row, self.rock_position.0) & 0b1000000 != 0)
            {
                return;
            }
        }

        self.rock_position.0 -= 1;
        if self.collides() {
            self.rock_position.0 += 1;
        }
    }

    fn move_right(&mut self) {
        self.last_action = Action::Move(Direction::Right);

        if let Some(rock) = self.current_rock {
            if rock
                .iter()
                .any(|row| shift(row, self.rock_position.0) & 0b1 != 0)
            {
                return;
            }
        }

        self.rock_position.0 += 1;
        if self.collides() {
            self.rock_position.0 -= 1;
        }
    }

    fn next_move(&mut self) {
        match self.moves.next() {
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }

    fn continue_until_rocks_locked(&mut self, rock_limit: usize) {
        while self.rocks_locked < rock_limit {
            self.one_action();
        }
    }
}

fn shift(rock_row: &u8, x: isize) -> u8 {
    if x < 0 {
        rock_row << -x
    } else {
        rock_row >> x
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rock_upper_y = self.rock_position.1 + 3;
        let rock_lower_y = self.rock_position.1;

        for (row_number, row_state) in self
            .data
            .iter()
            .enumerate()
            .take(self.number_of_filled_rows())
            .rev()
        {
            let row_str = ROW_LOOKUP[*row_state as usize];
            if self.current_rock.is_some()
                && row_number >= rock_lower_y
                && row_number <= rock_upper_y
            {
                let row_str = row_str.to_string();
                let rock_row = self.current_rock.unwrap()[row_number - rock_lower_y];
                let rock_row = shift(&rock_row, self.rock_position.0);
                let rock_str = ROW_LOOKUP[rock_row as usize];

                // zip row and rock, and display '@' when rock_str is a '#'
                for (row_c, rock_c) in row_str.chars().zip(rock_str.chars()) {
                    write!(f, "{}", if rock_c == '#' { '@' } else { row_c })?;
                }

                writeln!(f)?;
            } else {
                writeln!(f, "{}", row_str)?;
            }
        }
        write!(f, "+-------+")
    }
}

const ROW_LOOKUP: [&str; 1 << 7] = [
    "|.......|", //
    "|......#|", //
    "|.....#.|", //
    "|.....##|", //
    "|....#..|", //
    "|....#.#|", //
    "|....##.|", //
    "|....###|", //
    "|...#...|", //
    "|...#..#|", //
    "|...#.#.|", //
    "|...#.##|", //
    "|...##..|", //
    "|...##.#|", //
    "|...###.|", //
    "|...####|", //
    "|..#....|", //
    "|..#...#|", //
    "|..#..#.|", //
    "|..#..##|", //
    "|..#.#..|", //
    "|..#.#.#|", //
    "|..#.##.|", //
    "|..#.###|", //
    "|..##...|", //
    "|..##..#|", //
    "|..##.#.|", //
    "|..##.##|", //
    "|..###..|", //
    "|..###.#|", //
    "|..####.|", //
    "|..#####|", //
    "|.#.....|", //
    "|.#....#|", //
    "|.#...#.|", //
    "|.#...##|", //
    "|.#..#..|", //
    "|.#..#.#|", //
    "|.#..##.|", //
    "|.#..###|", //
    "|.#.#...|", //
    "|.#.#..#|", //
    "|.#.#.#.|", //
    "|.#.#.##|", //
    "|.#.##..|", //
    "|.#.##.#|", //
    "|.#.###.|", //
    "|.#.####|", //
    "|.##....|", //
    "|.##...#|", //
    "|.##..#.|", //
    "|.##..##|", //
    "|.##.#..|", //
    "|.##.#.#|", //
    "|.##.##.|", //
    "|.##.###|", //
    "|.###...|", //
    "|.###..#|", //
    "|.###.#.|", //
    "|.###.##|", //
    "|.####..|", //
    "|.####.#|", //
    "|.#####.|", //
    "|.######|", //
    "|#......|", //
    "|#.....#|", //
    "|#....#.|", //
    "|#....##|", //
    "|#...#..|", //
    "|#...#.#|", //
    "|#...##.|", //
    "|#...###|", //
    "|#..#...|", //
    "|#..#..#|", //
    "|#..#.#.|", //
    "|#..#.##|", //
    "|#..##..|", //
    "|#..##.#|", //
    "|#..###.|", //
    "|#..####|", //
    "|#.#....|", //
    "|#.#...#|", //
    "|#.#..#.|", //
    "|#.#..##|", //
    "|#.#.#..|", //
    "|#.#.#.#|", //
    "|#.#.##.|", //
    "|#.#.###|", //
    "|#.##...|", //
    "|#.##..#|", //
    "|#.##.#.|", //
    "|#.##.##|", //
    "|#.###..|", //
    "|#.###.#|", //
    "|#.####.|", //
    "|#.#####|", //
    "|##.....|", //
    "|##....#|", //
    "|##...#.|", //
    "|##...##|", //
    "|##..#..|", //
    "|##..#.#|", //
    "|##..##.|", //
    "|##..###|", //
    "|##.#...|", //
    "|##.#..#|", //
    "|##.#.#.|", //
    "|##.#.##|", //
    "|##.##..|", //
    "|##.##.#|", //
    "|##.###.|", //
    "|##.####|", //
    "|###....|", //
    "|###...#|", //
    "|###..#.|", //
    "|###..##|", //
    "|###.#..|", //
    "|###.#.#|", //
    "|###.##.|", //
    "|###.###|", //
    "|####...|", //
    "|####..#|", //
    "|####.#.|", //
    "|####.##|", //
    "|#####..|", //
    "|#####.#|", //
    "|######.|", //
    "|#######|", //
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let mut cycle = MoveCycle::new(input);

        assert_eq!(cycle.next(), Direction::Right);
        assert_eq!(cycle.next(), Direction::Right);
        assert_eq!(cycle.next(), Direction::Right);
        assert_eq!(cycle.next(), Direction::Left);
        assert_eq!(cycle.next(), Direction::Left);

        for _ in 5..input.len() {
            cycle.next();
        }

        assert_eq!(cycle.next(), Direction::Right);
        assert_eq!(cycle.next(), Direction::Right);
        assert_eq!(cycle.next(), Direction::Right);
        assert_eq!(cycle.next(), Direction::Left);
        assert_eq!(cycle.next(), Direction::Left);

        let mut field = Field::new(input);

        assert_eq!(
            field.to_string(),
            "\
+-------+"
        );

        assert_eq!(field.one_action(), Action::Rock);

        assert_eq!(
            field.to_string(),
            "\
|..@@@@.|
|.......|
|.......|
|.......|
+-------+"
        );

        assert_eq!(field.one_action(), Action::Move(Direction::Right));
        assert_eq!(
            field.to_string(),
            "\
|...@@@@|
|.......|
|.......|
|.......|
+-------+"
        );

        assert_eq!(field.one_action(), Action::Drop);
        assert_eq!(
            field.to_string(),
            "\
|...@@@@|
|.......|
|.......|
+-------+"
        );

        assert_eq!(field.one_action(), Action::Move(Direction::Right));
        assert_eq!(
            field.to_string(),
            "\
|...@@@@|
|.......|
|.......|
+-------+"
        );

        assert_eq!(field.one_action(), Action::Drop);
        assert_eq!(
            field.to_string(),
            "\
|...@@@@|
|.......|
+-------+"
        );

        assert_eq!(field.one_action(), Action::Move(Direction::Right));
        assert_eq!(
            field.to_string(),
            "\
|...@@@@|
|.......|
+-------+"
        );

        assert_eq!(field.one_action(), Action::Drop);
        assert_eq!(
            field.to_string(),
            "\
|...@@@@|
+-------+"
        );

        assert_eq!(field.one_action(), Action::Move(Direction::Left));
        assert_eq!(
            field.to_string(),
            "\
|..@@@@.|
+-------+"
        );

        assert_eq!(field.one_action(), Action::Lock);
        assert_ne!(field.data[0], 0);
        assert_eq!(field.number_of_filled_rows(), 1);
        assert_eq!(
            field.to_string(),
            "\
|..####.|
+-------+"
        );

        assert_eq!(field.one_action(), Action::Rock);
        assert_eq!(
            field.to_string(),
            "\
|...@...|
|..@@@..|
|...@...|
|.......|
|.......|
|.......|
|..####.|
+-------+"
        );

        field.continue_until_rocks_locked(2022);
        assert_eq!(field.number_of_filled_rows(), 3068)
    }

    #[test]
    fn test_part2() {}
}
