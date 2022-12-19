use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn first(input: &[String]) -> usize {
    // so looking at someone else's solution
    // - it's always worth building the lowest 'level' robot that we can
    // - if a path has fewer geodes than the best seen geode path - 1, then we can prune that branch
    // - memoize on seen states
    //

    let blueprints = parse_input(&input);
    blueprints.iter().enumerate().fold(0, |accum, (i, next)| {
        accum + ((i + 1) * max_geodes_for_blueprint(next))
    })
}


pub fn second(input: &[String]) -> usize {
    0
}

fn max_geodes_for_blueprint(blueprint: &Blueprint) -> usize {
    let mut visited = 0;
    let mut max_geodes = 0;
    let mut seen_states = HashSet::new();

    let mut unvisited: VecDeque<State> = VecDeque::new();
    unvisited.push_back(State{resource_counts: vec![0; 4], resource_deltas: vec!(1, 0, 0, 0), steps_remaining: 24});
    while let Some(state) = unvisited.pop_front() {
        if seen_states.contains(&state) || (max_geodes > 1 && state.resource_counts[ResourceType::Geode.i()] < (max_geodes - 1)) {
            continue;
        }

        visited += 1;
        max_geodes = max_geodes.max(state.resource_counts[ResourceType::Geode.i()]);
        // println!("visited & max geodes {} {} {:?} {:?} {}", visited, max_geodes, state.resource_counts, state.resource_deltas, state.steps_remaining);

        seen_states.insert(state.clone());
        if state.steps_remaining == 0 {
            continue;
        }
        
        // all resource increase by deltas
        let new_resources: Vec<usize> = ResourceType::all().iter().map(|resource| {
            state.resource_counts[resource.i()] + state.resource_deltas[resource.i()]
        }).collect();
        
        let new_steps_remaining = state.steps_remaining - 1;

        // what can we do each time step? 
        // we could do nothing
        unvisited.push_back(State{resource_counts: new_resources.clone(), resource_deltas: state.resource_deltas.clone(), steps_remaining: new_steps_remaining});

        // if we have enough resources we could increase the delta of some resource
        for resource in ResourceType::all().iter() {
            if blueprint.need_for(resource).iter().enumerate().fold(true, |accum, (i, needed)| {
                accum && (state.resource_counts[i] >= *needed)
            }) {
                let mut new_resources = new_resources.clone();
                blueprint.need_for(resource).iter().enumerate().for_each(|(i, amount)| {
                    new_resources[i] -= amount;
                });
                let mut new_resource_deltas = state.resource_deltas.clone();
                new_resource_deltas[resource.i()] += 1;
                unvisited.push_back(State{resource_counts: new_resources, resource_deltas: new_resource_deltas, steps_remaining: new_steps_remaining});
            }
        }
    }
    
    println!("visited & max geodes {} {}", visited, max_geodes);
    max_geodes
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode
}

impl ResourceType {
    fn i(&self) -> usize {
        match self {
            ResourceType::Ore => 0,
            ResourceType::Clay => 1, 
            ResourceType::Obsidian => 2, 
            ResourceType::Geode => 3,
        }
    }

    fn all() -> Vec<ResourceType> {
        vec!(ResourceType::Ore, ResourceType::Clay, ResourceType::Obsidian, ResourceType::Geode)
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct State {
    resource_counts: Vec<usize>,
    resource_deltas: Vec<usize>,
    // ore: usize,
    // clay: usize, 
    // obsidian: usize,
    // geode: usize,
    // ore_delta:usize, 
    // clay_delta:usize, 
    // obsidian_robot: usize,
    // geode_robot: usize,
    steps_remaining: usize
}

// list indexed by resource type to produce an additional delta of that resource, 
// where the resource list is indexed by resource type to the amount of that resource needed
#[derive(Debug, PartialEq, Eq)]
pub struct Blueprint{
    pub resource_needs: Vec<Vec<usize>>
}

impl Blueprint {
    fn need_for(&self, resource: &ResourceType) -> &Vec<usize> {
        &self.resource_needs[resource.i()]
    }
}

fn parse_input(input : &[String]) -> Vec<Blueprint> {
    input.iter().map(|input| { 
        let mut parts = input.split_whitespace();
        Blueprint{
            resource_needs: vec!(
                vec!(parts.nth(6).unwrap().parse().unwrap(), 0, 0 ,0), 
                vec!(parts.nth(5).unwrap().parse().unwrap(), 0, 0, 0),
                vec!(parts.nth(5).unwrap().parse().unwrap(), parts.nth(2).unwrap().parse().unwrap(), 0, 0),
                vec!(parts.nth(5).unwrap().parse().unwrap(), 0, parts.nth(2).unwrap().parse().unwrap(), 0),
            )
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
            "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.",
            "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    fn example_parsed() -> Vec<Blueprint> {    
        let blueprint1 = Blueprint{
            resource_needs: vec!(
                vec!(4, 0, 0 ,0), 
                vec!(2, 0, 0, 0),
                vec!(3, 14, 0, 0),
                vec!(2, 0, 7, 0),
            )
        };
        let blueprint2 = Blueprint{
            resource_needs: vec!(
                vec!(2, 0, 0 ,0), 
                vec!(3, 0, 0, 0),
                vec!(3, 8, 0, 0),
                vec!(3, 0, 12, 0),
            )
        };
        vec!(blueprint1, blueprint2)
    }

    #[test]
    fn test_parse_input() {
        let input = example(); 
        let parsed = parse_input(&input); 
        assert_eq!(parsed[0], example_parsed()[0]);
        assert_eq!(parsed[1], example_parsed()[1]);
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, 33);

    }

    // #[test]
    // fn second_test() {
    //     let input = example();
    //     let result = second(&input);
    //     assert_eq!(result, 0);
    // }
}
