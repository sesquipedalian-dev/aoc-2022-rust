use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::cmp::Ordering;

use std::collections::BinaryHeap;

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
    // let mut visited = 0;
    let mut max_geodes = 0;
    let mut seen_states = HashSet::new();

    // I don't think this is really going depth-first
    let mut visited = 0;
    // let mut unvisited: BinaryHeap<State> = BinaryHeap::new();
    let mut unvisited: Vec<State> = Vec::new();
    unvisited.push(State{ore: 0, clay: 0, obsidian: 0, geode: 0, ore_delta: 1, clay_delta: 0, obsidian_delta: 0, geode_delta: 0, steps_remaining: 24, ore_skipped: false, clay_skipped: false, obsidian_skipped: false});
    while let Some(state @ State{ore, clay, obsidian, geode, ore_delta, clay_delta, obsidian_delta, geode_delta, steps_remaining, ore_skipped, clay_skipped, obsidian_skipped}) = unvisited.pop() {
        if seen_states.contains(&state) {
            continue;
        }

        visited += 1;
        max_geodes = max_geodes.max(geode);
        println!("visited & max geodes {} {} {:?}", visited, max_geodes, state);

        seen_states.insert(state.clone());
        if state.steps_remaining == 0 {
            continue;
        }
        
        // in the next state, all resource increase by deltas
        let mut new_state = State{
            ore: ore + ore_delta,
            clay: clay + clay_delta,
            obsidian: obsidian + obsidian_delta,
            geode: geode + geode_delta,
            steps_remaining: steps_remaining - 1,
            .. state
        };

        // what can we do each time step? 
        // according to an answer on the solutions thread: https://www.reddit.com/r/adventofcode/comments/zpihwi/2022_day_19_solutions/j0u4fwu/
        // If you can a build a geode robot, you should, so no need to explore other possibilities in that case. (I'm actually not completely sure about this one, but it works with the test case and my input, so...)
        if ore >= blueprint.geode_bot_ore_cost && obsidian >= blueprint.geode_bot_obsidian_cost {
            let state = State{
                ore: new_state.ore - blueprint.geode_bot_ore_cost,
                obsidian: new_state.obsidian - blueprint.geode_bot_obsidian_cost,
                geode_delta: state.geode_delta + 1,
                ore_skipped: false,
                clay_skipped: false,
                obsidian_skipped: false,
                .. new_state
            };
            unvisited.push(state);
            continue
        }

        // You should not build a robot if you have already the number required to produce the amount of resource needed to build any robot in a turn
        // If you can build all types of robots, you must build one
        // And finally, if you skip building a robot when you can, you should not build it until you have build another one.

        // if we have enough resources we could increase the delta of some resource
        let can_build_ore = !ore_skipped && ore_delta < blueprint.max_ore_cost && ore >= blueprint.ore_bot_ore_cost;
        if can_build_ore {
            let state = State{
                ore: new_state.ore - blueprint.ore_bot_ore_cost,
                ore_delta: state.ore_delta + 1,
                ore_skipped: false,
                clay_skipped: false,
                obsidian_skipped: false,
                .. new_state
            };
            unvisited.push(state);
        }

        let can_build_clay = !clay_skipped && clay_delta < blueprint.obsidian_bot_clay_cost && ore >= blueprint.clay_bot_ore_cost;
        if can_build_clay {
            let state = State{
                ore: new_state.ore - blueprint.clay_bot_ore_cost,
                clay_delta: state.clay_delta + 1,
                ore_skipped: false,
                clay_skipped: false,
                obsidian_skipped: false,
                .. new_state
            };
            unvisited.push(state);
        }

        let can_build_obsidian = !obsidian_skipped && obsidian_delta < blueprint.geode_bot_obsidian_cost && ore >= blueprint.obsidian_bot_ore_cost && clay >= blueprint.obsidian_bot_clay_cost;
        if can_build_obsidian {
            let state = State{
                ore: new_state.ore - blueprint.obsidian_bot_ore_cost,
                clay: new_state.clay - blueprint.obsidian_bot_clay_cost,
                obsidian_delta: state.obsidian_delta + 1,
                ore_skipped: false,
                clay_skipped: false,
                obsidian_skipped: false,
                .. new_state
            };
            unvisited.push(state);
        }

        // we could do nothing
        if !can_build_ore || !can_build_clay ||!can_build_obsidian {
            new_state.ore_skipped = can_build_ore;
            new_state.clay_skipped = can_build_clay;
            new_state.obsidian_skipped = can_build_obsidian;
            unvisited.push(new_state);
        }
    }
    
    println!("visited & max geodes {} {}", visited, max_geodes);
    max_geodes
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
    steps_remaining: usize,
    ore_skipped: bool,
    clay_skipped: bool,
    obsidian_skipped: bool,
}


// The priority queue depends on `Ord`.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.geode.cmp(&self.geode)
            .then_with(|| self.geode_delta.cmp(&other.geode_delta))
            .then_with(|| self.obsidian.cmp(&other.obsidian))
            .then_with(|| self.obsidian_delta.cmp(&other.obsidian_delta))
            .then_with(|| self.clay.cmp(&other.clay))
            .then_with(|| self.clay_delta.cmp(&other.clay_delta))
            .then_with(|| self.ore.cmp(&other.ore))
            .then_with(|| self.ore_delta.cmp(&other.ore_delta))
            .then_with(|| self.steps_remaining.cmp(&other.steps_remaining))
            .reverse()
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
    max_ore_cost: usize
}

fn parse_input(input : &[String]) -> Vec<Blueprint> {
    input.iter().map(|input| { 
        let mut parts = input.split_whitespace();
        let mut result = Blueprint{
            ore_bot_ore_cost: parts.nth(6).unwrap().parse().unwrap(), 
            clay_bot_ore_cost: parts.nth(5).unwrap().parse().unwrap(), 
            obsidian_bot_ore_cost: parts.nth(5).unwrap().parse().unwrap(), 
            obsidian_bot_clay_cost: parts.nth(2).unwrap().parse().unwrap(), 
            geode_bot_ore_cost: parts.nth(5).unwrap().parse().unwrap(), 
            geode_bot_obsidian_cost: parts.nth(2).unwrap().parse().unwrap(),
            max_ore_cost: 0,
        };
        result.max_ore_cost = result.ore_bot_ore_cost.max(result.clay_bot_ore_cost).max(result.obsidian_bot_ore_cost).max(result.geode_bot_ore_cost);
        result
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
            max_ore_cost: 4,
        };
        let blueprint2 = Blueprint{
            ore_bot_ore_cost: 2, 
            clay_bot_ore_cost: 3, 
            obsidian_bot_ore_cost: 3, 
            obsidian_bot_clay_cost: 8, 
            geode_bot_ore_cost: 3, 
            geode_bot_obsidian_cost: 12,
            max_ore_cost:  3,
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
