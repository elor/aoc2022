use std::{
    collections::{HashMap, HashSet},
    fs,
};

use day15::day15::Range;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

/*
* Setup:
* Sensors and beacons, at integer x,y coords
* sensors know their positions
* sensors detect position of closest beacon precisely (manhattan distance)
* Never a tie between two distances!
* undetected beacons can exist
*
* Part 1:
* Count the number of positions that cannot contain a beacon in a given line
*
* */

fn part1(input: &str) -> usize {
    let field = Field::from(input);

    field.count_empty_positions_in_line(2_000_000)
}

fn part2(input: &str) -> usize {
    let field = Field::from(input);

    let lost_beacon = field.find_lost_beacon(4_000_000);

    lost_beacon.tuning_frequency()
}

type Position = (i32, i32);
type Beacon = Position;
type Sensor = Position;

trait TuningFrequency {
    fn tuning_frequency(&self) -> usize;
}

impl TuningFrequency for Beacon {
    fn tuning_frequency(&self) -> usize {
        let (x, y) = *self;

        assert!(x >= 0 && y >= 0);

        x as usize * 4000000 + y as usize
    }
}

fn beaconless_positions(sensor: &Sensor, range: i32, line: i32) -> Option<Range> {
    let (x, y) = *sensor;
    let span = range - (line - y).abs();

    if span <= 0 {
        return None;
    }

    let span = range - (line - y).abs();

    Some(Range::new(x - span, x + span))
}

struct Field {
    sensors: HashMap<Sensor, i32>,
    beacons: HashSet<Beacon>,
}

impl Field {
    fn from(input: &str) -> Self {
        let mut sensors = HashMap::new();
        let mut beacons = HashSet::new();

        input.trim().lines().for_each(|line| {
            let (sensor, beacon) = parse_line(line);
            let range = manhattan_distance(&sensor, &beacon);
            sensors.insert(sensor, range);
            beacons.insert(beacon);
        });

        Self { sensors, beacons }
    }

    fn count_empty_positions_in_line(&self, line: i32) -> usize {
        let mut blocked_positions = Vec::new();

        self.sensors.iter().for_each(|(sensor, &range)| {
            if let Some(range) = beaconless_positions(sensor, range, line) {
                blocked_positions.push(range);
            }
        });

        let blocked_positions = Range::join_vec(blocked_positions);

        // no need to add sensors; they're inside their own range

        // count the blocked positions
        let count = blocked_positions
            .iter()
            .fold(0, |acc, range| acc + range.len());

        let beacon_count_on_line = self
            .beacons
            .iter()
            .filter(|beacon| beacon.1 == line)
            .filter(|beacon| {
                blocked_positions
                    .iter()
                    .any(|range| range.contains(beacon.0))
            })
            .count();

        count - beacon_count_on_line
    }

    fn find_unknown_beacon_in_line(&self, line: i32, limit: i32) -> Option<Beacon> {
        let limit = Range::new(0, limit);
        let mut blocked_positions = Vec::new();

        self.sensors.iter().for_each(|(sensor, &range)| {
            if let Some(range) = beaconless_positions(sensor, range, line) {
                blocked_positions.push(range);
            }
        });

        // keep sensors, keep beacons

        // join blocked positions
        let blocked_positions = Range::join_vec(blocked_positions);

        match blocked_positions.len() {
            0 => None,
            1 => {
                let range = blocked_positions[0];

                if range.start == 1 {
                    Some((range.end, line))
                } else if range.end == limit.end - 1 {
                    Some((range.start, line))
                } else {
                    None
                }
            }
            2 => {
                let range1 = blocked_positions[0];
                let range2 = blocked_positions[1];

                assert!(range1.start <= 0);
                assert!(range2.end >= limit.end);

                assert_eq!(range1.end + 2, range2.start);

                Some((range1.end + 1, line))
            }
            _ => panic!("Too many potential unknown beacons, this must be wrong"),
        }
    }

    fn find_lost_beacon(&self, limit: i32) -> Beacon {
        let beacons: Vec<Beacon> = (0..=limit)
            .filter_map(|line| self.find_unknown_beacon_in_line(line, limit))
            .collect();

        assert_eq!(beacons.len(), 1);

        beacons[0]
    }
}

fn parse_line(line: &str) -> (Sensor, Beacon) {
    use sscanf::sscanf;

    match sscanf!(
        line.trim(),
        "Sensor at x={i32}, y={i32}: closest beacon is at x={i32}, y={i32}",
    ) {
        Ok((sx, sy, bx, by)) => ((sx, sy), (bx, by)),
        _ => {
            panic!("Invalid input line")
        }
    }
}

fn manhattan_distance(a: &Position, b: &Position) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = INPUT;
        let field = Field::from(input);

        let first_line = INPUT.trim().lines().next().unwrap();
        assert_eq!(parse_line(first_line), ((2, 18), (-2, 15)));

        assert_eq!(field.sensors.len(), 14);
        assert_eq!(field.beacons.len(), 6);

        assert_eq!(field.count_empty_positions_in_line(10), 26);
    }

    #[test]
    fn test_part2() {
        let input = INPUT;
        let field = Field::from(input);

        assert_eq!(field.find_unknown_beacon_in_line(10, 20), None);
        assert_eq!(field.find_unknown_beacon_in_line(11, 20), Some((14, 11)));

        let lost_beacon = field.find_lost_beacon(20);
        assert_eq!(lost_beacon, (14, 11));

        assert_eq!(lost_beacon.tuning_frequency(), 56000011);
    }

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_range() {
        use day15::day15::Range;

        assert_eq!(Range::new(1, 1).len(), 1);
        assert_eq!(Range::new(1, 2).len(), 2);
        assert_eq!(Range::new(0, 10).len(), 11);

        let range = Range::new(1, 10);

        assert_eq!(range.join(&Range::new(11, 20)), Some(Range::new(1, 20)));

        assert_eq!(range.join(&Range::new(5, 15)), Some(Range::new(1, 15)));

        assert_eq!(range.join(&Range::new(5, 9)), Some(Range::new(1, 10)));

        assert_eq!(range.join(&Range::new(100, 105)), None);

        let mut ranges = vec![
            Range::new(1, 1),
            Range::new(20, 30),
            Range::new(2, 10),
            Range::new(28, 31),
            Range::new(1, 5),
        ];
        ranges.sort();

        assert_eq!(
            ranges,
            vec![
                Range::new(1, 5),
                Range::new(1, 1),
                Range::new(2, 10),
                Range::new(20, 30),
                Range::new(28, 31),
            ]
        );

        let big_ranges =
            ranges
                .iter()
                .fold(Vec::<Range>::new(), |mut acc, range| match acc.pop() {
                    Some(last) => {
                        println!("last: {:?}, range: {:?}", last, range);
                        match last.join(range) {
                            Some(joined_range) => {
                                println!("joined: {:?}", joined_range);
                                acc.push(joined_range);
                            }
                            None => {
                                println!("not joined");
                                acc.push(last);
                                acc.push(*range);
                            }
                        }

                        acc.to_vec()
                    }
                    None => {
                        acc.push(*range);
                        acc
                    }
                });

        assert_eq!(big_ranges, vec![Range::new(1, 10), Range::new(20, 31)]);

        assert_eq!(
            Range::join_vec(ranges),
            vec![Range::new(1, 10), Range::new(20, 31)]
        );
    }
}
