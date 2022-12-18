use std::collections::HashMap;

// this seems to be taking forever, there's gotta be a pattern that repeats at some point
pub fn first(input: &[String]) -> usize {
    let jets: &String = input.first().unwrap();
    rock_fall_iterate(jets, 2022)
}

pub fn second(input: &[String]) -> usize {
    let jets: &String = input.first().unwrap();
    rock_fall_iterate(jets, 1_000_000_000_000)
}

fn rock_fall_iterate(jets: &String, max_generation: isize) -> usize {
    let next_command = Command::PushedByJet(0);
    let rock_shapes = RockShapes::new();
    let mut rocks_iter: isize = 0;
    let mut jets_iter = 0;
    let mut ys_state = Grid::new(); // these will be diffs from the max_y

    // memo from (Grid, shape index, jet index) -> (Grid, how much min y changed, jet index)
    let mut memo: HashMap<(Grid, isize, isize), (Grid, isize, isize)> = HashMap::new();
    let mut min_y = 0;
    let mut memo_hits = 0;

    for g in 0..max_generation {
        let memo_key = (ys_state, rocks_iter, jets_iter);
        let memo_value = memo.get(&memo_key);
        let (new_max_ys, min_y_delta, new_jet_index) = memo_value
            .map(|(ys, min_y_delta, jet_delta)| {
                memo_hits += 1;
                (*ys, *min_y_delta, *jet_delta)
            })
            .unwrap_or_else(|| {
                let (new_max_ys, min_y_delta, new_jet_index) =
                    spawn_rock(&ys_state, &rock_shapes.at(rocks_iter), jets_iter, &jets);
                memo.insert(memo_key, (new_max_ys, min_y_delta, new_jet_index));
                (new_max_ys, min_y_delta, new_jet_index)
            });

        min_y = min_y + min_y_delta;
        jets_iter = new_jet_index;
        rocks_iter = (rocks_iter + 1) % rock_shapes.shapes.len() as isize;
        ys_state = new_max_ys;
    }

    println!(
        "Memo effectiveness! {}",
        memo_hits as f32 / max_generation as f32
    );
    (min_y as usize) + ys_state.max()
}

// (Grid, shape index, jet index) -> (Grid, how much min y changed, jet index)
fn spawn_rock(
    max_ys: &Grid,
    shape: &RockShape,
    jet_index: isize,
    jets: &String,
) -> (Grid, isize, isize) {
    let mut new_max_ys = (*max_ys).clone();
    let mut rock_loc: (isize, isize) = (2, (max_ys.max() + 3).try_into().unwrap());
    let mut next_command = Command::PushedByJet(jet_index);
    loop {
        let (new_command, new_rock_loc) = next_command.next(jets, rock_loc);
        let intersect = shape.intersect(new_rock_loc.0, new_rock_loc.1, &new_max_ys);
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

        if should_move {
            rock_loc = new_rock_loc;
        }
        next_command = new_command;
    }

    // calculate new max_ys based on where rock landed
    shape.put_rock(rock_loc.0, rock_loc.1, &mut new_max_ys);

    let new_min_y = new_max_ys.shift_new_min_y();

    (new_max_ys, new_min_y, next_command.index())
}

enum Command {
    PushedByJet(isize),
    FallingDown(isize),
}

impl Command {
    fn next(&self, jets: &String, pair: (isize, isize)) -> (Command, (isize, isize)) {
        match self {
            Command::PushedByJet(index) => {
                let new_pair = match jets.chars().nth(*index as usize) {
                    Some('>') => (pair.0 + 1, pair.1),
                    Some('<') => (pair.0 - 1, pair.1),
                    _ => panic!("unknown jet char"),
                };
                (
                    Command::FallingDown((*index + 1) % jets.len() as isize),
                    new_pair,
                )
            }
            Command::FallingDown(index) => (Command::PushedByJet(*index), (pair.0, pair.1 - 1)),
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
    rock_diffs: Vec<(isize, isize)>,
}

impl RockShape {
    fn intersect(&self, x: isize, y: isize, grid: &Grid) -> bool {
        for (plus_x, plus_y) in self.rock_diffs.iter() {
            let new_x = x + plus_x;
            let new_y = y + plus_y;
            let b1 = new_x > 6 as isize;
            let b2 = new_x < 0;
            let b3 = new_y < 0;
            let b4 = b1 || b2 || b3 || grid.at(new_x, new_y);
            if b1 || b2 || b3 || b4 {
                return true;
            }
        }
        false
    }

    fn put_rock(&self, x: isize, y: isize, max_ys: &mut Grid) {
        for (plus_x, plus_y) in self.rock_diffs.iter() {
            let new_x = (x + plus_x) as isize;
            let new_y = (y + plus_y) as isize;
            max_ys.set(new_x, new_y, true);
        }
    }
}

struct RockShapes {
    shapes: Vec<RockShape>,
}

impl RockShapes {
    fn new() -> RockShapes {
        let shapes = vec![
            // Horizontal Line
            RockShape {
                rock_diffs: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            },
            // Cross
            RockShape {
                rock_diffs: vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            },
            // Backwards L
            RockShape {
                rock_diffs: vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            },
            // Veritcal Line
            RockShape {
                rock_diffs: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            },
            // Square
            RockShape {
                rock_diffs: vec![(0, 0), (1, 0), (1, 1), (0, 1)],
            },
        ];
        RockShapes { shapes }
    }

    fn at(&self, i: isize) -> &RockShape {
        &self.shapes[i as usize % self.shapes.len()]
    }
}

fn print_state(min_y: isize, max_ys: &Grid) {
    for y in (0..=max_ys.max()).rev() {
        print!("{}: | ", (y + (min_y as usize)));
        for x in 0..7 {
            if max_ys.maxes[x][y] {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!("|");
    }

    println!("  +_______+");
    println!();
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
struct Grid {
    maxes: [[bool; 100]; 7],
}

impl Grid {
    fn new() -> Grid {
        Grid {
            maxes: [[false; 100]; 7],
        }
    }

    fn at(&self, x: isize, y: isize) -> bool {
        self.maxes[x as usize][y as usize]
    }

    fn set(&mut self, x: isize, y: isize, to: bool) {
        self.maxes[x as usize][y as usize] = to;
    }

    fn max(&self) -> usize {
        let mut max_y = 0;
        for x in 0..7 {
            let mut col_max = 0;
            for y in (0..100).rev() {
                if self.maxes[x][y] {
                    col_max = y + 1;
                    break;
                }
            }
            if col_max > max_y {
                max_y = col_max;
            }
        }
        max_y
    }

    fn shift_new_min_y(&mut self) -> isize {
        // find the min column max y rock, if it's non 0 we have to adjust everything down by that amount

        let mut min_y = 100;
        for x in 0..7 {
            let mut col_max = 0;
            for y in (0..100).rev() {
                if self.maxes[x][y] {
                    col_max = y;
                    break;
                }
            }
            if col_max < min_y {
                min_y = col_max;
            }
        }

        if min_y != 0 {
            //  need to shift everything down min_y rows
            for x in 0..7 {
                for y in 0..(100 - min_y) {
                    self.maxes[x][y] = self.maxes[x][y + min_y];
                }
                for y in (100 - min_y)..100 {
                    self.maxes[x][y] = false;
                }
            }
        }

        min_y as isize
    }
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
