use std::{
    fmt::{Display, Formatter},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut field = Field::new();
    let binding = parse_sequence(input);
    let mut sequence = binding.iter().cycle();

    SHAPES.iter().cycle().take(2022).for_each(|shape| {
        field.drop_one_piece(shape, &mut sequence);
    });

    field.highest_row - 1
}

fn part2(_input: &str) -> usize {
    0
}

const WIDTH: usize = 7;
const MAX_HEIGHT: usize = 2022 * 4;

struct Field {
    field: [[bool; WIDTH]; MAX_HEIGHT],
    highest_row: usize,
    current_piece: Option<Piece>,
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_height = if let Some(piece) = &self.current_piece {
            piece.position.1 + piece.shape.len()
        } else {
            self.highest_row
        };

        if max_height > 0 {
            for row in (0..max_height).rev() {
                write!(f, "|")?;
                for cell in self.field[row].iter() {
                    write!(f, "{}", if *cell { "#" } else { "." })?;
                }
                writeln!(f, "|")?;
            }
        }

        write!(f, "+-------+")?;

        Ok(())
    }
}

enum DropResult {
    Landed,
    Falling,
}

impl Field {
    fn new() -> Field {
        Field {
            field: [[false; 7]; MAX_HEIGHT],
            highest_row: 0,
            current_piece: None,
        }
    }

    fn add_piece(&mut self, shape: &'static Shape) {
        let width = shape
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter_map(|(i, cell)| if *cell { Some(i) } else { None })
                    .max()
                    .unwrap_or(0)
                    + 1
            })
            .max()
            .unwrap();

        let piece = Piece {
            shape,
            position: (2, self.highest_row + 3),
            width,
        };
        self.current_piece = Some(piece);
    }

    fn anchor_piece(&mut self) {
        if let Some(piece) = &self.current_piece {
            for (i, row) in piece.shape.iter().enumerate() {
                let i = piece.shape.len() - i - 1;
                for (j, cell) in row.iter().enumerate() {
                    if *cell {
                        let x = piece.position.0 + j;
                        let y = piece.position.1 + i;
                        self.highest_row = self.highest_row.max(y + 1);

                        self.field[y][x] = true;
                    }
                }
            }

            self.current_piece = None;
        }
    }

    fn move_piece(&mut self, direction: &Direction) {
        match &mut self.current_piece {
            Some(piece) => match direction {
                Direction::Left => {
                    if piece.position.0 > 0 {
                        piece.position.0 -= 1;
                    }
                }
                Direction::Right => {
                    if piece.position.0 + piece.width < WIDTH {
                        piece.position.0 += 1;
                    }
                }
            },
            None => {}
        }

        if self.collides() {
            if let Some(piece) = &mut self.current_piece {
                match direction {
                    Direction::Left => {
                        if piece.position.0 > 0 {
                            piece.position.0 += 1;
                        }
                    }
                    Direction::Right => {
                        if piece.position.0 + piece.width < WIDTH {
                            piece.position.0 -= 1;
                        }
                    }
                }
            }
        }
    }

    fn drop(&mut self) -> DropResult {
        if let Some(piece) = &mut self.current_piece {
            if piece.position.1 == 0 {
                self.anchor_piece();
                return DropResult::Landed;
            }
            piece.position.1 -= 1;
        }

        if self.collides() {
            if let Some(piece) = &mut self.current_piece {
                piece.position.1 += 1;
                self.anchor_piece();
                return DropResult::Landed;
            }
        }

        DropResult::Falling
    }

    fn collides(&self) -> bool {
        let piece = self.current_piece.as_ref().unwrap();
        for (i, row) in piece.shape.iter().enumerate() {
            let i = piece.shape.len() - i - 1;
            for (j, cell) in row.iter().enumerate() {
                if *cell && self.field[piece.position.1 + i][piece.position.0 + j] {
                    return true;
                }
            }
        }

        false
    }

    fn has_piece(&self) -> bool {
        self.current_piece.is_some()
    }

    fn drop_one_piece(
        &mut self,
        shape: &'static Shape,
        sequence: &mut dyn Iterator<Item = &Direction>,
    ) {
        self.add_piece(shape);

        while self.has_piece() {
            self.move_piece(sequence.next().unwrap());
            self.drop();
        }
    }
}

type Shape = [[bool; 4]; 4];

struct Piece {
    shape: &'static Shape,
    width: usize,
    position: (usize, usize), // offset relative to bottom left corner
}

enum Direction {
    Left,
    Right,
}

fn parse_sequence(input: &str) -> Vec<Direction> {
    input
        .trim()
        .bytes()
        .map(|b| match b {
            b'<' => Direction::Left,
            b'>' => Direction::Right,
            _ => panic!("Invalid direction"),
        })
        .collect()
}

const SHAPES: [Shape; 5] = [
    [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
        [true, true, true, true],
    ],
    [
        [false, false, false, false],
        [false, true, false, false],
        [true, true, true, false],
        [false, true, false, false],
    ],
    [
        [false, false, false, false],
        [false, false, true, false],
        [false, false, true, false],
        [true, true, true, false],
    ],
    [
        [true, false, false, false],
        [true, false, false, false],
        [true, false, false, false],
        [true, false, false, false],
    ],
    [
        [false, false, false, false],
        [false, false, false, false],
        [true, true, false, false],
        [true, true, false, false],
    ],
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test.txt").unwrap();
        let binding = parse_sequence(&input);
        let mut sequence = binding.iter().cycle();

        let mut field = Field::new();

        assert_eq!(field.to_string(), "+-------+");

        let mut shapecycle = SHAPES.iter().cycle();

        field.drop_one_piece(shapecycle.next().unwrap(), &mut sequence);
        assert_eq!(field.to_string(), "|..####.|\n+-------+");
        assert_eq!(field.highest_row, 1);

        field.drop_one_piece(shapecycle.next().unwrap(), &mut sequence);
        assert_eq!(
            field.to_string(),
            "\
|...#...|
|..###..|
|...#...|
|..####.|
+-------+"
        );

        field.drop_one_piece(shapecycle.next().unwrap(), &mut sequence);
        assert_eq!(
            field.to_string(),
            "\
|..#....|
|..#....|
|####...|
|..###..|
|...#...|
|..####.|
+-------+"
        );

        field.drop_one_piece(shapecycle.next().unwrap(), &mut sequence);
        assert_eq!(
            field.to_string(),
            "\
|....#..|
|..#.#..|
|..#.#..|
|#####..|
|..###..|
|...#...|
|..####.|
+-------+"
        );

        field.drop_one_piece(shapecycle.next().unwrap(), &mut sequence);
        assert_eq!(
            field.to_string(),
            "\
|....##.|
|....##.|
|....#..|
|..#.#..|
|..#.#..|
|#####..|
|..###..|
|...#...|
|..####.|
+-------+"
        );

        field.drop_one_piece(shapecycle.next().unwrap(), &mut sequence);
        assert_eq!(
            field.to_string(),
            "\
|.####..|
|....##.|
|....##.|
|....#..|
|..#.#..|
|..#.#..|
|#####..|
|..###..|
|...#...|
|..####.|
+-------+"
        );

        field.drop_one_piece(shapecycle.next().unwrap(), &mut sequence);
        assert_eq!(
            field.to_string(),
            "\
|..#....|
|.###...|
|..#....|
|.####..|
|....##.|
|....##.|
|....#..|
|..#.#..|
|..#.#..|
|#####..|
|..###..|
|...#...|
|..####.|
+-------+"
        );

        field.drop_one_piece(shapecycle.next().unwrap(), &mut sequence);
        assert_eq!(
            field.to_string(),
            "\
|.....#.|
|.....#.|
|..####.|
|.###...|
|..#....|
|.####..|
|....##.|
|....##.|
|....#..|
|..#.#..|
|..#.#..|
|#####..|
|..###..|
|...#...|
|..####.|
+-------+"
        );

        let mut field = Field::new();
        let mut sequence = binding.iter().cycle();

        SHAPES.iter().cycle().take(2022).for_each(|shape| {
            field.drop_one_piece(shape, &mut sequence);
        });

        assert_eq!(field.highest_row, 3068);
    }

    #[test]
    fn test_part2() {}
}
