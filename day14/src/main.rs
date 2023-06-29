use std::fs;

fn main() {
    let filename = "input.txt";
    let input = match fs::read_to_string(filename) {
        Ok(file) => file,
        Err(error) => panic!("Error reading file '{}': {}", filename, error),
    };

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut field = Field::from_str(input);

    field.count_sand_drops_until_abyss()
}

fn part2(input: &str) -> i32 {
    let _field = Field::from_str(input);

    -1
}

#[derive(Clone)]
enum Material {
    Air,
    Rock,
    Sand,
    Spawn,
}

fn parse_lines(input: &str) -> Vec<Vec<(usize, usize)>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let coords: Vec<usize> = point
                        .split(",")
                        .map(|coord| coord.parse::<usize>().unwrap())
                        .collect();
                    (coords[0], coords[1])
                })
                .collect()
        })
        .collect()
}

#[derive(Clone)]
struct Field {
    field: Vec<Vec<Material>>,
    xmin: usize,
    xmax: usize,
    ymax: usize,
}

impl Field {
    const SPAWN: (usize, usize) = (500, 0);

    fn from_str(input: &str) -> Field {
        let lines = parse_lines(input);
        let xmin = lines
            .iter()
            .map(|line| line.iter().map(|(x, _)| x).min().unwrap())
            .min()
            .unwrap();
        let xmax = lines
            .iter()
            .map(|line| line.iter().map(|(x, _)| x).max().unwrap())
            .max()
            .unwrap();
        let ymax = lines
            .iter()
            .map(|line| line.iter().map(|(_, y)| y).max().unwrap())
            .max()
            .unwrap();

        let mut field = Field {
            field: vec![vec![Material::Air; ymax + 1]; xmax + 1],
            xmin: *xmin,
            xmax: *xmax,
            ymax: *ymax,
        };

        if !field.contains(Field::SPAWN) {
            panic!(
                "Spawn point at {:?} not in field\n Field has dimensions {}-{} x 0-{}",
                Field::SPAWN,
                field.xmin,
                field.xmax,
                field.ymax
            );
        }

        field.set_rock_lines(lines);

        // draw spawn point into field
        if !matches!(field.field[Field::SPAWN.0][Field::SPAWN.1], Material::Air) {
            panic!("Spawn point at {:?} is not air!", Field::SPAWN);
        }

        field.field[Field::SPAWN.0][Field::SPAWN.1] = Material::Spawn;

        field
    }

    fn contains(&self, (x, y): (usize, usize)) -> bool {
        x >= self.xmin && x <= self.xmax && y <= self.ymax
    }

    fn pretty_print(&self) -> String {
        let mut result = String::new();
        result.reserve(self.field.len() * (self.field[0].len() + 2));

        for y in 0..=self.ymax {
            if y > 0 {
                result.push('\n');
            }
            for x in self.xmin..=self.xmax {
                let c = match self.field[x][y] {
                    Material::Air => '.',
                    Material::Rock => '#',
                    Material::Sand => 'o',
                    Material::Spawn => '+',
                };
                result.push(c);
            }
        }

        result
    }

    fn set_rock_lines(&mut self, lines: Vec<Vec<(usize, usize)>>) {
        // draw rock lines into field
        for line in lines {
            for pair in line.windows(2) {
                let start = pair[0];
                let end = pair[1];

                self.rock_line(start, end);
            }
        }
    }

    fn rock_line(&mut self, start: (usize, usize), end: (usize, usize)) {
        let (x, y) = start;
        let (xend, yend) = end;

        use std::cmp::Ordering::*;

        match (x.cmp(&xend), y.cmp(&yend)) {
            // single point
            (Equal, Equal) => {
                self.field[x][y] = Material::Rock;
            }

            // vertical line, starting at top (i.e. lower y coordinate)
            (Equal, Less) => {
                for y in y..=yend {
                    self.field[x][y] = Material::Rock;
                }
            }

            // vertical line, starting at bottom (i.e. higher y coordinate)
            (Equal, Greater) => {
                for y in yend..=y {
                    self.field[x][y] = Material::Rock;
                }
            }

            // horizontal line, starting at left (i.e. lower x coordinate)
            (Less, Equal) => {
                for x in x..=xend {
                    self.field[x][y] = Material::Rock;
                }
            }

            // horizontal line, starting at right (i.e. higher x coordinate)
            (Greater, Equal) => {
                for x in xend..=x {
                    self.field[x][y] = Material::Rock;
                }
            }

            // any kind of diagonal line
            (_, _) => panic!("Diagonal lines not supported yet"),
        }
    }

    fn sand_grain_step(&self, pos: (usize, usize)) -> DropResult {
        let (current_x, current_y) = pos;

        // straight down
        let x = current_x;
        let y = current_y + 1;

        if !self.contains((x, y)) {
            return DropResult::Abyss;
        }

        if matches!(self.field[x][y], Material::Air) {
            return DropResult::Falling(x, y);
        }

        // left and down
        let x = current_x - 1;

        if !self.contains((x, y)) {
            return DropResult::Abyss;
        }

        if matches!(self.field[x][y], Material::Air) {
            return DropResult::Falling(x, y);
        }

        // right and down
        let x = current_x + 1;

        if !self.contains((x, y)) {
            return DropResult::Abyss;
        }

        if matches!(self.field[x][y], Material::Air) {
            return DropResult::Falling(x, y);
        }

        DropResult::Stuck(current_x, current_y)
    }

    fn drop_sand(&mut self) -> DropResult {
        let mut pos = Field::SPAWN;

        loop {
            match self.sand_grain_step(pos) {
                DropResult::Abyss => return DropResult::Abyss,
                DropResult::Falling(x, y) => {
                    pos = (x, y);
                }
                DropResult::Stuck(x, y) => {
                    self.field[x][y] = Material::Sand;
                    return DropResult::Stuck(x, y);
                }
            }
        }
    }

    fn count_sand_drops_until_abyss(&mut self) -> usize {
        let mut count = 0;

        loop {
            match self.drop_sand() {
                DropResult::Abyss => return count,
                DropResult::Stuck(_, _) => count += 1,
                _ => panic!("Unexpected result"),
            }
        }
    }
}

enum DropResult {
    Falling(usize, usize),
    Stuck(usize, usize),
    Abyss,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let ref_snapshots: Vec<(usize, &str)> = vec![
            (0, {
                "......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########."
            }),
            (1, {
                "......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
......o.#.
#########."
            }),
            (2, {
                "......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
.....oo.#.
#########."
            }),
            (5, {
                "......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
......o.#.
....oooo#.
#########."
            }),
            (22, {
                "......+...
..........
......o...
.....ooo..
....#ooo##
....#ooo#.
..###ooo#.
....oooo#.
...ooooo#.
#########."
            }),
            (24, {
                "......+...
..........
......o...
.....ooo..
....#ooo##
...o#ooo#.
..###ooo#.
....oooo#.
.o.ooooo#.
#########."
            }),
        ];

        let mut field = Field::from_str(INPUT);

        assert!(field.contains((500, 0)));
        assert_eq!(field.xmin, 494);
        assert_eq!(field.xmax, 503);
        assert_eq!(field.ymax, 9);

        assert_eq!(field.contains((field.xmin - 1, 0)), false);
        assert_eq!(field.contains((field.xmax + 1, 0)), false);
        assert_eq!(field.contains((field.xmin, field.ymax + 1)), false);

        assert_eq!(0, ref_snapshots[0].0);
        assert_eq!(field.pretty_print(), ref_snapshots[0].1);

        assert!(matches!(field.drop_sand(), DropResult::Stuck(500, 8)));

        assert_eq!(1, ref_snapshots[1].0);
        assert_eq!(field.pretty_print(), ref_snapshots[1].1);

        field.drop_sand();
        assert_eq!(2, ref_snapshots[2].0);
        assert_eq!(field.pretty_print(), ref_snapshots[2].1);

        for _ in 3..=5 {
            field.drop_sand();
        }

        assert_eq!(5, ref_snapshots[3].0);
        assert_eq!(field.pretty_print(), ref_snapshots[3].1);

        for _ in 6..=22 {
            field.drop_sand();
        }

        assert_eq!(22, ref_snapshots[4].0);
        assert_eq!(field.pretty_print(), ref_snapshots[4].1);

        for _ in 23..=24 {
            field.drop_sand();
        }
        assert_eq!(24, ref_snapshots[5].0);
        assert_eq!(field.pretty_print(), ref_snapshots[5].1);

        assert!(matches!(field.drop_sand(), DropResult::Abyss));

        let mut field = Field::from_str(INPUT);

        assert_eq!(24, field.count_sand_drops_until_abyss());
    }

    #[test]
    fn test_part2() {}

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
}
