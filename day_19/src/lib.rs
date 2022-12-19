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
    unvisited.push_back(State{ore: 0, clay: 0, obsidian: 0, geode: 0, ore_delta: 1, clay_delta: 0, obsidian_delta: 0, geode_delta: 0, steps_remaining: 24});
    while let Some(state @ State{ore, clay, obsidian, geode, ore_delta, clay_delta, obsidian_delta, geode_delta, steps_remaining}) = unvisited.pop_front() {
        if seen_states.contains(&state) || (max_geodes > 1 && geode < (max_geodes - 1)) {
            continue;
        }

        visited += 1;
        max_geodes = max_geodes.max(geode);
        // println!("visited & max geodes {} {} {:?} {:?} {}", visited, max_geodes, state.resource_counts, state.resource_deltas, state.steps_remaining);

        seen_states.insert(state.clone());
        if state.steps_remaining == 0 {
            continue;
        }
        
        // all resource increase by deltas
        let new_state = State{
            ore: ore + ore_delta,
            clay: clay + clay_delta,
            obsidian: obsidian + obsidian_delta,
            geode: geode + geode_delta,
            steps_remaining: steps_remaining - 1,
            .. state
        };

        // what can we do each time step? 

        // we could do nothing
        unvisited.push_back(new_state);

        // if we have enough resources we could increase the delta of some resource
        if state.ore >= blueprint.ore_bot_ore_cost {
            let state = State{
                ore: new_state.ore - blueprint.ore_bot_ore_cost,
                ore_delta: state.ore_delta + 1,
                .. new_state
            };
            unvisited.push_back(state);
        }

        if state.ore >= blueprint.clay_bot_ore_cost {
            let state = State{
                ore: new_state.ore - blueprint.clay_bot_ore_cost,
                clay_delta: state.clay_delta + 1,
                .. new_state
            };
            unvisited.push_back(state);
        }

        if state.ore >= blueprint.obsidian_bot_ore_cost && state.clay >= blueprint.obsidian_bot_clay_cost {
            let state = State{
                ore: new_state.ore - blueprint.obsidian_bot_ore_cost,
                clay: new_state.clay - blueprint.obsidian_bot_clay_cost,
                obsidian_delta: state.obsidian_delta + 1,
                .. new_state
            };
            unvisited.push_back(state);
        }

        if state.ore >= blueprint.geode_bot_ore_cost && state.obsidian >= blueprint.geode_bot_obsidian_cost {
            let state = State{
                ore: new_state.ore - blueprint.geode_bot_ore_cost,
                obsidian: new_state.obsidian - blueprint.geode_bot_obsidian_cost,
                geode_delta: state.geode_delta + 1,
                .. new_state
            };
            unvisited.push_back(state);
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

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct State {
    ore: usize,
    clay: usize, 
    obsidian: usize,
    geode: usize,
    ore_delta:usize, 
    clay_delta:usize, 
    obsidian_delta: usize,
    geode_delta: usize,
    steps_remaining: usize
}

// list indexed by resource type to produce an additional delta of that resource, 
// where the resource list is indexed by resource type to the amount of that resource needed
#[derive(Debug, PartialEq, Eq)]
pub struct Blueprint{
    ore_bot_ore_cost: usize, 
    clay_bot_ore_cost: usize, 
    obsidian_bot_ore_cost: usize, 
    obsidian_bot_clay_cost: usize, 
    geode_bot_ore_cost: usize, 
    geode_bot_obsidian_cost: usize,
}

fn parse_input(input : &[String]) -> Vec<Blueprint> {
    input.iter().map(|input| { 
        let mut parts = input.split_whitespace();
        Blueprint{
            ore_bot_ore_cost: parts.nth(6).unwrap().parse().unwrap(), 
            clay_bot_ore_cost: parts.nth(5).unwrap().parse().unwrap(), 
            obsidian_bot_ore_cost: parts.nth(5).unwrap().parse().unwrap(), 
            obsidian_bot_clay_cost: parts.nth(2).unwrap().parse().unwrap(), 
            geode_bot_ore_cost: parts.nth(5).unwrap().parse().unwrap(), 
            geode_bot_obsidian_cost: parts.nth(2).unwrap().parse().unwrap(),
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
            ore_bot_ore_cost: 4, 
            clay_bot_ore_cost: 2, 
            obsidian_bot_ore_cost: 3, 
            obsidian_bot_clay_cost: 14, 
            geode_bot_ore_cost: 2, 
            geode_bot_obsidian_cost: 7,
        };
        let blueprint2 = Blueprint{
            ore_bot_ore_cost: 2, 
            clay_bot_ore_cost: 3, 
            obsidian_bot_ore_cost: 3, 
            obsidian_bot_clay_cost: 8, 
            geode_bot_ore_cost: 3, 
            geode_bot_obsidian_cost: 12,
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
