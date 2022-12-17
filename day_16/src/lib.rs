use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::cmp::Ord;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn first(input: &[String]) -> usize {
    let mut nodes = parse_input(&input);
    nodes.sort_by(|a, b| a.flow_rate.cmp(&b.flow_rate).reverse());
    println!("sorted nodes by flow rate {:?}", nodes);
    let mut start = 0;
    let mut previouses: HashMap<usize, usize> = HashMap::new();
    let mut minutes_remaining = 30;

    let mut iter = nodes.iter();
    while let Some(node) = iter.next() {
        println!("trying {} to {} with {}", Valve::position_to_alpha(start), Valve::position_to_alpha(node.position), minutes_remaining);
        if minutes_remaining <= 0 {
            println!("out of time");
            break;
        }

        if start == node.position {
            println!("loop");
            break;
        }
        if previouses.get(&node.position).is_some() { 
            println!("already went here");
            continue;
        }
        if node.flow_rate == 0 { 
            println!("got to the 0 flow rate mfers)");
            break;
        }

    

        let mut already_seen = HashSet::new();
        previouses.iter().for_each(|(destination, previous)| {
            already_seen.insert(*destination);
            already_seen.insert(*previous);
        });
        let new_previouses = bfs(&parse_input(&input), start, node.position);
        new_previouses.iter().for_each(|(destination, previous)| { 
            minutes_remaining -= if nodes[*previous].flow_rate > 0 { 2 } else { 1 };
            previouses.insert(*destination, *previous); 
        });
        start = node.position;
    }
    0
}

pub fn second(input: &[String]) -> usize {
    0
}

struct State{
    position: usize,
    history: Vec<usize>,
}

fn bfs(edges: &Vec<Valve>, start: usize, end: usize) -> HashMap<usize, usize> {
    let mut stack: VecDeque<State> = VecDeque::new();
    stack.push_back(State{position: start, history: vec!(start)});

    while let Some(State{position, history}) = stack.pop_front() {
        println!("bfs @ {}", Valve::position_to_alpha(position));
        if position == end {
            let mut ret_val = HashMap::new();
            history.windows(2).for_each(|chunks| { ret_val.insert(chunks[1], chunks[0]); } );
            return ret_val;
        }

        for neighbor in edges[position].neighbors.iter() {
            let mut new_stack = history.clone();
            new_stack.push(*neighbor);

            let state = State{
                position: *neighbor,
                history: new_stack
            };
            stack.push_back(state);
        }
    }

    // return path not found
    HashMap::new()
}

// #[derive(PartialEq, Eq, Clone)]
// struct State {
//     cost: usize, 
//     position: usize,
//     already_flowing: HashSet<usize>,
// }

// // The priority queue depends on `Ord`.
// // Explicitly implement the trait so the queue becomes a min-heap
// // instead of a max-heap.
// impl Ord for State {
//     fn cmp(&self, other: &Self) -> Ordering {
//         // Notice that the we flip the ordering on costs.
//         // In case of a tie we compare positions - this step is necessary
//         // to make implementations of `PartialEq` and `Ord` consistent.
//         other
//             .cost
//             .cmp(&self.cost)
//             .then_with(|| self.position.cmp(&other.position))
//             .then_with(|| self.already_flowing.len().cmp(&other.already_flowing.len()))
//     }
// }

// // `PartialOrd` needs to be implemented as well.
// impl PartialOrd for State {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// fn djikstras(start: usize, end: usize, edges: &Vec<Valve>, already_flowing: &HashSet<usize>, max_cost: usize) -> HashMap<usize, usize> {
//     // Djikstra's! https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
//     let mut distances: HashMap<usize, usize> = edges
//         .iter()
//         .map(|valve| (valve.position, usize::MAX))
//         .collect();
//     // distances.insert(start, 0);
//     println!("distances {:?}", distances);

//     let mut previouses: HashMap<usize, usize> = HashMap::new(); // notate the path taken so we can later calculate the cost? maybe

//     let mut unvisited = BinaryHeap::new();
//     unvisited.push(State {
//         cost: 0,
//         position: start,
//         already_flowing: already_flowing.clone()
//     });

//     while let Some(State { cost, position, already_flowing }) = unvisited.pop() {
//         println!("current node {} {}", cost, Valve::position_to_alpha(position));
//         // if we're out of time we're done
//         if cost >= max_cost {
//             println!("minutes remaining 0");
//             return previouses;
//         }

//         if position == end {
//             println!("at target");
//             return previouses;
//         }

//         // // we've gone down a bad path
//         // if cost > *distances.get(&position).unwrap() {
//         //     println!("this is a bad path?");
//         //     continue;
//         // }

//         // for each neighbor, see if we can find a way with a lower cost going through this node
//         for neighbor in edges[position].neighbors.iter() {
//             println!("considering neighbor? {:?}", Valve::position_to_alpha(*neighbor));
//             let neighbor_already_flowing = already_flowing.contains(neighbor);
            
//             // let new_minutes = minutes_remaining - new_minutes;
//             let new_cost = if neighbor_already_flowing { 1 } else { 2 };
//             let new_already_flowing = if neighbor_already_flowing {
//                 already_flowing.clone()
//             } else {
//                 let mut new_one = already_flowing.clone();
//                 new_one.insert(*neighbor);
//                 new_one.clone()
//             }.clone();

//             let next = State {
//                 cost: cost + new_cost,
//                 position: *neighbor,
//                 already_flowing: new_already_flowing,
//             };

//             // if we have a better way to get there from here
//             unvisited.push(next);
//             distances.insert(*neighbor,new_cost);
//             previouses.insert(*neighbor, position);
//         }
//     }

//     // not reachable
//     panic!("OH NO")
// }

#[derive(Default, Clone, Debug)]
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
