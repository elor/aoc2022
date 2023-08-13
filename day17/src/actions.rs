#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

pub struct MoveCycle {
    data: Vec<Direction>,
    current: usize,
    count: usize,
}

impl MoveCycle {
    pub fn new(input: &str) -> Self {
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
            count: 0,
        }
    }

    pub fn next(&mut self) -> Direction {
        let result = self.data[self.current];
        self.current = (self.current + 1) % self.data.len();
        self.count += 1;

        result
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_input::INPUT;

    use super::*;

    #[test]
    fn test_move_cycle() {
        let mut cycle = MoveCycle::new(INPUT);

        assert_eq!(cycle.next(), Direction::Right);
        assert_eq!(cycle.next(), Direction::Right);
        assert_eq!(cycle.next(), Direction::Right);
        assert_eq!(cycle.next(), Direction::Left);
        assert_eq!(cycle.next(), Direction::Left);

        for _ in 5..INPUT.len() {
            cycle.next();
        }

        assert_eq!(cycle.next(), Direction::Right);
        assert_eq!(cycle.next(), Direction::Right);
        assert_eq!(cycle.next(), Direction::Right);
        assert_eq!(cycle.next(), Direction::Left);
        assert_eq!(cycle.next(), Direction::Left);
    }
}
