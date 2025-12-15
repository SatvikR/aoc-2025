use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct JunctionBoxPair {
    first: (u64, u64, u64),
    second: (u64, u64, u64),
    distance: u64, // pray this doesn't case issues. why are f64's not Ord'able...
}

impl Ord for JunctionBoxPair {
    fn cmp(&self, other: &Self) -> Ordering {
        // flip orderings, want min-heap.
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for JunctionBoxPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> Result<Vec<(u64, u64, u64)>, String> {
    input
        .lines()
        .map(|l| {
            let coords = l
                .split(',')
                .map(|s| {
                    s.parse::<u64>()
                        .map_err(|e| format!("Unable to parse '{}' as u64: {}", s, e))
                })
                .collect::<Result<Vec<u64>, String>>()?;
            if coords.len() != 3 {
                return Err(format!("Expected 3 coordinates on each line."));
            }
            Ok((coords[0], coords[1], coords[2]))
        })
        .collect::<Result<Vec<(u64, u64, u64)>, String>>()
}

fn part1(input: &str, num_connections: u64) -> Result<String, String> {
    let coords = parse_input(input).map_err(|e| format!("Error parsing input: {}", e))?;
    let mut dists = BinaryHeap::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            dists.push(JunctionBoxPair {
                first: coords[i],
                second: coords[j],
                distance: (coords[j].0.abs_diff(coords[i].0).pow(2)
                    + coords[j].1.abs_diff(coords[i].1).pow(2)
                    + coords[j].2.abs_diff(coords[i].2).pow(2))
                .isqrt(),
            })
        }
    }
    let mut circuits = HashMap::<(u64, u64, u64), u64>::new();
    let mut circuits_rev = HashMap::<u64, Vec<(u64, u64, u64)>>::new();
    let mut next_circuit_id = 0_u64;

    let mut i = 0;
    while i < num_connections {
        let connection = dists.pop().ok_or_else(|| {
            format!(
                "Expected junction boxes to make {} connections.",
                num_connections
            )
        })?;
        i += 1;
        let first_circuit = circuits.get(&connection.first).and_then(|a| Some(*a));
        let second_circuit = circuits.get(&connection.second).and_then(|a| Some(*a));
        match (first_circuit, second_circuit) {
            (Some(a), Some(b)) => {
                if a == b {
                    continue;
                }
                // if in different circuits, map all of b onto a
                circuits_rev.remove(&b).unwrap().iter().for_each(|coord| {
                    circuits_rev.get_mut(&a).unwrap().push(*coord);
                    circuits.insert(*coord, a);
                });
            }
            (Some(a), None) => {
                circuits.insert(connection.second, a);
                circuits_rev.get_mut(&a).unwrap().push(connection.second);
            }
            (None, Some(b)) => {
                circuits.insert(connection.first, b);
                circuits_rev.get_mut(&b).unwrap().push(connection.first);
            }
            (None, None) => {
                circuits.insert(connection.first, next_circuit_id);
                circuits.insert(connection.second, next_circuit_id);
                circuits_rev.insert(next_circuit_id, vec![connection.first, connection.second]);
                next_circuit_id += 1;
            }
        }
    }
    let mut circuit_sizes = circuits_rev
        .iter()
        .map(|(_, coords)| coords.len() as u64)
        .collect::<Vec<u64>>();
    circuit_sizes.sort();
    Ok(circuit_sizes
        .iter()
        .rev()
        .take(3)
        .product::<u64>()
        .to_string())
}

fn part2(input: &str) -> Result<String, String> {
    let coords = parse_input(input).map_err(|e| format!("Error parsing input: {}", e))?;
    let mut dists = BinaryHeap::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            dists.push(JunctionBoxPair {
                first: coords[i],
                second: coords[j],
                distance: (coords[j].0.abs_diff(coords[i].0).pow(2)
                    + coords[j].1.abs_diff(coords[i].1).pow(2)
                    + coords[j].2.abs_diff(coords[i].2).pow(2))
                .isqrt(),
            })
        }
    }
    let mut circuits = HashMap::<(u64, u64, u64), u64>::new();
    let mut circuits_rev = HashMap::<u64, Vec<(u64, u64, u64)>>::new();
    let mut next_circuit_id = 0_u64;

    let mut last_connection_wall_dist = 0;
    while let Some(connection) = dists.pop() {
        let first_circuit = circuits.get(&connection.first).and_then(|a| Some(*a));
        let second_circuit = circuits.get(&connection.second).and_then(|a| Some(*a));
        match (first_circuit, second_circuit) {
            (Some(a), Some(b)) => {
                if a == b {
                    continue;
                }
                // if in different circuits, map all of b onto a
                circuits_rev.remove(&b).unwrap().iter().for_each(|coord| {
                    circuits_rev.get_mut(&a).unwrap().push(*coord);
                    circuits.insert(*coord, a);
                });
            }
            (Some(a), None) => {
                circuits.insert(connection.second, a);
                circuits_rev.get_mut(&a).unwrap().push(connection.second);
            }
            (None, Some(b)) => {
                circuits.insert(connection.first, b);
                circuits_rev.get_mut(&b).unwrap().push(connection.first);
            }
            (None, None) => {
                circuits.insert(connection.first, next_circuit_id);
                circuits.insert(connection.second, next_circuit_id);
                circuits_rev.insert(next_circuit_id, vec![connection.first, connection.second]);
                next_circuit_id += 1;
            }
        }
        last_connection_wall_dist = connection.first.0 * connection.second.0;
    }
    Ok(last_connection_wall_dist.to_string())
}

fn main() {
    let example_input =
        fs::read_to_string("./example.txt").expect("Unable to read from example.txt");
    let part1_input = fs::read_to_string("./p1.txt").expect("Unable to read from p1.txt");

    let _ = part1(&example_input, 10)
        .map(|s| println!("Answer to part 1 (example input): {}", s))
        .map_err(|err| println!("Error computing part 1 (example input): {}", err));
    let _ = part1(&part1_input, 1000)
        .map(|s| println!("Answer to part 1 (part 1 input): {}", s))
        .map_err(|err| println!("Error computing part 1 (part 1 input): {}", err));
    let _ = part2(&example_input)
        .map(|s| println!("Answer to part 2 (example input): {}", s))
        .map_err(|err| println!("Error computing part 2 (example input): {}", err));
    let _ = part2(&part1_input)
        .map(|s| println!("Answer to part 2 (part 1 input): {}", s))
        .map_err(|err| println!("Error computing part 2 (part 1 input): {}", err));
}
