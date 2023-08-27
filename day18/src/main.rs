use std::fs;

mod field;

use crate::field::Field;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let field = Field::from_str(input);

    field.surface_count()
}

fn part2(_input: &str) -> usize {
    let mut field = Field::from_str(_input);

    field.fill_inner_voids();
    field.surface_count()
}
