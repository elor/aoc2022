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

    field.step_until(2022);

    field.stack_height()
}

fn part2(input: &str) -> usize {
    let mut field = Field::new(input);

    field.longstep_until(1_000_000_000_000);

    field.stack_height()
}
