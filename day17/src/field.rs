use crate::actions::*;

use crate::rocks::*;
use crate::row_lookup::ROW_LOOKUP;
use circular_buffer::CircularBuffer;

// const MAX_SIZE: usize = 1_000_000;
const MAX_SIZE: usize = 4096;

pub struct Field {
    data: CircularBuffer<MAX_SIZE, u8>,
    move_cycle: MoveCycle,
    rock_cycle: RockCycle,
    rock: Rock,
    is_rock_fresh: bool,
    rock_position: (isize, usize),
    pub rocks_locked: usize,
    pruned_rows: usize,
}

impl Field {
    pub fn new(input: &str) -> Self {
        let mut rock_cycle = RockCycle::new();
        let rock = rock_cycle.next();
        Self {
            data: CircularBuffer::new(),
            move_cycle: MoveCycle::new(input),
            rock_cycle,

            rock,
            is_rock_fresh: true,

            // x,y offset:
            // x - distance to the left border
            // y - distance to the bottom
            rock_position: (0, 3),

            rocks_locked: 0,
            pruned_rows: 0,
        }
    }

    pub fn stack_height(&self) -> usize {
        self.data.len() + self.pruned_rows
    }

    fn new_rock(&mut self) {
        self.rock = self.rock_cycle.next();

        let x = 0; // offset of 2 is already included in ROCK
        let y = self.data.len() + 3;

        self.rock_position = (x, y);
        self.is_rock_fresh = true;
    }

    fn get_row(&self, row: usize) -> u8 {
        *self.data.get(row).unwrap_or(&0)
    }

    fn get_row_mut(&mut self, row: usize) -> &mut u8 {
        while row >= self.data.len() {
            self.data.push_back(0);
        }
        self.data.get_mut(row).unwrap()
    }

    fn lock_rock(&mut self) {
        let (x, y) = self.rock_position;
        for i in 0..number_of_rows_in_rock(&self.rock) {
            let row = y + i;
            *self.get_row_mut(row) |= shift(&self.rock[i], x);
        }
        self.rocks_locked += 1;
        self.new_rock();
    }

    fn highest_row_to_print(&self) -> usize {
        self.data
            .len()
            .max(self.rock_position.1 + number_of_rows_in_rock(&self.rock))
    }

    fn fall(&mut self) {
        self.is_rock_fresh = false;
        if self.rock_position.1 == 0 {
            // hit the ground
            self.lock_rock();
        } else {
            // rock falls
            self.rock_position.1 -= 1;

            if self.collides() {
                // hit something, back up and lock in place
                self.rock_position.1 += 1;
                self.lock_rock();
            } else {
                // free-falling
            }
        }
    }

    fn collides(&self) -> bool {
        let (x, y) = self.rock_position;

        (0..4).any(|i| {
            let row = y + i;
            self.get_row(row) & shift(&self.rock[i], x) != 0
        })
    }

    fn move_left(&mut self) {
        if self
            .rock
            .iter()
            .any(|row| shift(row, self.rock_position.0) & 0b1000000 != 0)
        {
            return;
        }

        self.rock_position.0 -= 1;
        if self.collides() {
            self.rock_position.0 += 1;
        }
    }

    fn move_right(&mut self) {
        if self
            .rock
            .iter()
            .any(|row| shift(row, self.rock_position.0) & 0b1 != 0)
        {
            return;
        }

        self.rock_position.0 += 1;
        if self.collides() {
            self.rock_position.0 -= 1;
        }
    }

    fn next_move(&mut self) {
        match self.move_cycle.next() {
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }

    pub fn one_step(&mut self) {
        self.next_move();
        self.fall();
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

        let num_rows = self.highest_row_to_print();

        for row_number in (0..num_rows).rev() {
            let row_state = self.get_row(row_number);
            let row_str = ROW_LOOKUP[row_state as usize];
            if row_number >= rock_lower_y && row_number <= rock_upper_y {
                let row_str = row_str.to_string();
                let rock_row = self.rock[row_number - rock_lower_y];
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

        if self.pruned_rows == 0 {
            write!(f, "+-------+")
        } else {
            write!(f, "| pruned|")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_input::INPUT;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_field() {
        let mut field = Field::new(INPUT);

        assert_eq!(field.rock, [0b00011110, 0b00000000, 0b00000000, 0b00000000]);

        assert!(field.is_rock_fresh);
        assert_eq!(
            field.to_string(),
            "\
|..@@@@.|
|.......|
|.......|
|.......|
+-------+"
        );

        field.one_step();
        assert_eq!(field.is_rock_fresh, false);
        assert_eq!(
            field.to_string(),
            "\
|...@@@@|
|.......|
|.......|
+-------+"
        );

        field.one_step();
        assert_eq!(
            field.to_string(),
            "\
|...@@@@|
|.......|
+-------+"
        );

        field.one_step();
        assert_eq!(
            field.to_string(),
            "\
|...@@@@|
+-------+"
        );

        field.one_step();
        assert!(field.is_rock_fresh);
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

        while field.rocks_locked < 2 {
            field.one_step();
        }
        assert_eq!(
            field.to_string(),
            "\
|....@..|
|....@..|
|..@@@..|
|.......|
|.......|
|.......|
|...#...|
|..###..|
|...#...|
|..####.|
+-------+"
        );

        while field.rocks_locked < 2022 {
            field.one_step();
        }
        assert_eq!(field.stack_height(), 3068)
    }

    #[test]
    fn test_ringbuf() {
        let mut buf = CircularBuffer::<4096, u8>::new();

        assert_eq!(buf.is_empty(), true);

        for i in 0..100 {
            buf.push_back(i);
        }

        assert_eq!(buf.len(), 100);
        assert_eq!(buf.is_full(), false);
        assert_eq!(buf.is_empty(), false);

        for i in 0..100 {
            assert_eq!(buf.get(i), Some(&(i as u8)));
        }

        assert_eq!(buf.pop_front(), Some(0));

        assert_eq!(buf.len(), 99);
        assert_eq!(buf.get(5), Some(&6));
    }
}
