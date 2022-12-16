use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::cmp::Ord;

pub fn first(input: &[String], target_y: isize) -> usize {
    djikstras(0, &parse_input(&input))
}

pub fn second(input: &[String], max_x_y: isize) -> usize {
    0
}

#[derive(PartialEq, Hash, Eq, Copy, Clone)]
struct State {
    cost: usize, 
    position: usize,
    minutes_remaining: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.minutes_remaining.cmp(&other.minutes_remaining))
            .reverse() // we want the largest cost actually
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn djikstras(start: usize, edges: &Vec<Valve>) -> usize {
    // Djikstra's! https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
    let mut distances: HashMap<usize, usize> = edges
        .iter()
        .map(|valve| (valve.position, 0))
        .collect();
    // distances.insert(start, 0);
    println!("distances {:?}", distances);

    let mut previouses: HashMap<usize, usize> = HashMap::new(); // notate the path taken so we can later calculate the cost? maybe

    let mut unvisited = BinaryHeap::new();
    unvisited.push(State {
        cost: 0,
        position: start,
        minutes_remaining: 30
    });

    while let Some(State { cost, position, minutes_remaining }) = unvisited.pop() {
        // if we're out of time we're done
        if minutes_remaining == 0 {
            return cost;
        }

        // we've gone down a bad path
        if cost < *distances.get(&position).unwrap() {
            continue;
        }

        // for each neighbor, see if we can find a way with a lower cost going through this node
        'neighbors: for neighbor in edges[position].neighbors.iter() {
            // TODO I think this is only valid if the edge hasn't already been visited on this path
            // NO, that's not right because the example uses previously visited nodes to navigate to others
            // so the trick is that the 'cost' doesn't increase if it's a previous
            let mut previouses_current = position;
            let found_previous = loop {
                let previous = previouses.get(&previouses_current);
                if previous.is_none() {
                    break false
                }
                if previous.unwrap() == neighbor {
                    break true
                }
                previouses_current =*previous.unwrap();
            };

            let next = State {
                cost: cost + if(found_previous) { 0 } else {((minutes_remaining - 2) * edges[*neighbor].flow_rate)},
                position: *neighbor,
                minutes_remaining: minutes_remaining - 2,
            };

            // if we have a better way to get there from here
            if next.cost >= *distances.get(&neighbor).unwrap() {
                println!("from {} to {} with cost {}", Valve::position_to_alpha(position), Valve::position_to_alpha(*neighbor), next.cost);
                unvisited.push(next);
                distances.insert(*neighbor, next.cost);
                previouses.insert(*neighbor, position);
            }
        }
    }

    // not reachable
    panic!("OH NO")
}

#[derive(Default, Clone)]
struct Valve{
    flow_rate: usize, 
    neighbors: Vec<usize>,
    position: usize
}

impl Valve {
    fn from(input: &String) -> Valve {
        let mut parts: Vec<&str> = input.split_whitespace().collect();
        let position = Valve::alpha_to_position(parts[1]);
        let flow_rate = Some(parts[4]).and_then(|p| p.strip_prefix("rate=")).and_then(|p| p.strip_suffix(";")).and_then(|p| p.parse().ok()).unwrap();
        let mut neighbors = vec!();
        for neighbor_alpha in parts[9..].iter() {
            neighbors.push(Valve::alpha_to_position(neighbor_alpha.strip_suffix(",").unwrap_or(&neighbor_alpha)));
        }
        Valve { position, flow_rate, neighbors}
    }

    fn alpha_to_position(alpha: &str) -> usize {
        let mut chars = alpha.chars();
        ((26 * ((chars.next().unwrap() as isize) - ('A' as isize))) + ((chars.next().unwrap() as isize) - ('A' as isize))) as usize
    }

    fn position_to_alpha(position: usize) -> String {
        vec!(
            char::from(((position / 26) + 65) as u8),
            char::from(((position % 26) + 65) as u8),
        ).iter().collect()
    }
}

fn parse_input(input: &[String]) -> Vec<Valve> {
    let mut valves = vec![Default::default(); 26 * 26];
    for i in input.iter() {
        let valve = Valve::from(i);
        let position = valve.position;
        valves[position] = valve;
    }
    valves
}

#[cfg(test)]
mod tests {

    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
            "Valve HH has flow rate=22; tunnel leads to valve GG",
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
            "Valve JJ has flow rate=21; tunnel leads to valve II",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn test_parse_input() {
        let input = example();
        let parsed = parse_input(&input);

        assert_eq!(parsed[0].flow_rate, 0);
        assert_eq!(parsed[0].position, 0);
        assert_eq!(parsed[0].neighbors, vec!(81, 216, 27));

        assert_eq!(parsed[81].flow_rate, 20);
        assert_eq!(parsed[81].position, 81);
        assert_eq!(parsed[81].neighbors, vec!(54, 0, 108));
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input, 10);
        assert_eq!(result, 1651);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input, 20);
        assert_eq!(result, 0);
    }
}
