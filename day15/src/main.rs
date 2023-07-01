use std::{
    collections::{HashMap, HashSet},
    fs,
};

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

fn part2(_input: &str) -> usize {
    0
}

type Position = (i32, i32);
type Beacon = Position;
type Sensor = Position;

fn beaconless_positions(sensor: &Sensor, range: i32, line: i32) -> Vec<Position> {
    let (x, y) = *sensor;
    let span = range - (line - y).abs();

    if span <= 0 {
        return vec![];
    }

    let mut pos: Position = (x - range - 1, line);

    while manhattan_distance(&pos, sensor) > range {
        pos.0 += 1;
    }

    let mut positions = vec![];
    while manhattan_distance(&pos, sensor) <= range {
        positions.push(pos);
        pos.0 += 1;
    }

    positions
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
        let mut blocked_positions = HashSet::new();

        self.sensors.iter().for_each(|(sensor, &range)| {
            blocked_positions.extend(beaconless_positions(sensor, range, line));
        });

        // no need to add sensors; they're inside their own range

        // remove known beacons, since they're known positions of beacons
        for beacon in &self.beacons {
            blocked_positions.remove(beacon);
        }

        blocked_positions.len()
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
    fn test_part2() {}

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
}
