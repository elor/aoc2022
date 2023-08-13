use std::iter::{Copied, Cycle};

pub type Rock = [u8; 4];

/// calculates the height of a rock
///
/// Well, actually it performs a lookup,
/// since the first row of a rock is unique
pub fn number_of_rows_in_rock(rock: &Rock) -> usize {
    match rock[0] {
        0b0011110 => 1,
        0b0001000 => 3,
        0b0011100 => 3,
        0b0010000 => 4,
        0b0011000 => 2,
        _ => 0,
    }
}

pub const ROCKS: [Rock; 5] = [
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

pub struct RockCycle {
    rocks: [Rock; 5],
    index: usize,
}

impl RockCycle {
    pub fn new() -> Self {
        Self {
            rocks: ROCKS,
            index: 0,
        }
    }

    pub fn next(&mut self) -> Rock {
        let rock = self.rocks[self.index];
        self.index = (self.index + 1) % self.rocks.len();
        rock
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_rock_height() {
        assert_eq!(number_of_rows_in_rock(&ROCKS[0]), 1);
        assert_eq!(number_of_rows_in_rock(&ROCKS[1]), 3);
        assert_eq!(number_of_rows_in_rock(&ROCKS[2]), 3);
        assert_eq!(number_of_rows_in_rock(&ROCKS[3]), 4);
        assert_eq!(number_of_rows_in_rock(&ROCKS[4]), 2);
    }
}
