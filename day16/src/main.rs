use sscanf::sscanf;
use std::collections::{HashMap, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let (valves, indices) = parse_valves(input);

    let starting_valve = indices.get("AA").unwrap().to_owned();

    find_best_solution(&valves, starting_valve)
}

fn part2(_input: &str) -> usize {
    0
}

type ValveIndex<'a> = HashMap<&'a str, usize>;

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

fn parse_valves(input: &str) -> (Vec<Valve>, ValveIndex) {
    // First, get indices
    let indices: ValveIndex = input
        .trim()
        .lines()
        .enumerate()
        .map(|(id, line)| match sscanf!(line, "Valve {str} has {str}") {
            Ok((name, _)) => {
                assert_eq!(name.len(), 2);
                (name, id)
            }
            Err(_) => panic!("Valve has no id: {}", line),
        })
        .collect();

    let valves = input
        .trim()
        .lines()
        .enumerate()
        .map(|(id, line)| parse_valve(line, id, &indices))
        .collect();

    (valves, indices)
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

fn get_distance_matrix(valves: &[Valve]) -> Vec<Vec<usize>> {
    let mut matrix = vec![vec![usize::MAX; valves.len()]; valves.len()];

    // naive algorithm:
    // For each index, find the neighbor indices, and increase the distance by 1 each time. Stop when no more indices are found. Skip indices with distance != usize::MAX

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

#[cfg(test)]
fn calculate_cumulative_flow(
    current_valve: usize,
    valve_sequence: &[usize],
    valves: &[Valve],
    distances: &[Vec<usize>],
    time_left: usize,
) -> usize {
    if valve_sequence.is_empty() {
        return 0;
    }

    let next_valve_index = valve_sequence[0];
    let remaining_valves = &valve_sequence[1..];

    let new_time_left = time_left - distances[current_valve][next_valve_index];

    (new_time_left * valves[next_valve_index].flow)
        + calculate_cumulative_flow(
            next_valve_index,
            remaining_valves,
            valves,
            distances,
            new_time_left,
        )
}

fn get_valves_that_can_be_opened(valves: &[Valve]) -> Vec<usize> {
    valves
        .iter()
        .enumerate()
        .filter_map(|(i, valve)| if valve.flow > 0 { Some(i) } else { None })
        .collect()
}

fn get_flows(valves: &[Valve]) -> Vec<usize> {
    valves.iter().map(|valve| valve.flow).collect()
}

fn find_best_solution(valves: &[Valve], starting_valve: usize) -> usize {
    let distances = get_distance_matrix(valves);
    let remaining_valves = get_valves_that_can_be_opened(valves);
    let flows = get_flows(valves);
    let time_left = 30;

    find_solution(
        starting_valve,
        time_left,
        &flows,
        &distances,
        &remaining_valves,
    )
}

fn find_solution(
    current_valve: usize,
    time_left: usize,
    flows: &[usize],
    distances: &[Vec<usize>],
    remaining_valves: &[usize],
) -> usize {
    if remaining_valves.is_empty() {
        return 0;
    }

    let mut best_solution = 0;

    // sort remaining valves by flow, descending
    let mut remaining_valves = remaining_valves.to_owned();
    remaining_valves.sort_by_key(|valve| flows[*valve]);
    remaining_valves.reverse();

    for (i, valve) in remaining_valves.iter().enumerate() {
        let distance_to_valve = distances[current_valve][*valve];
        if distance_to_valve > time_left {
            continue;
        }
        let new_time_left = time_left - distance_to_valve;

        if new_time_left > 0 {
            let mut new_remaining_valves = remaining_valves.to_owned();
            new_remaining_valves.remove(i);

            let cumulative_flow_change = new_time_left * flows[*valve];
            let new_solution = cumulative_flow_change
                + find_solution(
                    *valve,
                    new_time_left,
                    flows,
                    distances,
                    &new_remaining_valves,
                );

            if new_solution > best_solution {
                best_solution = new_solution;
            }
        }
    }

    best_solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (valves, _indices) = parse_valves(INPUT);

        assert_eq!(valves[0], Valve::new(0, 0, vec![3, 8, 1]));

        let distances = get_distance_matrix(&valves);
        assert_eq!(distances[0], [1, 2, 3, 2, 3, 4, 5, 6, 2, 3]);
        assert_eq!(distances[1], [2, 1, 2, 3, 4, 5, 6, 7, 3, 4]);

        assert_eq!(
            calculate_cumulative_flow(0, &[0], &valves, &distances, 30),
            0
        );

        assert_eq!(
            calculate_cumulative_flow(0, &[2], &valves, &distances, 30),
            27 * 2
        );

        assert_eq!(
            calculate_cumulative_flow(0, &[3, 1, 9, 7, 4, 2], &valves, &distances, 30),
            1651
        );

        assert_eq!(get_valves_that_can_be_opened(&valves), [1, 2, 3, 4, 7, 9]);

        let starting_valve = _indices.get("AA").unwrap().to_owned();
        assert_eq!(find_best_solution(&valves, starting_valve), 1651);

        assert_eq!(find_best_solution(&[Valve::new(0, 0, vec![])], 0), 0);
        assert_eq!(find_best_solution(&[Valve::new(0, 1, vec![])], 0), 29);
    }

    #[test]
    fn test_part2() {}

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
