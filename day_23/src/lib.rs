use std::collections::HashMap;

pub fn first(input: &[String], max_rounds: usize) -> usize {
    let mut elves = parse_input(input);
    // println!("Starting position:");
    // elves.print();
    let consider_directions = ConsiderDirection::new();
    let mut consider_directions_index = 0;

    let mut round_num = 0;
    while round_num < max_rounds && round(&mut elves, &consider_directions, &mut consider_directions_index) {
        round_num += 1;
    }
    elves.empty_tiles()
}

pub fn second(input: &[String]) -> usize {
    0
}

fn round(elves: &mut Elves, consider_directions: &Vec<ConsiderDirection>, consider_index: &mut usize) -> bool {
    // part 1: find the proposals for all current elves
    let mut proposals: HashMap<Coord, Vec<Elf>> = HashMap::new();
    for (coord, elf) in elves.elves().iter() {
        let mut can_move_directions: Vec<&Direction> = vec!();
        for i in 0..consider_directions.len() {
            let consider_index = (i + *consider_index) % consider_directions.len();
            if consider_directions[consider_index].can_move(elf, elves) {
                can_move_directions.push(&consider_directions[consider_index].out_direction);
            }
        }
        
        // if an elf COULD move in any direction, it has no neighbors and thus won't move this round
        if can_move_directions.len() >= 4 {
            // println!("a guy found 4 directions to move!? {:?}", coord);
            continue;
        }

        let chosen_direction = can_move_directions.first();

        if let Some(d) = chosen_direction { 
            let proposals = proposals.entry(elf.neighbor(d)).or_insert(vec!());
            proposals.push(elf.clone());
        }
    }
    // println!("Proposals: {:?}", proposals);

    // part 2: move all proposals made by one and only one elf
    let mut anyone_moved = false;
    for (destination, proposing_elves) in proposals.iter() {
        if proposing_elves.len() > 1 {
            continue;
        }

        let moving_elf = &proposing_elves[0];
        elves.elves_mut().remove(&moving_elf.loc);
        elves.elves_mut().insert(*destination, Elf{loc: *destination});
        anyone_moved = true;
    }

    // part 3: print
    // elves.print();

    // part 4: increment index
    *consider_index = (*consider_index + 1) % consider_directions.len();

    anyone_moved
}



#[derive(Debug, Eq, PartialEq)]
struct Elves(HashMap<Coord, Elf>);
struct BoundingBox{
    min_x: isize, 
    min_y: isize,
    max_x: isize, 
    max_y: isize,
}

impl Elves {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn elves(&self) -> &HashMap<Coord, Elf> {
        let Elves(ret) = self;
        ret
    }

    fn elves_mut(&mut self) -> &mut HashMap<Coord, Elf> { 
        let Elves(ret) = self;
        ret
    }

    fn bounding_box(&self) -> BoundingBox {
        let Elves(elves) = self;
        let min_x = *elves.iter().map(|(Coord{x, y}, _)| x).min().unwrap_or(&0);
        let min_y = *elves.iter().map(|(Coord{x, y}, _)| y).min().unwrap_or(&0);
        let max_x = *elves.iter().map(|(Coord{x, y}, _)| x).max().unwrap_or(&0);
        let max_y = *elves.iter().map(|(Coord{x, y}, _)| y).max().unwrap_or(&0);
        BoundingBox{min_x, min_y, max_x, max_y}
    }

    fn print(&self) {
        let BoundingBox{min_x, min_y, max_x, max_y} = self.bounding_box();
        let x_span = max_x - min_x + 1;
        let y_span = max_y - min_y + 1;
        for y_offset in 0..y_span {
            let y = min_y + y_offset;
            for x_offset in 0..x_span {
                let x = min_x + x_offset;
                if self.elves().contains_key(&Coord{x, y}) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    fn empty_tiles(&self) -> usize {
        let BoundingBox{min_x, min_y, max_x, max_y} = self.bounding_box(); 
        let x_span = max_x - min_x + 1; 
        let y_span = max_y - min_y + 1;
        // println!("Gonna calculate empty tiles! {} {} {}", x_span, y_span, self.elves().len());
        ((x_span * y_span) as usize) - self.elves().len()
    }
}

struct ConsiderDirection {
    check_directions: Vec<Direction>,
    out_direction: Direction
}

impl ConsiderDirection {
    fn new() -> Vec<ConsiderDirection> { 
        vec!(
            ConsiderDirection{
                check_directions: vec!(Direction::NorthWest, Direction::North, Direction::NorthEast), 
                out_direction: Direction::North,
            },
            ConsiderDirection{
                check_directions: vec!(Direction::SouthWest, Direction::South, Direction::SouthEast), 
                out_direction: Direction::South,
            },
            ConsiderDirection{
                check_directions: vec!(Direction::NorthWest, Direction::West, Direction::SouthWest), 
                out_direction: Direction::West,
            },
            ConsiderDirection{
                check_directions: vec!(Direction::NorthEast, Direction::East, Direction::SouthEast), 
                out_direction: Direction::East,
            },
        )
    }

    fn can_move(&self, elf: &Elf, elves: &Elves) -> bool {
        let found_a_neighbor = self.check_directions.iter().find(|d| {
            let neighboring_coord = elf.neighbor(d);
            
            let res = elves.elves().contains_key(&neighboring_coord);
            // println!("Check if there's a neighbor {:?} {:?} {} {:?} {:?}", elf.loc, neighboring_coord, res, d, self.out_direction);
            res
        });

        found_a_neighbor.is_none()
    }
}

#[derive(Debug)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
struct Coord{
    x: isize,
    y: isize
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Elf {
    loc: Coord
}

impl Elf {
    fn neighbor(&self, direction: &Direction) -> Coord {
        match *direction {
            Direction::North => Coord{x: self.loc.x, y: self.loc.y - 1},
            Direction::NorthEast => Coord{x: self.loc.x + 1, y: self.loc.y - 1},
            Direction::East => Coord{x: self.loc.x + 1, y: self.loc.y},
            Direction::SouthEast => Coord{x: self.loc.x + 1, y: self.loc.y + 1},
            Direction::SouthWest => Coord{x: self.loc.x - 1, y: self.loc.y + 1},
            Direction::West => Coord{x: self.loc.x - 1, y: self.loc.y},
            Direction::NorthWest => Coord{x: self.loc.x - 1, y: self.loc.y - 1},
            Direction::South => Coord{x: self.loc.x, y: self.loc.y + 1},
            _ => panic!("unknown direction!")
        }
    }
}

fn parse_input(input: &[String]) -> Elves {
    let mut elves = Elves::new();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => elves.elves_mut().insert(Coord{x: x as isize, y: y as isize}, Elf{loc: Coord{x: x as isize, y: y as isize}}),
                _ => None,
            };
        }
    }
    elves
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
            ".....",
            "..##.",
            "..#..",
            ".....",
            "..##.",
            ".....",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    fn example_parsed() -> Elves { 
        let mut elves = HashMap::new();
        elves.insert(Coord{x: 2, y: 1}, Elf{loc: Coord{x: 2, y: 1}});
        elves.insert(Coord{x: 3, y: 1}, Elf{loc: Coord{x: 3, y: 1}});
        elves.insert(Coord{x: 2, y: 2}, Elf{loc: Coord{x: 2, y: 2}});
        elves.insert(Coord{x: 2, y: 4}, Elf{loc: Coord{x: 2, y: 4}});
        elves.insert(Coord{x: 3, y: 4}, Elf{loc: Coord{x: 3, y: 4}});
        Elves(elves)
    }
    
    fn example_2() -> Vec<String> { 
        let input: Vec<&str> = vec![
            "....#..",
            "..###.#",
            "#...#.#",
            ".#...##",
            "#.###..",
            "##.#.##",
            ".#..#..",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn test_round() { 
        let mut elves = example_parsed(); 
        let mut index = 0;
        let directions = ConsiderDirection::new();
        let moved = round(&mut elves, &directions, &mut index); 

        assert_eq!(index, 1);
        assert!(moved);
        assert!(elves.elves().contains_key(&Coord{x: 2, y: 0}));
        assert!(elves.elves().contains_key(&Coord{x: 3, y: 0}));
        assert!(elves.elves().contains_key(&Coord{x: 3, y: 3}));
        assert!(!elves.elves().contains_key(&Coord{x: 2, y: 3}));
    }

    #[test]
    fn test_empty_tiles() { 
        let elves = example_parsed();
        assert_eq!(elves.empty_tiles(), 3);
    }

    #[test]
    fn test_parse_input() { 
        let input = example(); 
        let parsed = parse_input(&input);
        assert_eq!(parsed, example_parsed());
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input, 100);
        assert_eq!(result, 25);

        let input = example_2();
        let result = first(&input, 10); 
        assert_eq!(result, 110)
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 0);
    }
}
