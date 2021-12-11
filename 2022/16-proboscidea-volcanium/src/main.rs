use std::collections::{VecDeque, HashMap, HashSet};
use std::env::args;

fn main() {
    let argv: Vec<String> = args().collect();

    if argv.len() < 2 {
        println!("usage: cargo run -- puzzle-input");
        std::process::exit(1);
    }

    let tunnels = make_tunnels(&std::fs::read_to_string(&argv[1]).unwrap());

    let starting_label = *tunnels.name_to_label.get("AA").unwrap();
    let dists = tunnels.distances();

    let nonzero_labels = (0..tunnels.valves.len())
        .map(|valve_label| (valve_label, &tunnels.valves[valve_label]))
        .filter(|(_, valve)| valve.flow_rate != 0)
        .map(|(valve_label, _)| valve_label)
        .collect::<Vec<usize>>();

    let mut best_flow = 0;
    for &label in &nonzero_labels {
        let travel_time = dists[starting_label][label].unwrap(); // Assumes connected graph.
        best_flow = std::cmp::max(
            best_flow,
            tunnels.release_valves(label, 30 - travel_time, &mut HashSet::new(), &dists)
        );
    }

    println!("part one: {}", best_flow);

    let mut best_flow = 0;
    for i in 0..nonzero_labels.len() {
        for j in (i + 1)..nonzero_labels.len() {
            let me_label = nonzero_labels[i];
            let elephant_label = nonzero_labels[j];
            let me_travel_time = dists[starting_label][me_label].unwrap(); // Assumes connected graph.
            let elephant_travel_time = dists[starting_label][elephant_label].unwrap(); // Ditto.
            best_flow = std::cmp::max(
                best_flow,
                tunnels.release_valves_with_elephant(
                    (me_label, elephant_label),
                    (26 - me_travel_time, 26 - elephant_travel_time),
                    &mut HashSet::new(), &dists
                )
            );
        }
    }

    println!("part two: {}", best_flow);
}

struct Tunnels {
    name_to_label: HashMap<String, usize>,
    edges: Vec<Vec<usize>>,
    valves: Vec<Valve>
}

#[derive(Debug)]
struct Valve {
    _name: String,
    flow_rate: isize,
}

fn make_tunnels(puzzle: &str) -> Tunnels
{
    let mut name_to_label = HashMap::new();
    let mut edges = Vec::new();
    let mut valves: Vec<Valve> = Vec::new();
    let parsed_valves = &read_scan(puzzle);

    for (name, flow_rate, _) in parsed_valves {
        let label = valves.len();
        valves.push(Valve { _name: name.to_string(), flow_rate: *flow_rate });
        name_to_label.insert(name.to_string(), label);
    }

    for (_, _, neighbor_names) in parsed_valves {
        let neighbors = neighbor_names.into_iter()
            .flat_map(|name| name_to_label.get(name).map(|&label| label))
            .collect();
        edges.push(neighbors);
    }

    Tunnels { name_to_label, edges, valves }
}

impl Tunnels {
    fn distance_to(&self, starting_label: usize) -> Vec<Option<isize>> {
        let mut distances = vec![None; self.valves.len()];

        // We do not have self loops in our tunnels. There is no tunnel connecting a valve to itself.
        distances[starting_label] = None;

        let mut to_explore: VecDeque<(usize, isize)> = VecDeque::from([ (starting_label, 0) ]);
        let mut discovered = HashSet::from([ starting_label ]);

        while !to_explore.is_empty() {
            let (valve_label, distance) = to_explore.pop_front().unwrap();
            for &neighbor_label in &self.edges[valve_label] {
                if !discovered.contains(&neighbor_label) {
                    discovered.insert(neighbor_label);
                    to_explore.push_back((neighbor_label, distance + 1));
                    distances[neighbor_label] = Some(distance + 1);
                }
            }
        }

        distances
    }

    fn distances(&self) -> Vec<Vec<Option<isize>>> {
        (0..self.valves.len()).map(|valve_label| self.distance_to(valve_label)).collect()
    }

    fn release_valves_with_elephant(&self, (me_label, elephant_label): (usize, usize), (me_time, elephant_time): (isize, isize), released: &mut HashSet<usize>, dists: &Vec<Vec<Option<isize>>>) -> isize {
        // Assumes `me_label` and `elephant_label` has a non-zero flow rate.

        let me_valve = &self.valves[me_label];
        let elephant_valve = &self.valves[elephant_label];

        // use std::ops::Deref;
        // println!("release_valves(({}, {}) ({}, {}) {:?})", me_valve._name, elephant_valve._name, me_time, elephant_time,
        //     released.iter().map(|&label| self.valves[label]._name.deref()).collect::<Vec<&str>>());

        if me_time <= 0 && elephant_time <= 0 {
            return 0;
        }

        if me_time <= 0 {
            let flow = self.release_valves(elephant_label, elephant_time, released, dists);
            return flow;
        }

        if elephant_time <= 0 {
            let flow = self.release_valves(me_label, me_time, released, dists);
            return flow;
        }

        released.extend([ me_label, elephant_label ]);

        let reachable_by_me = dists[me_label].iter().enumerate()
            .flat_map(|(neighbor_label, &distance)| {
                if let Some(distance) = distance {
                    Some((neighbor_label, distance))
                } else {
                    None
                }
            })
            .filter(|(neighbor_label, _)| !released.contains(neighbor_label))
            .filter(|&(neighbor_label, _)| self.valves[neighbor_label].flow_rate != 0)
            .collect::<Vec<(usize, isize)>>();

        let reachable_by_elephant = dists[elephant_label].iter().enumerate()
            .flat_map(|(neighbor_label, &distance)| {
                if let Some(distance) = distance {
                    Some((neighbor_label, distance))
                } else {
                    None
                }
            })
            .filter(|(neighbor_label, _)| !released.contains(neighbor_label))
            .filter(|&(neighbor_label, _)| self.valves[neighbor_label].flow_rate != 0)
            .collect::<Vec<(usize, isize)>>();

        let mut neighbor_flow = 0;
        for (me_neighbor_label, me_neighbor_distance) in reachable_by_me {
            for &(elephant_neighbor_label, elephant_neighbor_distance) in &reachable_by_elephant {
                if me_neighbor_label != elephant_neighbor_label {
                    neighbor_flow = std::cmp::max(
                        self.release_valves_with_elephant(
                            (me_neighbor_label, elephant_neighbor_label),
                            (me_time - 1 - me_neighbor_distance, elephant_time - 1 - elephant_neighbor_distance),
                            released, dists
                        ),
                        neighbor_flow
                    );
                }
            }
        }

        released.remove(&me_label);
        released.remove(&elephant_label);

        let flow = me_valve.flow_rate * (me_time - 1)
            + elephant_valve.flow_rate * (elephant_time - 1)
            + neighbor_flow;

        flow
    }

    fn release_valves(&self, valve_label: usize, time_left: isize, released: &mut HashSet<usize>, dists: &Vec<Vec<Option<isize>>>) -> isize {
        // We have two choices. We can release the valve if it is not already released, or skip the
        // valve.

        // Assumes `valve_label` has a non-zero flow rate.

        let valve = &self.valves[valve_label];

        // use std::ops::Deref;
        // println!("release_valves({}, {}, {:?})", valve._name, time_left,
        //     released.iter().map(|&label| self.valves[label]._name.deref()).collect::<Vec<&str>>());

        if time_left <= 0 {
            return 0;
        }

        // Optimization: We don't know which valve we need to release next. But we do know that we
        // must take the shortest path there. We compute these distances in advance. This enables us
        // to check only a single path to the next valve we release instead of all of them. Still
        // too slow.

        // We must *immediately* release the valve we choose next, following the shortest series of
        // tunnels to reach it. If we instead skipped over this valve, and selected another valve,
        // we may not have selected the shortest path to this new valve. Triangle inequality.

        released.insert(valve_label);

        let reachable = dists[valve_label].iter().enumerate()
            .flat_map(|(neighbor_label, &distance)| {
                if let Some(distance) = distance {
                    Some((neighbor_label, distance))
                } else {
                    None
                }
            })
            .filter(|&(neighbor_label, _)| {
                let neighbor_valve = &self.valves[neighbor_label];
                !released.contains(&neighbor_label) && neighbor_valve.flow_rate != 0
            })
            .collect::<Vec<(usize, isize)>>();

        let neighbor_flow = reachable.iter()
            .map(|&(neighbor_label, distance)| {
                self.release_valves(neighbor_label, time_left - 1 - distance, released, dists)
            })
            .max()
            .unwrap_or_default();

        released.remove(&valve_label);

        let flow = valve.flow_rate * (time_left - 1) + neighbor_flow;

        flow
    }

    fn _release_valves(&self, valve_label: usize, time_left: isize, released: &mut HashSet<usize>) -> isize {
        // We have two choices. We can release the valve if it is not already released, or skip the
        // valve.

        let valve = &self.valves[valve_label];

        // use std::ops::Deref;
        // println!("release_valves({}, {}, {:?})", valve._name, time_left,
        //     released.iter().map(|&label| self.valves[label]._name.deref()).collect::<Vec<&str>>());

        // Optimization: If we've opened all of the valves, we can obtain no more flow.
        if time_left <= 0 || released.len() == self.valves.len() {
            return 0;
        }

        let flow_when_skipped = self.edges[valve_label].iter()
            .map(|&neighbor_label| self._release_valves(neighbor_label, time_left - 1, released))
            .max()
            .unwrap_or_default();

        if released.contains(&valve_label) {
            flow_when_skipped
        } else {
            let neighbor_flow_when_released = self.edges[valve_label].iter()
                .map(|&neighbor_label| {
                    released.insert(valve_label);
                    let total = self._release_valves(neighbor_label, time_left - 2, released);
                    released.remove(&valve_label);
                    total
                })
                .max()
                .unwrap_or_default();

            let flow_when_released = valve.flow_rate * (time_left - 1) + neighbor_flow_when_released;

            std::cmp::max(flow_when_skipped, flow_when_released)
        }
    }
}

fn read_scan(puzzle: &str) -> Vec<(String, isize, Vec<String>)>
{
    puzzle.lines().flat_map(read_valve).collect()
}

fn read_valve(line: &str) -> Option<(String, isize, Vec<String>)>
{
    let mut parts = line.split("; ");

    let left = &mut parts.next()?.split_whitespace();
    let valve_label = left.skip(1).next()?.to_string();
    let flow_rate: isize = left.skip(2).next()?.strip_prefix("rate=")?.parse().ok()?;

    let right = &mut parts.next()?;
    let edges: Vec<String> = right.strip_prefix("tunnel leads to valve ")
        .or(right.strip_prefix("tunnels lead to valves "))?
        .split(", ")
        .map(|s| s.to_string())
        .collect();

    Some((valve_label, flow_rate, edges))
}

#[test]
fn read_first_valve()
{
    assert_eq!(
        read_valve("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"),
        Some(("AA".to_string(), 0, vec!["DD".to_string(), "II".to_string(), "BB".to_string()]))
    );
}

#[test]
fn read_hh_valve()
{
    assert_eq!(
        read_valve("Valve HH has flow rate=22; tunnel leads to valve GG"),
        Some(("HH".to_string(), 22, vec!["GG".to_string()]))
    );
}

#[test]
fn distance_from_start_in_tiny_tunnels()
{
    let tiny_tunnels = r"Valve AA has flow rate=0; tunnels lead to valves BB
Valve BB has flow rate=3; tunnels lead to valves CC, DD
Valve CC has flow rate=6; tunnels lead to valves DD
Valve DD has flow rate=9; tunnels lead to valves BB, CC";

    let tunnels = make_tunnels(tiny_tunnels);
    let starting_label = *tunnels.name_to_label.get("AA").unwrap();

    assert_eq!(
        tunnels.distance_to(starting_label),
        vec![None, Some(1), Some(2), Some(2)]
    );
}

#[test]
fn distance_from_start_in_sample_tunnels()
{
    let sample_tunnels = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    let tunnels = make_tunnels(sample_tunnels);
    let starting_label = *tunnels.name_to_label.get("AA").unwrap();

    assert_eq!(
        tunnels.distance_to(starting_label),
        vec![None, Some(1), Some(2), Some(1), Some(2), Some(3), Some(4), Some(5), Some(1), Some(2)]
    );
}
