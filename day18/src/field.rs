use std::collections::VecDeque;

use ndarray::{s, Array3};

#[derive(Debug)]
pub struct Field {
    data: Array3<i8>,
}

type Coordinate = [usize; 3];

fn parse_input(input: &str) -> Vec<Coordinate> {
    input.trim().lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Coordinate {
    let mut numbers = line.trim().split(',').map(|s| s.parse().unwrap());

    [
        numbers.next().unwrap(),
        numbers.next().unwrap(),
        numbers.next().unwrap(),
    ]
}

impl Field {
    pub fn from_str(input: &str) -> Self {
        let coords = parse_input(input);
        let x_max = coords.iter().map(|c| c[0]).max().unwrap();
        let y_max = coords.iter().map(|c| c[1]).max().unwrap();
        let z_max = coords.iter().map(|c| c[2]).max().unwrap();

        let x_min = coords.iter().map(|c| c[0]).min().unwrap();
        let y_min = coords.iter().map(|c| c[1]).min().unwrap();
        let z_min = coords.iter().map(|c| c[2]).min().unwrap();
        assert!(x_min > 0);
        assert!(y_min > 0);
        assert!(z_min > 0);

        // add 1 for max->size; and another 1 for zero-padding
        let mut data = Array3::zeros((x_max + 2, y_max + 2, z_max + 2));

        // set every coordinate to 1
        for coord in coords {
            data[coord] = 1;
        }

        Field { data }
    }

    fn x_surface_count(&self) -> usize {
        //idea: shift data array by 1 in x direction, then sum up all differences.
        let original = self.data.slice(s![..-1, .., ..]);
        let shifted = self.data.slice(s![1.., .., ..]);

        original
            .iter()
            .zip(shifted.iter())
            .map(|(a, b)| (a - b).unsigned_abs() as usize)
            .sum()
    }

    fn y_surface_count(&self) -> usize {
        //idea: shift data array by 1 in x direction, then sum up all differences.
        let original = self.data.slice(s![.., ..-1, ..]);
        let shifted = self.data.slice(s![.., 1.., ..]);

        original
            .iter()
            .zip(shifted.iter())
            .map(|(a, b)| (a - b).unsigned_abs() as usize)
            .sum()
    }

    fn z_surface_count(&self) -> usize {
        //idea: shift data array by 1 in x direction, then sum up all differences.
        let original = self.data.slice(s![.., .., ..-1]);
        let shifted = self.data.slice(s![.., .., 1..]);

        original
            .iter()
            .zip(shifted.iter())
            .map(|(a, b)| (a - b).unsigned_abs() as usize)
            .sum()
    }

    pub fn surface_count(&self) -> usize {
        self.x_surface_count() + self.y_surface_count() + self.z_surface_count()
    }

    pub fn fill_inner_voids(&mut self) {
        // floodfill the outside with 2s, and set all remaining 0s to 1s, before reverting all 2s
        // to 0s.

        // floodfill, starting at 0,0,0
        let mut stack: VecDeque<Coordinate> = VecDeque::new();
        stack.push_back([0, 0, 0]);
        while let Some(coord) = stack.pop_front() {
            let [x, y, z] = coord;

            if self.data[coord] == 0 {
                self.data[coord] = 2;
                // add all neighbours to stack
                if x > 0 {
                    stack.push_back([x - 1, y, z]);
                }
                if x < self.data.shape()[0] - 1 {
                    stack.push_back([x + 1, y, z]);
                }
                if y > 0 {
                    stack.push_back([x, y - 1, z]);
                }
                if y < self.data.shape()[1] - 1 {
                    stack.push_back([x, y + 1, z]);
                }
                if z > 0 {
                    stack.push_back([x, y, z - 1]);
                }
                if z < self.data.shape()[2] - 1 {
                    stack.push_back([x, y, z + 1]);
                }
            }
        }

        // set all remaining 0s to 1s
        for val in self.data.iter_mut() {
            if *val == 0 {
                *val = 1;
            }
        }

        // revert all 2s to 0s
        for val in self.data.iter_mut() {
            if *val == 2 {
                *val = 0;
            }
        }

        return;
    }
}

#[cfg(test)]
mod tests {
    use crate::field::Field;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_field_init() {
        let mut field = Field::from_str("1,1,1");
        assert_eq!(field.data.shape(), &[3, 3, 3]);
        assert_eq!(field.data[[1, 1, 1]], 1);
        assert_eq!(field.data.sum(), 1);

        field.data[[0, 0, 1]] = 1;
        assert_eq!(field.data.sum(), 2);
    }

    #[test]
    fn test_10_example() {
        let field = Field::from_str("2,2,2\n2,2,3");

        assert_eq!(field.data.sum(), 2);
        assert_eq!(field.x_surface_count(), 4);
        assert_eq!(field.y_surface_count(), 4);
        assert_eq!(field.z_surface_count(), 2);

        assert_eq!(field.surface_count(), 10);
    }

    #[test]
    fn test_test_input() {
        let field = Field::from_str(TEST_INPUT);
        assert_eq!(field.surface_count(), 64);
    }

    #[test]
    fn test_fill_inner_voids() {
        let mut field = Field::from_str(TEST_INPUT);
        field.fill_inner_voids();

        assert_eq!(field.surface_count(), 58);
    }
}
