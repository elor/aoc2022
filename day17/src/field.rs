use crate::actions::*;

use crate::column_counter::{ColumnCounter, ColumnCounterFn};
use crate::rocks::*;
use crate::row_lookup::ROW_LOOKUP;
use circular_buffer::CircularBuffer;

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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
    pub moves_performed: usize,
    pruned_rows: usize,
    column_counter: ColumnCounter,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Cycle {
    start: usize,
    period: usize,
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
            column_counter: ColumnCounter::new(),
            moves_performed: 0,
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
            assert!(!self.data.is_full());
            self.data.push_back(0);
        }
        self.data.get_mut(row).unwrap()
    }

    fn lock_rock(&mut self) {
        let (x, y) = self.rock_position;
        for i in 0..number_of_rows_in_rock(&self.rock) {
            let row = y + i;
            let shifted_rock_row = shift(self.rock[i], x);
            self.column_counter.add(shifted_rock_row);
            *self.get_row_mut(row) |= shifted_rock_row;
        }
        self.rocks_locked += 1;
        self.new_rock();
        self.prune();
    }

    fn prune(&mut self) {
        while self.column_counter.is_full() {
            let popped_row = self.data.pop_front().unwrap();
            self.column_counter.remove(popped_row);
            self.pruned_rows += 1;
            self.rock_position.1 -= 1;
        }
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

        (0..4).any(|i| -> bool {
            let row = y + i;
            self.get_row(row) & shift(self.rock[i], x) != 0
        })
    }

    fn move_left(&mut self) {
        if self
            .rock
            .iter()
            .any(|row| shift(*row, self.rock_position.0) & 0b1000000 != 0)
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
            .any(|row| shift(*row, self.rock_position.0) & 0b1 != 0)
        {
            return;
        }

        self.rock_position.0 += 1;
        if self.collides() {
            self.rock_position.0 -= 1;
        }
    }

    fn next_move(&mut self) {
        self.moves_performed += 1;
        match self.move_cycle.next() {
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }

    pub fn one_step(&mut self) {
        self.next_move();
        self.fall();
    }

    pub fn step_until(&mut self, rocks_locked: usize) {
        while self.rocks_locked < rocks_locked {
            self.one_step();
        }
    }

    pub fn longstep_until(&mut self, rocks_locked_target: usize) {
        let timeout_steps = self.move_cycle.len() * self.rock_cycle.len() * 5;
        if let Some(cycle) = self.find_cycle(timeout_steps) {
            let old_rocks_locked = self.rocks_locked;
            let old_pruned_rows = self.pruned_rows;
            let supercycle_length = cycle.period;

            for _ in 0..supercycle_length {
                self.one_step();
            }
            let rocks_locked = self.rocks_locked;
            let pruned_rows = self.pruned_rows;

            let rocks_locked_diff = rocks_locked - old_rocks_locked;
            let pruned_rows_diff = pruned_rows - old_pruned_rows;
            let moves_diff = supercycle_length;

            // accelerate

            let rocks_left = rocks_locked_target - self.rocks_locked;
            let supercycles_left = rocks_left / rocks_locked_diff;

            self.rocks_locked += rocks_locked_diff * supercycles_left;
            self.pruned_rows += pruned_rows_diff * supercycles_left;
            self.moves_performed += moves_diff * supercycles_left;

            assert!(self.rocks_locked <= rocks_locked_target);
            assert!(self.rocks_locked + rocks_locked_diff >= rocks_locked_target);
        }

        self.step_until(rocks_locked_target);
    }

    pub fn find_cycle(&mut self, timeout_in_moves: usize) -> Option<Cycle> {
        let mut seen = HashMap::new();

        // let test_cycle_length = self.move_cycle.len();

        // 1. run a full move cycle
        // 2. if hash is in map -> cycle found, return start and difference
        // 3. if not -> store in map and continue with 1.

        let start_moves = self.moves_performed;

        let supercycle_length = self.move_cycle.len() * self.rock_cycle.len();

        while self.moves_performed < start_moves + timeout_in_moves {
            if self.moves_performed + supercycle_length > start_moves + timeout_in_moves {
                // early abort
                return None;
            }

            for _ in 0..supercycle_length {
                self.one_step();
            }

            let hash = self.get_hash();
            if let Some(start) = seen.get(&hash) {
                return Some(Cycle {
                    start: *start,
                    period: self.moves_performed - start,
                });
            }
            seen.insert(hash, self.moves_performed);
            // println!("hash: {hash:016x} at move {}\r", self.moves_performed);
            // println!("   counters: {:?}", self.column_counter);
        }

        return None;
    }

    fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

fn shift(rock_row: u8, x: isize) -> u8 {
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
                let rock_row = shift(rock_row, self.rock_position.0);
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

impl Hash for Field {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // self.data.hash(state);
        self.column_counter.hash(state);
        self.move_cycle.hash(state);
        self.rock.hash(state);
        self.rock_position.hash(state);
        self.is_rock_fresh.hash(state);
        self.move_cycle.hash(state);
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

    #[test]
    fn test_hash() {
        let mut field = Field::new(INPUT);
        let mut field2 = Field::new(INPUT);

        field.step_until(100);
        field2.step_until(100);

        assert_eq!(field.get_hash(), field2.get_hash());

        field.one_step();
        assert_ne!(field.get_hash(), field2.get_hash());

        field2.one_step();
        assert_eq!(field.get_hash(), field2.get_hash());

        field2.one_step();
        assert_ne!(field.get_hash(), field2.get_hash());
    }

    #[test]
    fn test_find_cycle() {
        let mut field = Field::new(INPUT);

        let cycle = field.find_cycle(1_000);

        assert!(cycle.is_some());
        assert_eq!(
            cycle.unwrap(),
            Cycle {
                start: 200,
                period: 200,
            }
        );
    }

    #[test]
    fn test_one_million_rocks() {
        let mut field = Field::new(INPUT);

        field.longstep_until(1_000_000);

        assert_eq!(field.stack_height(), 1_514_288);
    }

    #[test]
    fn test_one_trillion_rocks() {
        let mut field = Field::new(INPUT);

        field.longstep_until(1_000_000_000_000);

        assert_eq!(field.stack_height(), 1_514_285_714_288);
    }
}
