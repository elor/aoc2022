use std::fs;

mod actions;
mod column_counter;
mod field;
mod rocks;
mod row_lookup;
mod test_input;

use crate::field::Field;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut field = Field::new(input);

    while field.rocks_locked < 2022 {
        field.one_step();
    }

    field.stack_height()
}

fn part2(_input: &str) -> usize {
    0
}
