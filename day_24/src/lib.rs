pub fn first(input: &[String]) -> usize {
    // initial thought is the DFS calculating the blizzard state at each step
    // we only have to calculate states that are within a certain manhattan distance of our location based on the timestep? and we can gradually fill this in
    // and reuse it in between states.
    // you'd think that each step you wouldn't want to move backward, but because the blizzards move you might want to sometimes
    //
    // also we should preference DFS towards trending East, South, West, North, then wait
    //
    0
}

pub fn second(input: &[String]) -> usize {
    0
}

#[derive(Clone, Debug)]
enum Direction {
    East,
    West,
    North,
    South
}

#[derive(Clone, Debug)]
enum TileState {
    NotCalculated,
    Clear,
    Start,
    End,
    Wall,
    Blizzards(Vec<Direction>)
}

#[derive(Debug)]
// indexed by time stamp (0 for initial state), row, column
struct Tiles{
    tiles: Vec<Vec<Vec<TileState>>>,
    rows: usize, 
    columns: usize,
}

impl Tiles {
    fn new(rows: usize, columns: usize) -> Tiles {
        Tiles{tiles: vec!(vec![vec![TileState::NotCalculated; columns]; rows]), rows, columns}
    }

    fn at(&mut self, time: usize, row: usize, column: usize) -> &TileState {
        if let TileState::NotCalculated = self.tiles[time][row][column] {
            return self.calculate(time, row, column);
        } 
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
            _ => TileState::NotCalculated,
        };
        self.tiles[time][row][column] = new_state;
        &self.tiles[time][row][column]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
            
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 0);
    }
}
