pub type ColumnCounter = [usize; 7];

pub trait ColumnCounterFn {
    fn new() -> Self;
    fn add(&mut self, row: u8);
    fn remove(&mut self, row: u8);
    fn is_full(&self) -> bool;
}

impl ColumnCounterFn for ColumnCounter {
    fn new() -> Self {
        [0; 7]
    }

    fn add(&mut self, row: u8) {
        for (i, item) in self.iter_mut().enumerate() {
            if row & (1 << i) != 0 {
                *item += 1;
            }
        }
    }

    fn remove(&mut self, row: u8) {
        for (i, item) in self.iter_mut().enumerate() {
            if row & (1 << i) != 0 {
                *item -= 1;
            }
        }
    }

    fn is_full(&self) -> bool {
        self.iter().all(|&c| c != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_counter() {
        let mut counter = ColumnCounter::new();

        assert!(!counter.is_full());

        counter.add(0b111_1111);

        assert!(counter.is_full());

        counter.remove(0b001_0000);
        assert!(!counter.is_full());
    }
}
