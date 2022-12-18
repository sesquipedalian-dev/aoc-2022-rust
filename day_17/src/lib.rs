use std::collections::HashMap;

// this seems to be taking forever, there's gotta be a pattern that repeats at some point
pub fn first(input: &[String]) -> isize {
    const MAX_GENERATION: isize = 2022;
    // const MIN_X: isize = 0;
    // const MAX_X : isize = MIN_X + 6;
    // const MIN_Y: isize = -1;

    // let mut grid = [[false; MAX_GENERATION * 4]; 7]; // 7 x (num of generations * tallest rock), array that should be enough

    let jets: &String = input.first().unwrap();
    let mut next_command = Command::PushedByJet(0);
    let mut max_y = 0 as isize;
    let mut rock_shapes = RockShapes::new();
    let mut rocks_iter: isize = 0;
    let mut jets_iter = 0;
    // let mut generation = 0;

     // TODO ok for second, what we need to look at is the next y in each column
    // that will determine how each shape falls
    // so if we memoize on nexy y in each colum & the shape that's falling, we can 
    // determine how the shape of the max ys will change
    // that should give us the repeat pattern to do 1_000_000_000_000 iterations

    let mut ys_state = MaxYs{maxes: [0;7] }; // these will be diffs from the max_y
    // memo from (MaxYs, shape index, jet index) -> (MaxYs, how much min y changed, jet index)
    let mut memo: HashMap<(String, isize, isize), (String, isize, isize)> = HashMap::new();
    let mut min_y = 0;

    for g in 0..MAX_GENERATION {
        println!("Next generation {} {:?} {}", g, ys_state, min_y);
        print_state(min_y, &ys_state);

        let memo_key = (ys_state.key(), rocks_iter, jets_iter);
        let memo_value = memo.get(&memo_key);
        let (new_max_ys, min_y_delta, new_jet_index) = memo_value.map(|(ys_key, min_y_delta, jet_delta)|{
            (MaxYs::from(ys_key), *min_y_delta, *jet_delta)
        }).unwrap_or_else(|| {
            let (new_max_ys, min_y_delta, new_jet_index) = spawn_rock(
                &ys_state,
                &rock_shapes.at(rocks_iter),
                jets_iter, 
                &jets,
            );
            memo.insert(memo_key, (new_max_ys.key(), min_y_delta, new_jet_index));
            (new_max_ys, min_y_delta, new_jet_index)
        });

        min_y = min_y + min_y_delta;
        jets_iter = new_jet_index;
        rocks_iter = rocks_iter + 1;
        ys_state = new_max_ys;
    }

    min_y + *ys_state.maxes.iter().max().unwrap_or(&0)
}

// (MaxYs, shape index, jet index) -> (MaxYs, how much min y changed, jet index)
fn spawn_rock(max_ys: &MaxYs, shape: &RockShape, jet_index: isize, jets: &String) -> (MaxYs, isize, isize) {
    // fill in the grid based on the max_ys
    let mut grid: Grid = [[false; 100_000]; 7];
    for (x, max_y) in max_ys.maxes.iter().enumerate() {
        if *max_y < 1 {
            continue;
        }
        grid[x][*max_y as usize - 1] = true;
    }

    // loop while the rock is not at rest
    let mut rock_loc = (2, max_ys.max() + 3);
    let mut next_command = Command::PushedByJet(jet_index);
    loop {
        // move the rock
        let (new_command, new_rock_loc) = next_command.next(jets, rock_loc);
        let intersect = shape.intersect(new_rock_loc.0, new_rock_loc.1, &grid);
        let should_move = match next_command {
            Command::PushedByJet(_) => !intersect,
            Command::FallingDown(_) => {
                if intersect {
                    break;
                } else {
                    true
                }
            }
        };
        // println!("should move? {}", should_move);
        if should_move {
                // println!("move from {:?} to {:?}", rock_loc, new_rock_loc);
            rock_loc = new_rock_loc;
        }
        next_command = new_command;
    }

    // calculate new max_ys based on where rock landed
    let mut new_max_ys = shape.new_max_y(rock_loc.0, rock_loc.1, &grid, &max_ys);
    println!("New max ys {:?}", new_max_ys);

    // find the min max y, if it's non 0 we have to adjust everything down by that amount
    let new_min_y = *new_max_ys.maxes.iter().min().unwrap_or(&0);
    if new_min_y > 0 {
        println!("Adjusting min_y {}", new_min_y);
        for i in 0..7 {
            new_max_ys.maxes[i] = new_max_ys.maxes[i] - new_min_y;
        }
    }

    (new_max_ys, new_min_y, next_command.index())
}
type Grid = [[bool; 100_000]; 7];

enum Command {
    PushedByJet(isize),
    FallingDown(isize),
}

impl Command {
    fn next(&self, jets: &String, pair: (isize, isize)) -> (Command, (isize, isize)) {
        match self {
            Command::PushedByJet(index) => {
                // println!("Pushed by Jet! {:?}", jets.chars().nth(*index));
                let new_pair = match jets.chars().nth(*index as usize) {
                    Some('>') => (pair.0 + 1, pair.1),
                    Some('<') => (pair.0 - 1, pair.1),
                    _ => panic!("unknown jet char"),
                };
                (Command::FallingDown((*index + 1) % jets.len() as isize), new_pair)
            }
            Command::FallingDown(index) => {
                // println!("Fall down");
                (Command::PushedByJet(*index), (pair.0, pair.1 - 1))
            }
        }
    }

    fn index(&self) -> isize {
        match self {
            Command::PushedByJet(index) => *index,
            Command::FallingDown(index) => *index,
        }
    }
}

struct RockShape {
    rock_diffs: Vec<(isize, isize)>
}

impl RockShape {
    fn intersect(&self, x: isize, y: isize, grid: &Grid) -> bool {
        for (plus_x, plus_y) in self.rock_diffs.iter() {
            let new_x = x + plus_x;
            let new_y = y + plus_y;
            let b1 = new_x >= grid.len() as isize;
            let b2 = new_x < 0;
            let b3 = new_y < 0;
            let b4 = b1 || b2 || b3 || grid[new_x as usize][new_y as usize];
            if b1 || b2 || b3 || b4 {
                // println!("{:?} interesecttion because {} || {} || {} || {}", (new_x, new_y), b1, b2, b3, b4);
                return true;
            }
        }
        false
    }

    fn new_max_y(&self, x: isize, y: isize, grid: &Grid, max_ys: &MaxYs) -> MaxYs {
        let mut new_max_ys = MaxYs{maxes: max_ys.maxes.clone()};
        for (plus_x, plus_y) in self.rock_diffs.iter() {
            let new_x = (x + plus_x) as isize;
            let new_y = (y + plus_y) as isize;
            new_max_ys.maxes[new_x as usize] = (new_y + 1).max(max_ys.maxes[new_x as usize]);
        }
        new_max_ys
    }

}

struct RockShapes {
    shapes: Vec<RockShape>
}

impl RockShapes {
    fn new() -> RockShapes {
        let shapes = vec![
            // Horizontal Line
            RockShape{ rock_diffs: vec![(0, 0), (1, 0), (2, 0), (3, 0)]},
            // Cross
            RockShape{ rock_diffs: vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]},
            // Backwards L
            RockShape{ rock_diffs: vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]},
            // Veritcal Line
            RockShape{ rock_diffs: vec![(0, 0), (0, 1), (0, 2), (0, 3)]},
            // Square
            RockShape{ rock_diffs: vec![(0, 0), (1, 0), (1, 1), (0, 1)]},
        ];
        RockShapes { shapes }
    }

    fn at(&self, i: isize) -> &RockShape {
        &self.shapes[i as usize % self.shapes.len()]
    }

    
    // fn new_max_y(&self, cur_max_y: isize, cur_loc: (isize, isize)) -> isize {
    //     let mut max_y = cur_max_y;
    //     for (_, plus_y) in self.shapes[self.iter].iter() {
    //         // let new_x = cur_loc.0 + plus_x;
    //         let new_y = cur_loc.1 + plus_y;
    //         max_y = max_y.max(new_y);
    //     }
    //     max_y
    // }
}

fn print_state(min_y: isize, max_ys: &MaxYs) {
    let max_y = *max_ys.maxes.iter().max().unwrap_or(&0);
    for y in (0..=(max_y)).rev() {
        let y_label = min_y + y;
        print!("{}: |", y_label);
        for x in 0..=6 {
            if (max_ys.maxes[x as usize] - 1) == y {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("{}: +_______+", min_y);
    println!();
}

#[derive(Debug)]
struct MaxYs {
    maxes: [isize; 7]
}

impl MaxYs {
    fn key(&self) -> String {
        let mut key = String::new();
        for max in self.maxes.iter() {
            key.push_str(max.to_string().as_str());
            key.push_str(",");
        }
        key
    }

    fn from(key: &String) -> MaxYs {
        let mut new_max_ys = MaxYs{maxes: [0;7]};
        for (i, max_str) in key.split(",").enumerate() {
            new_max_ys.maxes[i] = max_str.parse().unwrap();
        }
        new_max_ys
    }

    fn max(&self) -> isize {
        *self.maxes.iter().max().unwrap_or(&0)
    }
}

pub fn second(input: &[String]) -> isize {
    // TODO ok for second, what we need to look at is the next y in each column
    // that will determine how each shape falls
    // so if we memoize on nexy y in each colum & the shape that's falling, we can 
    // determine how the shape of the max ys will change
    // that should give us the repeat pattern to do 1_000_000_000_000 iterations

    let ys_state = [0;7]; // these will be diffs from the max_y
    // memo from (MaxYs, shape index) -> MaxYs
    let memo: HashMap<(String, isize), String> = HashMap::new();



    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn first_test() {
        unsafe { backtrace_on_stack_overflow::enable() };
        let input = example();
        let result = first(&input);
        assert_eq!(result, 3068);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 0);
    }
}
