use std::collections::HashMap;

pub fn first(input: &[String]) -> usize {
    let blueprints = parse_input(&input);
    blueprints.iter().enumerate().fold(0, |accum, (i, next)| {
        let mut memo: HashMap<State, usize> = HashMap::new();
        let value = maximize(State::new(), next, &mut memo);
        println!("value for i {} {}", i, value);
        accum + ((i + 1) * value)
    })
}

pub fn second(input: &[String]) -> usize {
    let blueprints = parse_input(&input);
    blueprints.iter().enumerate().map(|(i, next)| {
        let mut memo: HashMap<State, usize> = HashMap::new();
        // let new_state = State{ ore: 3, clay: 15, obsidian: 0, geode: 0, ore_delta: 2, clay_delta: 6, obsidian_delta: 0, geode_delta: 0, time_remaining: 20 };
        let new_state = State{time_remaining: 32, .. State::new()};
        let value = maximize(new_state, next, &mut memo);
        println!("value for i {} {}", i, value);
        value
    }).reduce(|a, b| a * b).unwrap_or(0)
}

// Top down recursive dynamic programming, with memoization. For each subproblem, I go through the 4 possible bots I want to make
// (regardless of whether I can currently afford it), then move to the subproblem which represent waiting x units of time until I can build it,
// then build it.
// To optimise, I stopped building ore/clay bots if I'm currently producing more than I could ever need
// (producing at least the max cost of the bots).
// qqI also didn't build bots if there was too little time left to yield benefit from them, e.g. building clay bots with fewer than 6 minutes left is pointless.
//
fn maximize(
    state @ State {
        ore,
        clay,
        obsidian,
        geode,
        ore_delta,
        clay_delta,
        obsidian_delta,
        geode_delta,
        time_remaining,
    }: State,
    blueprint: &Blueprint,
    memo: &mut HashMap<State, usize>,
) -> usize {
    if time_remaining == 0 {
        memo.insert(state, geode);
        return geode;
    }

    if let Some(memoized) = memo.get(&state) {
        return *memoized;
    }

    let (make_geode_bot, make_geode_state) = if obsidian_delta > 0 {
        let obsidian_cost = if obsidian >= blueprint.geode_bot_obsidian_cost {
            0
        } else {
            blueprint.geode_bot_obsidian_cost - obsidian
        };
        let min_turns_for_enough_obsidian = obsidian_cost / obsidian_delta;
        let min_turns_for_enough_obsidian = if obsidian_cost % obsidian_delta == 0 {
            min_turns_for_enough_obsidian
        } else {
            min_turns_for_enough_obsidian + 1
        };

        let ore_cost = if ore >= blueprint.geode_bot_ore_cost {
            0
        } else {
            blueprint.geode_bot_ore_cost - ore
        };
        let min_turns_for_enough_ore = ore_cost / ore_delta;
        let min_turns_for_enough_ore = if ore_cost % ore_delta == 0 {
            min_turns_for_enough_ore
        } else {
            min_turns_for_enough_ore + 1
        };

        let turns_to_make = min_turns_for_enough_obsidian.max(min_turns_for_enough_ore) + 1; // +1 to build
        if time_remaining > turns_to_make {
            let new_state = State {
                time_remaining: time_remaining - turns_to_make,
                geode_delta: geode_delta + 1,
                ore: ore + ore_delta * turns_to_make - blueprint.geode_bot_ore_cost,
                obsidian: obsidian + obsidian_delta * turns_to_make
                    - blueprint.geode_bot_obsidian_cost,
                clay: clay + clay_delta * turns_to_make,
                geode: geode + geode_delta * turns_to_make,
                ..state
            };

            // println!("1 From {:?} to {:?}", state, new_state);
            (maximize(new_state, &blueprint, memo), new_state)
        } else {
            (0, State::new())
        }
    } else {
        (0, State::new())
    };

    let (make_obsidian_bot, make_obsidian_state) = if clay_delta > 0
        && obsidian < blueprint.geode_bot_obsidian_cost
        && obsidian_delta < blueprint.geode_bot_obsidian_cost
    {
        let clay_cost = if clay >= blueprint.obsidian_bot_clay_cost {
            0
        } else {
            blueprint.obsidian_bot_clay_cost - clay
        };
        let min_turns_for_enough_clay = clay_cost / clay_delta;
        let min_turns_for_enough_clay = if clay_cost % clay_delta == 0 {
            min_turns_for_enough_clay
        } else {
            min_turns_for_enough_clay + 1
        };

        let ore_cost = if ore >= blueprint.obsidian_bot_ore_cost {
            0
        } else {
            blueprint.obsidian_bot_ore_cost - ore
        };
        let min_turns_for_enough_ore = ore_cost / ore_delta;
        let min_turns_for_enough_ore = if ore_cost % ore_delta == 0 {
            min_turns_for_enough_ore
        } else {
            min_turns_for_enough_ore + 1
        };

        let turns_to_make = min_turns_for_enough_clay.max(min_turns_for_enough_ore) + 1; // +1 to build
        if time_remaining >= turns_to_make {
            // and possibly turns_to_make + blueprint.geode_bot_obsidian_cost
            // println!("*** ", clay, clay_delta, turns_to_make, blueprint.obsidian_bot_clay_cost);
            let new_state = State {
                time_remaining: time_remaining - turns_to_make,
                obsidian_delta: obsidian_delta + 1,
                ore: ore + ore_delta * turns_to_make - blueprint.obsidian_bot_ore_cost,
                obsidian: obsidian + obsidian_delta * turns_to_make,
                clay: clay + clay_delta * turns_to_make - blueprint.obsidian_bot_clay_cost,
                geode: geode + geode_delta * turns_to_make,
                ..state
            };

            // println!("2 From {:?} to {:?} {}", state, new_state, best);
            (maximize(new_state, &blueprint, memo), new_state)
        } else {
            (0, State::new())
        }
    } else {
        (0, State::new())
    };

//     (an obsidian bot) was the best way to go from State { ore: 3, clay: 15, obsidian: 0, geode: 0, ore_delta: 2, clay_delta: 6, obsidian_delta: 0, geode_delta: 0, time_remaining: 20 } to State { ore: 2, clay: 7, obsidian: 0, geode: 0, ore_delta: 2, clay_delta: 6, obsidian_delta: 1, geode_delta: 0, time_remaining: 19 }
// **** geode bot score: 0 to go to State { ore: 0, clay: 0, obsidian: 0, geode: 0, ore_delta: 1, clay_delta: 0, obsidian_delta: 0, geode_delta: 0, time_remaining: 24 }
// **** obsidian bot score: 54 to go to State { ore: 2, clay: 7, obsidian: 0, geode: 0, ore_delta: 2, clay_delta: 6, obsidian_delta: 1, geode_delta: 0, time_remaining: 19 }
// **** clay bot score: 0 to go to State { ore: 0, clay: 0, obsidian: 0, geode: 0, ore_delta: 1, clay_delta: 0, obsidian_delta: 0, geode_delta: 0, time_remaining: 24 }
// **** ore bot score: 50 to go to State { ore: 3, clay: 27, obsidian: 0, geode: 0, ore_delta: 3, clay_delta: 6, obsidian_delta: 0, geode_delta: 0, time_remaining: 18 }

    let (make_clay_bot, make_clay_state) = if clay_delta < blueprint.obsidian_bot_clay_cost
    {
        let cost = if ore >= blueprint.clay_bot_ore_cost {
            0
        } else {
            blueprint.clay_bot_ore_cost - ore
        };
        let turns_to_make = cost / ore_delta + 1;
        let turns_to_make_remainder = cost % ore_delta;
        let turns_to_make = if turns_to_make_remainder != 0 {
            turns_to_make + 1
        } else {
            turns_to_make
        };

        // there's some relation between clay we have, clay delta, and remaining turns
        // such that adding a clay bot now changes how many future obsidian bots we can make
        if time_remaining >= (turns_to_make + blueprint.obsidian_bot_clay_cost) {
            let new_state = State {
                time_remaining: time_remaining - turns_to_make,
                clay_delta: clay_delta + 1,
                ore: ore + ore_delta * turns_to_make - blueprint.clay_bot_ore_cost,
                obsidian: obsidian + obsidian_delta * turns_to_make,
                clay: clay + clay_delta * turns_to_make,
                geode: geode + geode_delta * turns_to_make,
                ..state
            };

            // println!("3 From {:?} to {:?} {}", state, new_state, best);
            (maximize(new_state, &blueprint, memo), new_state)
        } else {
            (0, State::new())
        }
    } else {
        (0, State::new())
    };

    let (make_ore_bot, make_ore_state) =
        if ore < blueprint.max_ore_cost && ore_delta < blueprint.max_ore_cost {
            let cost = if ore >= blueprint.ore_bot_ore_cost {
                0
            } else {
                blueprint.ore_bot_ore_cost - ore
            };
            let turns_to_make = cost / ore_delta + 1;
            let turns_to_make_remainder = cost % ore_delta;
            let turns_to_make = if turns_to_make_remainder != 0 {
                turns_to_make + 1
            } else {
                turns_to_make
            };
            if time_remaining >= turns_to_make {
                let new_state = State {
                    time_remaining: time_remaining - turns_to_make,
                    ore_delta: ore_delta + 1,
                    ore: ore + ore_delta * turns_to_make - blueprint.ore_bot_ore_cost,
                    obsidian: obsidian + obsidian_delta * turns_to_make,
                    clay: clay + clay_delta * turns_to_make,
                    geode: geode + geode_delta * turns_to_make,
                    ..state
                };
                // println!("4 From {:?} to {:?} {}", state, new_state, best);
                (maximize(new_state, &blueprint, memo), new_state)
            } else {
                (0, State::new())
            }
        } else {
            (0, State::new())
        };

    let best = *vec![
        make_geode_bot,
        make_obsidian_bot,
        make_clay_bot,
        make_ore_bot,
    ]
    .iter()
    .max()
    .unwrap();
    let (the_best_thing_to_do, to_state) = if best == make_geode_bot {
        ("a geode bot", make_geode_state)
    } else if best == make_obsidian_bot {
        ("an obsidian bot", make_obsidian_state)
    } else if best == make_clay_bot {
        ("a clay bot", make_clay_state)
    } else if best == make_ore_bot {
        ("an ore bot", make_ore_state)
    } else {
        ("", State::new())
    };

    // FROM HERE the paths diverge - counts obsidian bot and clay bot equally
// (an obsidian bot) was the best way to go from State { ore: 3, clay: 10, obsidian: 0, geode: 0, ore_delta: 2, clay_delta: 5, obsidian_delta: 0, geode_delta: 0, time_remaining: 21 } to State { ore: 4, clay: 6, obsidian: 0, geode: 0, ore_delta: 2, clay_delta: 5, obsidian_delta: 1, geode_delta: 0, time_remaining: 19 }
// 
    println!(
        "({}) was the best way to go from {:?} to {:?}",
        the_best_thing_to_do, state, to_state
    );
    println!(
        "**** geode bot score: {} to go to {:?}",
        make_geode_bot, make_geode_state
    );
    println!(
        "**** obsidian bot score: {} to go to {:?}",
        make_obsidian_bot, make_obsidian_state
    );
    println!(
        "**** clay bot score: {} to go to {:?}",
        make_clay_bot, make_clay_state
    );
    println!(
        "**** ore bot score: {} to go to {:?}",
        make_ore_bot, make_ore_state
    );
    println!();
    if best == 0 && geode_delta > 0 {
        // println!("Visited a waiting for geodes node {:?}", state);
        println!("best thing to do was wait it out");
        geode + geode_delta * time_remaining
    } else {
        // println!("{}", the_best_thing_to_do);
        memo.insert(state, best);
        best
    }

    // println!("Visited {}", best);
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct State {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    ore_delta: usize,
    clay_delta: usize,
    obsidian_delta: usize,
    geode_delta: usize,
    time_remaining: usize,
}

impl State {
    fn new() -> State {
        State {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_delta: 1,
            clay_delta: 0,
            obsidian_delta: 0,
            geode_delta: 0,
            time_remaining: 24,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Blueprint {
    ore_bot_ore_cost: usize,
    clay_bot_ore_cost: usize,
    obsidian_bot_ore_cost: usize,
    obsidian_bot_clay_cost: usize,
    geode_bot_ore_cost: usize,
    geode_bot_obsidian_cost: usize,
    max_ore_cost: usize,
    min_ore_time: usize,
}

fn parse_input(input: &[String]) -> Vec<Blueprint> {
    input
        .iter()
        .map(|input| {
            let mut parts = input.split_whitespace();
            let mut result = Blueprint {
                ore_bot_ore_cost: parts.nth(6).unwrap().parse().unwrap(),
                clay_bot_ore_cost: parts.nth(5).unwrap().parse().unwrap(),
                obsidian_bot_ore_cost: parts.nth(5).unwrap().parse().unwrap(),
                obsidian_bot_clay_cost: parts.nth(2).unwrap().parse().unwrap(),
                geode_bot_ore_cost: parts.nth(5).unwrap().parse().unwrap(),
                geode_bot_obsidian_cost: parts.nth(2).unwrap().parse().unwrap(),
                max_ore_cost: 0,
                min_ore_time: 0,
            };
            result.max_ore_cost = result
                .ore_bot_ore_cost
                .max(result.clay_bot_ore_cost)
                .max(result.obsidian_bot_ore_cost)
                .max(result.geode_bot_ore_cost);
            result.min_ore_time = result
                .ore_bot_ore_cost
                .min(result.clay_bot_ore_cost)
                .min(result.obsidian_bot_ore_cost)
                .min(result.geode_bot_ore_cost);
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
            ore_bot_ore_cost: 4,
            clay_bot_ore_cost: 2,
            obsidian_bot_ore_cost: 3,
            obsidian_bot_clay_cost: 14,
            geode_bot_ore_cost: 2,
            geode_bot_obsidian_cost: 7,
            max_ore_cost: 4,
            min_ore_time: 2,
        };
        let blueprint2 = Blueprint {
            ore_bot_ore_cost: 2,
            clay_bot_ore_cost: 3,
            obsidian_bot_ore_cost: 3,
            obsidian_bot_clay_cost: 8,
            geode_bot_ore_cost: 3,
            geode_bot_obsidian_cost: 12,
            max_ore_cost: 3,
            min_ore_time: 2,
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

    // #[test]
    // fn first_test() {
    //     let input = example();
    //     let result = first(&input);
    //     assert_eq!(result, 33);
    // }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 56 * 62);
    }
}
