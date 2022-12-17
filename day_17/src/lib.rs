// this seems to be taking forever, there's gotta be a pattern that repeats at some point
pub fn first(input: &[String]) -> isize {
    const MAX_GENERATION: usize = 2022;
    // const MIN_X: usize = 0;
    // const MAX_X : usize = MIN_X + 6;
    const MIN_Y: isize = -1;

    let mut grid = [[false; MAX_GENERATION * 4]; 7]; // 7 x (num of generations * tallest rock), array that should be enough

    let jets: &String = input.first().unwrap();
    let mut next_command = Command::PushedByJet(0);
    let mut max_y = MIN_Y as isize;
    let mut rock_shapes = RockShapes::new();
    // let mut generation = 0;

    for _ in 0..MAX_GENERATION {
        // next rock appears at 2, max_y + 3
        let mut rock_loc: (isize, isize) = (2, (max_y + 4) as isize);

        // loop while the rock is not at rest
        next_command = Command::PushedByJet(next_command.index());
        loop {
            // move the rock
            let (new_command, new_rock_loc) = next_command.next(jets, rock_loc);
            let intersect = rock_shapes.intersect(new_rock_loc.0, new_rock_loc.1, &grid);
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
                //     println!("move from {:?} to {:?}", rock_loc, new_rock_loc);
                rock_loc = new_rock_loc;
            }
            next_command = new_command;
        }

        // put the rock in the previous location
        rock_shapes.fill_in_grid(rock_loc.0, rock_loc.1, &mut grid);

        max_y = rock_shapes.new_max_y(max_y, rock_loc);

        // print state
        // print_state(&grid, max_y);

        rock_shapes.next();
    }

    print_state(&grid, max_y);
    max_y + 1
}

type Grid = [[bool; 8088]; 7];

enum Command {
    PushedByJet(usize),
    FallingDown(usize),
}

impl Command {
    fn next(&self, jets: &String, pair: (isize, isize)) -> (Command, (isize, isize)) {
        match self {
            Command::PushedByJet(index) => {
                // println!("Pushed by Jet! {:?}", jets.chars().nth(*index));
                let new_pair = match jets.chars().nth(*index) {
                    Some('>') => (pair.0 + 1, pair.1),
                    Some('<') => (pair.0 - 1, pair.1),
                    _ => panic!("unknown jet char"),
                };
                (Command::FallingDown((*index + 1) % jets.len()), new_pair)
            }
            Command::FallingDown(index) => {
                // println!("Fall down");
                (Command::PushedByJet(*index), (pair.0, pair.1 - 1))
            }
        }
    }

    fn index(&self) -> usize {
        match self {
            Command::PushedByJet(index) => *index,
            Command::FallingDown(index) => *index,
        }
    }
}

type RockShape = Vec<(isize, isize)>;

struct RockShapes {
    shapes: Vec<RockShape>,
    iter: usize,
}

impl RockShapes {
    fn new() -> RockShapes {
        let shapes = vec![
            // Horizontal Line
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            // Cross
            vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            // Backwards L
            vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            // Veritcal Line
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            // Square
            vec![(0, 0), (1, 0), (1, 1), (0, 1)],
        ];
        RockShapes { shapes, iter: 0 }
    }

    fn next(&mut self) {
        self.iter = (self.iter + 1) % self.shapes.len();
    }

    fn intersect(&self, x: isize, y: isize, grid: &Grid) -> bool {
        for (plus_x, plus_y) in self.shapes[self.iter].iter() {
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

    fn fill_in_grid(&self, x: isize, y: isize, grid: &mut Grid) {
        for (plus_x, plus_y) in self.shapes[self.iter].iter() {
            let new_x = x + plus_x;
            let new_y = y + plus_y;
            grid[new_x as usize][new_y as usize] = true
        }
    }

    fn new_max_y(&self, cur_max_y: isize, cur_loc: (isize, isize)) -> isize {
        let mut max_y = cur_max_y;
        for (_, plus_y) in self.shapes[self.iter].iter() {
            // let new_x = cur_loc.0 + plus_x;
            let new_y = cur_loc.1 + plus_y;
            max_y = max_y.max(new_y);
        }
        max_y
    }
}

fn print_state(grid: &Grid, max_y: isize) {
    for y in (0..=max_y).rev() {
        print!("|");
        for x in 0..=6 {
            if grid[x][y as usize] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("+_______+");
    println!();
}

pub fn second(input: &[String]) -> usize {
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
