use std::collections::HashSet;

pub fn first(input: &[String]) -> usize {
    let (mut rock_coords, max_y) = parse_input(&input);
    let mut generation = 0;

    loop {
        let mut sand_iter = SandIter::new(&rock_coords, PartMaxY::Part1(max_y));
        let mut last_sand_move_result = SandMoveResult::NeverAtRest;
        loop {
            last_sand_move_result = sand_iter.try_move();
            if let SandMoveResult::Moving(_) = last_sand_move_result {
            } else {
                break;
            }
        }

        match last_sand_move_result {
            SandMoveResult::AtRest(coord) => {
                rock_coords.insert(coord);
            }
            SandMoveResult::NeverAtRest => {
                break;
            }
            o => {
                panic!("unexpected move result! {:?}", o);
            } // never happens
        }
        generation += 1
    }

    generation
}

pub fn second(input: &[String]) -> usize {
    let (mut rock_coords, max_y) = parse_input(&input);
    let mut generation = 0;

    loop {
        let mut sand_iter = SandIter::new(&rock_coords, PartMaxY::Part2(max_y + 2));
        let mut last_sand_move_result = SandMoveResult::NeverAtRest;
        loop {
            last_sand_move_result = sand_iter.try_move();
            if let SandMoveResult::Moving(_) = last_sand_move_result {
            } else {
                break;
            }
        }

        generation += 1;

        match last_sand_move_result {
            SandMoveResult::AtRest(coord) => {
                if coord == Coord(500, 0) {
                    break;
                }
                rock_coords.insert(coord);
            }
            o => {
                panic!("unexpected move result! {:?}", o);
            } // never happens
        }
    }

    generation
}

#[derive(Debug)]
enum SandMoveResult {
    NeverAtRest,
    AtRest(Coord),
    Moving(Coord),
}

#[derive(PartialEq, Eq)]
enum PartMaxY {
    Part1(usize),
    Part2(usize),
}

struct SandIter<'a> {
    rock_coords: &'a HashSet<Coord>,
    max_y: PartMaxY,
    current_location: Coord,
}

impl<'a> SandIter<'a> {
    fn new(rock_coords: &HashSet<Coord>, max_y: PartMaxY) -> SandIter {
        SandIter {
            rock_coords,
            current_location: Coord(500, 0),
            max_y,
        }
    }

    fn try_move(&mut self) -> SandMoveResult {
        if let PartMaxY::Part1(max_y) = self.max_y {
            if self.current_location.1 >= max_y {
                return SandMoveResult::NeverAtRest;
            }
        }

        let potential_directions = vec![
            Coord(self.current_location.0, self.current_location.1 + 1),
            Coord(self.current_location.0 - 1, self.current_location.1 + 1),
            Coord(self.current_location.0 + 1, self.current_location.1 + 1),
        ];

        potential_directions
            .iter()
            .find(|direction| {
                !(self.rock_coords.contains(direction)
                    || self.max_y == PartMaxY::Part2(direction.1))
            })
            .map(|found_direction| {
                self.current_location = *found_direction;
                SandMoveResult::Moving(*found_direction)
            })
            .unwrap_or(SandMoveResult::AtRest(self.current_location))
    }
}

#[derive(PartialEq, Hash, Eq, Clone, Copy, Debug)]
struct Coord(usize, usize);

// looks like my inputs range from 400-600
// HashSet<Coords> because of relatively sparse grid
fn parse_input(input: &[String]) -> (HashSet<Coord>, usize) {
    let mut rock_coords = HashSet::new();
    let mut max_y_coord = Coord(0, 0);

    for line in input {
        let mut last_coord: Option<Coord> = None;
        let coords = line.split(" -> ");
        for rock_coord_str in coords {
            let mut coord_parts = rock_coord_str.split(",");
            let rock_coord = Coord(
                coord_parts.next().unwrap().parse().unwrap(),
                coord_parts.next().unwrap().parse().unwrap(),
            );
            if let Some(prev_coord) = last_coord {
                let (min_x, max_x) = if rock_coord.0 < prev_coord.0 {
                    (rock_coord.0, prev_coord.0)
                } else {
                    (prev_coord.0, rock_coord.0)
                };
                let (min_y, max_y) = if rock_coord.1 < prev_coord.1 {
                    (rock_coord.1, prev_coord.1)
                } else {
                    (prev_coord.1, rock_coord.1)
                };
                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        let new_coord = Coord(x, y);
                        rock_coords.insert(new_coord);

                        if y > max_y_coord.1 {
                            max_y_coord = new_coord
                        }
                    }
                }
            }
            last_coord = Some(rock_coord);
        }
    }

    (rock_coords, max_y_coord.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
            "498,4 -> 498,6 -> 496,6",
            "503,4 -> 502,4 -> 502,9 -> 494,9",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn test_parse_input() {
        let input = example();
        let (rocks, max_y) = parse_input(&input);

        assert_eq!(rocks.contains(&Coord(498, 4)), true);
        assert_eq!(rocks.contains(&Coord(500, 0)), false);
        assert_eq!(rocks.contains(&Coord(498, 5)), true);
        assert_eq!(max_y, 9);
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, 24);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 93);
    }
}
