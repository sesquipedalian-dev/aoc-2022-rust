use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn first(input: &[String]) -> usize {
    let mut nodes = parse_input(&input);
    nodes.sort_by(|a, b| a.flow_rate.cmp(&b.flow_rate).reverse());
    println!("sorted nodes by flow rate {:?}", nodes);

    let all_distances: HashMap<usize, HashMap<usize, usize>> = nodes
        .iter()
        .flat_map(|node| {
            if node.neighbors.is_empty() {
                None
            } else {
                Some((node.position, djikstras(node.position, &nodes)))
            }
        })
        .collect();
    println!(
        "for each node, all shortest paths to other nodes {:?}",
        all_distances
    );

    let dfs_paths = dfs(&all_distances, &mut nodes, 0, 30);
    println!("dfs paths {:?}", dfs_paths);

    0
}

pub fn second(input: &[String]) -> usize {
    0
}

#[derive(PartialEq, Eq, Clone)]
struct State {
    cost: usize,
    position: usize,
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
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn djikstras(start: usize, edges: &Vec<Valve>) -> HashMap<usize, usize> {
    // Djikstra's! https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
    let mut distances: HashMap<usize, usize> =
        edges.iter().map(|valve| (valve.position, 99)).collect();
    distances.insert(start, 0);
    println!("distances {:?}", distances);

    // let mut previouses: HashMap<usize, usize> = HashMap::new(); // notate the path taken so we can later calculate the cost? maybe

    let mut unvisited = BinaryHeap::new();
    unvisited.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = unvisited.pop() {
        println!(
            "current node {} {}",
            cost,
            Valve::position_to_alpha(position)
        );

        // // we've gone down a bad path
        // if cost > *distances.get(&position).unwrap() {
        //     println!("this is a bad path?");
        //     continue;
        // }

        // for each neighbor, see if we can find a way with a lower cost going through this node
        for neighbor in edges[position].neighbors.iter() {
            println!(
                "considering neighbor? {:?}",
                Valve::position_to_alpha(*neighbor)
            );
            let next = State {
                cost: cost + 1,
                position: *neighbor,
            };

            // if we have a better way to get there from here
            if next.cost < *distances.get(neighbor).unwrap() {
                unvisited.push(next);
                distances.insert(*neighbor, cost + 1);
            }
            // previouses.insert(*neighbor, position);
        }
    }

    // not reachable
    distances
}

fn dfs(
    distances: &HashMap<usize, HashMap<usize, usize>>,
    nodes: &Vec<Valve>,
    position: usize,
    remaining_time: usize,
) -> Vec<Vec<usize>> {
    let mut paths: Vec<Vec<usize>> = vec![];

    fn dfs_recurse(
        nodes: &Vec<Valve>,
        distances: &HashMap<usize, HashMap<usize, usize>>,
        paths: &mut Vec<Vec<usize>>,
        position: usize,
        remaining_time: usize,
        visited: Vec<usize>,
    ) {
        if remaining_time <= 0 {
            println!("out of time");
            return;
        }

        for (next, distance) in distances.get(&position).unwrap().iter() {
            if nodes[*next].flow_rate == 0 {
                // TODO oh darn we've messed up the vector / alpha mapping again somehow
                println!(
                    "skipping no flow rate sob {:?} {:?}",
                    Valve::position_to_alpha(*next),
                    nodes[*next]
                );
                continue;
            }

            if visited.contains(next) {
                println!("skipping already visited");
                continue;
            }

            if remaining_time - distance - 1 <= 0 {
                println!("skipping one we don't have time to get to");
                continue;
            }

            let mut new_visited = visited.clone();
            new_visited.push(*next);
            dfs_recurse(
                &nodes,
                &distances,
                paths,
                *next,
                remaining_time - distance - 1,
                new_visited,
            );
        }
        paths.push(visited);
    }

    dfs_recurse(
        &nodes,
        &distances,
        &mut paths,
        position,
        remaining_time,
        vec![],
    );
    paths
}

#[derive(Default, Clone, Debug)]
struct Valve {
    flow_rate: usize,
    neighbors: Vec<usize>,
    position: usize,
}

impl Valve {
    fn from(input: &String) -> Valve {
        let mut parts: Vec<&str> = input.split_whitespace().collect();
        let position = Valve::alpha_to_position(parts[1]);
        let flow_rate = Some(parts[4])
            .and_then(|p| p.strip_prefix("rate="))
            .and_then(|p| p.strip_suffix(";"))
            .and_then(|p| p.parse().ok())
            .unwrap();
        let mut neighbors = vec![];
        for neighbor_alpha in parts[9..].iter() {
            neighbors.push(Valve::alpha_to_position(
                neighbor_alpha.strip_suffix(",").unwrap_or(&neighbor_alpha),
            ));
        }
        Valve {
            position,
            flow_rate,
            neighbors,
        }
    }

    fn alpha_to_position(alpha: &str) -> usize {
        let mut chars = alpha.chars();
        ((26 * ((chars.next().unwrap() as isize) - ('A' as isize)))
            + ((chars.next().unwrap() as isize) - ('A' as isize))) as usize
    }

    fn position_to_alpha(position: usize) -> String {
        vec![
            char::from(((position / 26) + 65) as u8),
            char::from(((position % 26) + 65) as u8),
        ]
        .iter()
        .collect()
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
        let result = first(&input);
        assert_eq!(result, 1651);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 0);
    }
}
