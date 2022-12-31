use std::collections::HashMap;

pub fn first(input: &[String]) -> usize {
    let blueprints = parse_input(&input);
    blueprints.iter().enumerate().fold(0, |accum, (i, next)| {
        let value = maximize(next, 24);
        println!("value for i {} {}", i, value);
        accum + ((i + 1) * value)
    })
}

pub fn second(input: &[String]) -> u128 {
    let blueprints = parse_input(&input);
    blueprints.iter().enumerate().take(3).map(|(i, next)| {
        let value = maximize(next, 32);
        println!("value for i {} {}", i, value);
        value as u128
    }).reduce(|a, b| a * b).unwrap_or(0)
}

// not really factorial?
fn factorial(n: usize) -> usize { 
    if n == 0 { 
        return 0
    }
    if n == 1 {
        return 1
    }

    n + factorial(n-1)
}
fn maximize(blueprint: &Blueprint, max_time: usize) -> usize { 
    // borrowing from these heuristics: 
    // https://www.reddit.com/r/adventofcode/comments/zpihwi/2022_day_19_solutions/j1vj08v/
    // and their pastebin: https://pastebin.com/5dBc0hRD
    // Trimmed the search space by stopping all robot production at the max number of resources needed to build one bot. 
    // Also trimmed paths which had a theoretical production less than the current max. 
    // The theoretical production was the current amount of geodes, 
    // plus all that could be produced with the current number of geode bots, 
    // and the possible geodes that could be produced if a geode bot was made for every remaining minute. 
    // The comparing the max possible geode production drastically cut run times down. From 25s to 8s for part 1, and allowed part 2 to run in 27 seconds.
    // My algorithm decide which bot to make next from all possibilities, and just waited the amount of time to make that bot for each possible bot.
    //

    let mut queue: Vec<State> = vec!(State::new(max_time));
    let mut current_max = 0; // keep track of the best branch we've seen

    while let Some(State{resources, deltas, time_remaining}) = queue.pop() {
        // println!("Visiting state {} {:?} {:?} {:?}", current_max, resources, deltas, time_remaining);
        // Visiting state 55 ResourceVec([6, 63, 9, 32]) ResourceVec([2, 7, 5, 7]) 3
        // The theoretical production was the current amount of geodes, 
        // plus all that could be produced with the current number of geode bots, 
        // and the possible geodes that could be produced if a geode bot was made for every remaining minute. 
        let res_part = resources.at(&ResourceType::Geode); // 32
        let delts_part = (deltas.at(&ResourceType::Geode) * time_remaining); // 7 * 3 = 21   53
        let additional_bots_part = factorial(time_remaining - 1);
        let theoretical_max_geode = res_part + delts_part + additional_bots_part;
            // + ((time_remaining - 1) * (time_remaining / 2))  // 2 * 1 = 2 = 55
            // ;
        if theoretical_max_geode <= current_max {
            // println!("Bailling, not better than current max {} {} {} {} {} {} {}", theoretical_max_geode, current_max, time_remaining - 1, time_remaining / 2,
                // res_part, delts_part, additional_bots_part
            // );
            continue;
        }

        // try to build each bot, preferring bots in the resource type order 
        // (e.g. possible GeodeBot construction goes at end of the queue so it gets popped next round)
        for rt in ResourceType::all().iter() {
            // no need to build a bot that already is producing the max resources needed to produce a new bot
            match *rt {
                ResourceType::Geode => (),
                _ => if deltas.at(rt) >= blueprint.maxes.at(rt) {
                    continue;
                }
            };

            let costs = &blueprint.costs[usize::from(*rt)];
            let mut time_to_build = 0usize;
            for cost_type in ResourceType::all().iter() {
                if costs.at(cost_type) > resources.at(cost_type) {
                    if deltas.at(cost_type) == 0 { 
                        time_to_build = 1000;
                        break;
                    }
                    let needed = costs.at(cost_type) - resources.at(cost_type);
                    let this_time = if needed % deltas.at(cost_type) == 0 { 
                        needed / deltas.at(cost_type)
                    } else {
                        (needed / deltas.at(cost_type)) + 1
                    };
                    time_to_build = time_to_build.max(this_time);
                }
            }
            time_to_build += 1;

            if time_to_build >= time_remaining {
                // let it ride 
                let this_max = resources.at(&ResourceType::Geode) + (deltas.at(&ResourceType::Geode) * time_remaining );
                // PUshing new state ResourceVec([5, 63, 11, 47]) ResourceVec([2, 7, 6, 8]) 1
                if this_max > current_max { 
                    // println!("Setting new max! {} {}", this_max, current_max);
                    current_max = this_max;
                } else {
                    // println!(" not better than previous max {} {} {:?}", this_max, current_max, rt);
                }
                // current_max = current_max.max(this_max);
            } else { 
                let mut new_resources = resources.clone();
                for cost_type in ResourceType::all().iter() {
                    new_resources.add(cost_type, deltas.at(cost_type) * time_to_build);
                    new_resources.sub(cost_type, costs.at(cost_type));
                }
                let mut new_deltas = deltas.clone();
                new_deltas.add(rt, 1);
                // println!("PUshing new state {:?} {:?} {:?}", new_resources, new_deltas, time_remaining - time_to_build);
                queue.push(State { resources: new_resources, deltas: new_deltas, time_remaining: time_remaining - time_to_build });
            }
        }
    }

    current_max
}


#[derive(Debug, Clone, Copy)]
enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode
}

impl ResourceType {
    fn all() -> Vec<ResourceType> { 
        vec!(ResourceType::Ore, ResourceType::Clay, ResourceType:: Obsidian, ResourceType:: Geode)
    }
}
impl From<ResourceType> for usize {
    fn from(value: ResourceType) -> usize {
        match value { 
            ResourceType::Ore => 0,
            ResourceType::Clay => 1,
            ResourceType::Obsidian => 2,
            ResourceType::Geode => 3,
            _ => panic!("unknown resource type"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct ResourceVec(Vec<usize>);
impl ResourceVec {
    fn at(&self, index: &ResourceType) -> usize {
        let Self(v) = self;
        v[usize::from(*index)]
    }

    fn add(&mut self, index: &ResourceType, amount: usize) {
        let Self(v) = self;
        v[usize::from(*index)] += amount;
    }

    fn sub(&mut self, index: &ResourceType, amount: usize) {
        let Self(v) = self;
        v[usize::from(*index)] -= amount;
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct State {
    resources: ResourceVec,
    deltas: ResourceVec,
    time_remaining: usize,
}

impl State {
    fn new(time_remaining: usize) -> State {
        let resources = ResourceVec(vec![0; 4]);
        let mut deltas_vec = vec![0; 3];
        deltas_vec.insert(ResourceType::Ore.into(), 1);
        let deltas = ResourceVec(deltas_vec);

        State {resources, deltas, time_remaining}
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Blueprint {
    costs: Vec<ResourceVec>,
    maxes: ResourceVec,
}

fn parse_input(input: &[String]) -> Vec<Blueprint> {
    input
        .iter()
        .map(|input| {
            let mut parts = input.split_whitespace();
            let mut result = Blueprint {
                costs: vec!(
                    ResourceVec(vec!(parts.nth(6).unwrap().parse().unwrap(), 0, 0, 0)),
                    ResourceVec(vec!(parts.nth(5).unwrap().parse().unwrap(), 0, 0,0)),
                    ResourceVec(vec!(parts.nth(5).unwrap().parse().unwrap(), parts.nth(2).unwrap().parse().unwrap(),0,0)),
                    ResourceVec(vec!(parts.nth(5).unwrap().parse().unwrap(),0, parts.nth(2).unwrap().parse().unwrap(),0))
                ),
                maxes: ResourceVec(vec!()),
            };
            let mut max_v = vec!();
            for rt in ResourceType::all().iter() {
                max_v.push(result.costs.iter().map(|v| v.at(rt)).max().unwrap_or(0));
            }
            result.maxes = ResourceVec(max_v);
            result
        })
        .collect()
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
        let blueprint1 = Blueprint {
            costs: vec!(
                ResourceVec(vec!(4, 0, 0, 0)),
                ResourceVec(vec!(2, 0, 0, 0)),
                ResourceVec(vec!(3, 14, 0, 0)), 
                ResourceVec(vec!(2, 0, 7, 0)),
            ),
            maxes: ResourceVec(vec!(
                4, 14, 7, 0
            )),
        };
        let blueprint2 = Blueprint {
            costs: vec!(
                ResourceVec(vec!(2, 0, 0, 0)), 
                ResourceVec(vec!(3, 0, 0, 0)),
                ResourceVec(vec!(3, 8, 0, 0)),
                ResourceVec(vec!(3, 0, 12, 0)),
            ),
            maxes: ResourceVec(vec!(
                3, 8, 12, 0
            )),
        };
        vec![blueprint1, blueprint2]
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

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 56 * 62);
    }
}
