use itertools::{Either, Itertools};
use sscanf::sscanf;
use std::collections::{HashMap, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let problem = ProblemStatement::from_str(input);

    problem.solve_part1()
}

fn part2(input: &str) -> usize {
    let problem = ProblemStatement::from_str(input);

    println!("  Calculation for part 2 will take a while...");

    problem.solve_part2()
}

// naming is coincidental
type ValveIndex = HashMap<String, usize>;

#[derive(Debug, PartialEq)]
struct Valve {
    id: usize,
    flow: usize,
    tunnels: Vec<usize>,
}

impl Valve {
    fn new(id: usize, flow: usize, tunnels: Vec<usize>) -> Self {
        Self { id, flow, tunnels }
    }
}

struct ProblemStatement {
    distances: Vec<Vec<usize>>,
    flows: Vec<usize>,
    closed_valves: Vec<usize>,
    starting_valve: usize,
}

impl ProblemStatement {
    fn from_str(input: &str) -> Self {
        // First, get indices
        let indices: ValveIndex = input
            .trim()
            .lines()
            .enumerate()
            .map(|(id, line)| match sscanf!(line, "Valve {str} has {str}") {
                Ok((name, _)) => {
                    assert_eq!(name.len(), 2);
                    (name.to_string(), id)
                }
                Err(_) => panic!("Valve has no id: {}", line),
            })
            .collect();

        let valves: Vec<Valve> = input
            .trim()
            .lines()
            .enumerate()
            .map(|(id, line)| ProblemStatement::parse_valve(line, id, &indices))
            .collect();

        ProblemStatement {
            distances: ProblemStatement::get_distance_matrix(&valves),
            flows: valves.iter().map(|valve| valve.flow).collect(),
            closed_valves: ProblemStatement::get_closed_valves(&valves),
            starting_valve: indices.get("AA").unwrap().to_owned(),
        }
    }

    fn parse_valve(line: &str, id: usize, indices: &ValveIndex) -> Valve {
        let line = line.trim();

        match sscanf!(
            line,
            "Valve {str} has flow rate={usize}; tunnels lead to valves {str}"
        ) {
            Ok((_name, flow_rate, tunnels)) => Valve::new(
                id,
                flow_rate,
                tunnels
                    .split(", ")
                    .map(|tunnel| indices.get(tunnel).unwrap().to_owned())
                    .collect(),
            ),
            Err(_) => match sscanf!(
                line,
                "Valve {str} has flow rate={usize}; tunnel leads to valve {str}"
            ) {
                Ok((_name, flow_rate, tunnel)) => {
                    Valve::new(id, flow_rate, vec![indices.get(tunnel).unwrap().to_owned()])
                }
                Err(_) => match sscanf!(line, "Valve {str} has flow rate={usize}") {
                    Ok((_name, flow_rate)) => Valve::new(id, flow_rate, vec![]),
                    Err(_) => panic!("Invalid input: {}", line),
                },
            },
        }
    }

    fn solve_part1(&self) -> usize {
        let time_left = 30;

        self.partial_solve_part1(self.starting_valve, time_left, &self.closed_valves)
    }

    fn partial_solve_part1(
        &self,
        current_valve: usize,
        time_left: usize,
        remaining_valves: &[usize],
    ) -> usize {
        if remaining_valves.is_empty() {
            return 0;
        }

        let mut best_solution = 0;

        // sort remaining valves by flow, descending
        let mut remaining_valves = remaining_valves.to_owned();
        remaining_valves.sort_by_key(|valve| self.flows[*valve]);
        remaining_valves.reverse();

        for (i, valve) in remaining_valves.iter().enumerate() {
            let distance_to_valve = self.distances[current_valve][*valve];
            if distance_to_valve > time_left {
                continue;
            }
            let new_time_left = time_left - distance_to_valve;

            if new_time_left > 0 {
                let mut new_remaining_valves = remaining_valves.to_owned();
                new_remaining_valves.remove(i);

                let cumulative_flow_change = new_time_left * self.flows[*valve];
                let new_solution = cumulative_flow_change
                    + self.partial_solve_part1(*valve, new_time_left, &new_remaining_valves);

                if new_solution > best_solution {
                    best_solution = new_solution;
                }
            }
        }

        best_solution
    }

    fn get_distance_matrix(valves: &[Valve]) -> Vec<Vec<usize>> {
        let mut matrix = vec![vec![usize::MAX; valves.len()]; valves.len()];

        // naive algorithm:
        // For each index, find the neighbor indices, and increase the distance by 1 each time.
        // Stop when no more indices are found. Skip indices with distance != usize::MAX
        //
        // I know there's better algorithms, but this is good enough for the problem at hand, and
        // that's good enough for me

        #[allow(clippy::needless_range_loop)]
        for i in 0..valves.len() {
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

            queue.push_back((i, 1));
            matrix[i][i] = 1;

            while let Some((index, distance)) = queue.pop_front() {
                for j in &valves[index].tunnels {
                    if matrix[i][*j] == usize::MAX {
                        matrix[i][*j] = distance + 1;
                        queue.push_back((*j, distance + 1));
                    }
                }
            }
        }

        matrix
    }

    fn get_closed_valves(valves: &[Valve]) -> Vec<usize> {
        valves
            .iter()
            .enumerate()
            .filter_map(|(i, valve)| if valve.flow > 0 { Some(i) } else { None })
            .collect()
    }

    fn solve_part2(&self) -> usize {
        let time_left = 26;

        let mut max_flow = 0;

        let number_of_divisions = 2_usize.pow(self.closed_valves.len() as u32);

        for division_index in 0..number_of_divisions {
            let (elephant_valves, me_valves): (Vec<usize>, Vec<usize>) = self
                .closed_valves
                .iter()
                .enumerate()
                .partition_map(|(i, valve)| {
                    if 1 << i & division_index == 0 {
                        Either::Left(valve)
                    } else {
                        Either::Right(valve)
                    }
                });

            #[cfg(test)]
            println!("Me: {me_valves:?}\nElephant: {elephant_valves:?}\n");

            let elephant_flow =
                self.partial_solve_part1(self.starting_valve, time_left, &elephant_valves);
            let me_flow = self.partial_solve_part1(self.starting_valve, time_left, &me_valves);

            max_flow = max_flow.max(elephant_flow + me_flow);
        }

        max_flow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let problem = ProblemStatement::from_str(INPUT);

        assert_eq!(problem.flows[0], 0);

        assert_eq!(problem.distances[0], [1, 2, 3, 2, 3, 4, 5, 6, 2, 3]);
        assert_eq!(problem.distances[1], [2, 1, 2, 3, 4, 5, 6, 7, 3, 4]);

        assert_eq!(problem.closed_valves, [1, 2, 3, 4, 7, 9]);

        assert_eq!(
            ProblemStatement::from_str("Valve AA has flow rate=0").solve_part1(),
            0
        );
        assert_eq!(
            ProblemStatement::from_str("Valve AA has flow rate=1").solve_part1(),
            29
        );

        assert_eq!(problem.solve_part1(), 1651);
    }

    #[test]
    fn test_part2() {
        let problem = ProblemStatement::from_str(INPUT);

        assert_eq!(problem.solve_part2(), 1707);
    }

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
}
