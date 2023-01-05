use std::collections::HashSet;

pub fn first(input: &[String], rows: usize, columns: usize) -> usize {
    // initial thought is the DFS calculating the blizzard state at each step
    // we only have to calculate states that are within a certain manhattan distance of our location based on the timestep? and we can gradually fill this in
    // and reuse it in between states.
    // you'd think that each step you wouldn't want to move backward, but because the blizzards move you might want to sometimes
    //
    // also we should preference DFS towards trending East, South, West, North, then wait
    //

    let mut tiles = Tiles::new(rows, columns, input);
    let mut queue = vec!(State::new(&tiles));
    let mut min_time = 1_000;
    let mut memo: HashSet<State> = HashSet::new(); 
    while let Some(ref state @ State{row, column, time}) = queue.pop() {
        if memo.contains(state) {
            continue;
        }
        memo.insert(state.clone());

        // println!("visiting state {:?}", state);
        if time >= min_time {
            // println!("past current min time tick {} {}", time, min_time);
            continue;
        }

        // try the waiting state unless a blizzard would catch us
        if let TileState::Clear = tiles.at(time + 1, row as isize, column as isize) {
            queue.push(State{row, column, time: time + 1});
        } else if let TileState::Start = tiles.at(time + 1, row as isize, column as isize) {
            queue.push(State{row, column, time: time + 1});
        }

        // try moving in each direction that is clear at this time
        for direction in Direction::all().iter() { 
            let new_row = (row as isize) + direction.to_delta().0;
            let new_column = (column as isize) + direction.to_delta().1;
            
            // println!("checking direction {:?} {} {}", direction, new_row, new_column);
            if let TileState::Clear = tiles.at(time + 1, new_row, new_column) {
                // println!("going in direction");
                let new_state = State{row: new_row as usize, column: new_column as usize, time: time + 1};
                // println!("pushing new state {:?}", new_state);
                queue.push(new_state);
            } else if let TileState::End = tiles.at(time + 1, new_row, new_column) {
                // println!("next is the end!");
                if (time + 1) < min_time {
                    println!("visiting state {:?}", state);
                    println!("checking direction {:?} {} {}", direction, new_row, new_column);
                    println!("new min! {} {}", time + 1, min_time);
                    min_time = time + 1;
                }
            }
        }
    }

    tiles.print();

    min_time
}

pub fn second(input: &[String]) -> usize {
    0
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct State {
    row: usize, 
    column: usize,
    time: usize
}

impl State {
    fn new(tiles: &Tiles) -> State {
        let (row, column) = tiles.start_position();
        State{row, column, time: 0}
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
enum Direction {
    East,
    West,
    North,
    South
}

impl Direction {
    // row, column
    fn to_delta(&self) -> (isize, isize) {
        match self {
            Direction::East => (0, 1),
            Direction::West => (0, -1),
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
        }
    }

    fn all() -> Vec<Direction> { 
        vec!(
            Direction::North,
            Direction::West, 
            Direction::South,
            Direction::East, 
        )
    }

    fn opposite(&self) -> &Direction {
        match self {
            Direction::East => &Direction::West,
            Direction::West => &Direction::East,
            Direction::North => &Direction::South,
            Direction::South => &Direction::North,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum TileState {
    NotCalculated,
    Clear,
    Start,
    End,
    Wall,
    Blizzards(HashSet<Direction>)
}

#[derive(Debug)]
// indexed by time stamp (0 for initial state), row, column
struct Tiles{
    tiles: Vec<Vec<Vec<TileState>>>,
    rows: usize, 
    columns: usize,
}

impl Tiles {
    fn new(rows: usize, columns: usize, input: &[String]) -> Tiles {
        let rows = rows + 2;
        let columns = columns + 2;
        let mut tiles = vec!(vec![vec![TileState::NotCalculated; columns]; rows]);
        for (row, line) in input.iter().enumerate() { 
            for (column, c) in line.chars().enumerate() {
                let tile_state = match c {
                    '#' => TileState::Wall,
                    '.' if row == 0 => TileState::Start,
                    '.' if (row + 1) == rows => TileState::End,
                    '.'  => TileState::Clear,
                    '>' => TileState::Blizzards(HashSet::from([Direction::East])),
                    '^' => TileState::Blizzards(HashSet::from([Direction::North])),
                    'v' => TileState::Blizzards(HashSet::from([Direction::South])),
                    '<' => TileState::Blizzards(HashSet::from([Direction::West])),
                    _ => panic!("Unknown input char {}", c),
                };
                tiles[0][row][column] = tile_state;
            }
        }
        Tiles{tiles, rows, columns}
    }

    fn start_position(&self) -> (usize, usize) {
        for (row, columns) in self.tiles[0].iter().enumerate() { 
            for (column, state) in columns.iter().enumerate() { 
                if let TileState::Start = state {
                    return (row, column)
                }
            }
        }
        (0, 0)
    }
    
    fn end_position(&self) -> (usize, usize) {
        for (row, columns) in self.tiles[0].iter().enumerate() { 
            for (column, state) in columns.iter().enumerate() { 
                if let TileState::End = state {
                    return (row, column)
                }
            }
        }
        (0, 0)
    }

    fn at(&mut self, time: usize, row: isize, column: isize) -> &TileState {
        // println!("Checking at {} {} {} {}", row, column, self.rows, self.columns);
        if row < 0 || column < 0 {
            return &TileState::Wall;
        }

        let row = row as usize; 
        let column = column as usize;

        if row >= self.rows || column >= self.columns {
            return &TileState::Wall;
        }

        if time >= self.tiles.len() {
            for _ in 0..=(time - self.tiles.len()) {
                self.tiles.push(vec![vec![TileState::NotCalculated; self.columns]; self.rows]);
            }
        }
        if let TileState::NotCalculated = self.tiles[time][row][column] {
            return self.calculate(time, row, column);
        } 

        // println!("returning {:?}", &self.tiles[time][row][column]);
        &self.tiles[time][row][column]
    }

    fn calculate(&mut self, time: usize, row: usize, column: usize) -> &TileState {
        let starting_state = &self.tiles[0][row][column];
        let new_state = match *starting_state {
            // walls, start, and end don't move and blizzards don't move into them
            TileState::Start => TileState::Start,
            TileState::End => TileState::End,
            TileState::Wall => TileState::Wall,
            // otherwise, it's one of the field tiles in the middle.  We must find any blizzards that have moved into here.
            // If there's a blizzard here now, at the initial state it must have been at a point time units away from us,
            // in the neighborly direction, wrapping around walls.
            // Must we calculate all 4 directions? no, since this step isn't used as the basis for later steps, we can 
            // give up as soon as we find a blizzard.
            _ =>  {
                let mut blizzards_here = HashSet::new();
                for direction in Direction::all().iter() {
                    // what location is (time) units in the direction we're considering
                    // this calc is weird because we want to % within the walls - so #..>#; (col - 1) starting point, + delta, % (cols - 2 for the walls), then add back the 1 wall.
                    let new_row = 1 + ((row as isize) - 1 + direction.to_delta().0 * (time as isize)).rem_euclid(self.rows as isize - 2);
                    let new_column = 1 + ((column as isize) - 1 + direction.to_delta().1 * (time as isize)).rem_euclid(self.columns as isize - 2);

                    // is there a blizzard heading in the opposite direction at that location at time = 0?
                    if let TileState::Blizzards(set) = &self.tiles[0][new_row as usize][new_column as usize] {
                        // println!("found blizzards");
                        if set.contains(direction.opposite()) {
                            // println!("found opposite");
                            blizzards_here.insert(*direction.opposite());
                        }
                    }
                }

                if blizzards_here.is_empty() {
                    TileState::Clear
                } else {
                    TileState::Blizzards(blizzards_here)
                }
            },
        };
        self.tiles[time][row][column] = new_state;
        &self.tiles[time][row][column]
    }

    fn print(&self) {
        for (timestamp, v) in self.tiles.iter().enumerate() {
            println!("State at timestamp {}:", timestamp);
            for row in 0..self.rows {
                for column in 0..self.columns {
                    let to_print = match self.tiles[timestamp][row][column] {
                        TileState::Start => ".",
                        TileState::End => ".",
                        TileState::Clear => ".",
                        TileState::Wall => "#",
                        TileState::Blizzards(_) => "B",
                        TileState::NotCalculated => "_",
                    };
                    print!("{}", to_print);
                }
                println!();
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
            "#.#####",
            "#.....#",
            "#>....#",
            "#.....#",
            "#...v.#",
            "#.....#",
            "#####.#",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    fn example_2() -> Vec<String> { 
        let input: Vec<&str> = vec![
            "#.######",
            "#>>.<^<#",
            "#.<..<<#",
            "#>v.><>#",
            "#<^v^^>#",
            "######.#",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn test_create() {
        let input = example(); 
        let mut tiles = Tiles::new(5, 5, &input);
        assert_eq!(*tiles.at(0, 0, 1), TileState::Start);
        assert_eq!(*tiles.at(0, 6, 5), TileState::End);
        assert_eq!(*tiles.at(0, 1, 1), TileState::Clear);
        assert_eq!(*tiles.at(0, 2, 1), TileState::Blizzards(HashSet::from([Direction::East])));
    }

    #[test]
    fn test_calculate() { 
        let input = example(); 
        let mut tiles = Tiles::new(5, 5, &input);
        assert_eq!(*tiles.at(1, 2, 1), TileState::Clear);
        assert_eq!(*tiles.at(1, 2, 2), TileState::Blizzards(HashSet::from([Direction::East])));

        // visiting state State { row: 2, column: 2, time: 10 } seems to be a tricky entry
        let input = example_2();
        let mut tiles = Tiles::new(4, 6, &input);
        assert_eq!(*tiles.at(10, 2, 2), TileState::Blizzards(HashSet::from([Direction::West, Direction::North])));
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input, 5, 5);
        assert_eq!(result, 10);

        let input = example_2();
        let result = first(&input, 4, 6);
        assert_eq!(result, 18);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 0);
    }
}
