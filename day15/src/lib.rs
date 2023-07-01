pub mod day15 {
    use std::cmp::Ordering;

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct Range {
        start: i32,
        end: i32,
    }

    impl Ord for Range {
        fn cmp(&self, other: &Self) -> Ordering {
            match self.start.cmp(&other.start) {
                Ordering::Equal => match self.end.cmp(&other.end) {
                    Ordering::Equal => Ordering::Equal,
                    Ordering::Less => Ordering::Greater,
                    Ordering::Greater => Ordering::Less,
                },
                other => other,
            }
        }
    }

    impl PartialOrd for Range {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Range {
        pub fn new(start: i32, end: i32) -> Self {
            Self { start, end }
        }

        pub fn len(&self) -> usize {
            (self.end - self.start + 1) as usize
        }

        pub fn is_empty(&self) -> bool {
            false
        }

        pub fn contains(&self, value: i32) -> bool {
            self.start <= value && value <= self.end
        }

        pub fn join(&self, other: &Self) -> Option<Range> {
            if self.start > other.end + 1 || other.start > self.end + 1 {
                return None;
            }

            let start = self.start.min(other.start);
            let end = self.end.max(other.end);

            Some(Range { start, end })
        }

        pub fn join_vec(ranges: Vec<Range>) -> Vec<Range> {
            let mut ranges = ranges;
            ranges.sort();

            ranges
                .iter()
                .fold(Vec::<Range>::new(), |mut acc, range| match acc.pop() {
                    Some(last) => match last.join(range) {
                        Some(joined_range) => {
                            acc.push(joined_range);
                            acc.to_vec()
                        }
                        None => {
                            acc.push(last);
                            acc.push(*range);
                            acc.to_vec()
                        }
                    },
                    None => {
                        acc.push(*range);
                        acc.to_vec()
                    }
                })
        }
    }
}
