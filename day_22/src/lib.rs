pub fn first(input: &[String]) -> usize {
    0
}

pub fn second(input: &[String]) -> usize {
   0
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Move {
    Forward(usize), // number of tiles to move
    Turn((isize, isize)), // direction represents x / y deltas of a move; for example (0, 1) is moving downward
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Row{
    offset: usize, 
    walls: Vec<bool>, // true if blocked
}

fn parse_input(input: &[String]) -> (Vec<Row>, Vec<Move>) {
    let mut maze = vec!();
    let mut moves = vec!();
    let mut looking_for_move = false;

    for line in input {
        if line.len() < 1 {
            looking_for_move = true;
            continue;
        }

        if looking_for_move { 
            // e.g. 10R5L5R10L4R5L5
            let mut forward = String::new();
            let mut chars = line.chars();

            let mut last_turn = Move::Turn((1, 0));
            while let Some(c) = chars.next() {
                match c {
                    'R' => {
                        if forward.len() > 0 {
                            moves.push(Move::Forward(forward.parse().unwrap()));
                            forward.clear();
                        }
                        let next_turn = Move::Turn(match last_turn {
                            Move::Turn((0, 1)) => (-1, 0),
                            Move::Turn((0, -1)) => (1, 0),
                            Move::Turn((1, 0)) => (0, 1), 
                            Move::Turn((-1, 0)) => (0, -1),
                            _ => panic!("unknown direction!")
                        });
                        last_turn = next_turn.clone();
                        moves.push(next_turn)
                    },
                    'L' => {
                        if forward.len() > 0 {
                            moves.push(Move::Forward(forward.parse().unwrap()));
                            forward.clear();
                        }
                        let next_turn = Move::Turn(match last_turn {
                            Move::Turn((0, 1)) => (1, 0),
                            Move::Turn((0, -1)) => (-1, 0),
                            Move::Turn((1, 0)) => (0, -1), 
                            Move::Turn((-1, 0)) => (0, 1),
                            _ => panic!("unknown direction!")
                        });
                        last_turn = next_turn.clone();
                        moves.push(next_turn)
                    },
                    c if '0' <= c && c <= '9' => forward.push(c),
                    _ => panic!("unknown move designation!")
                }
            }
            if forward.len() > 0 {
                moves.push(Move::Forward(forward.parse().unwrap()));
                forward.clear();
            }
        } else {
            let mut chars = line.chars().peekable();
            let mut offset = 0;
            while let Some(' ') = chars.peek() {
                offset += 1;
                chars.next();
            }
            // hm, need to put back a value

            let mut walls = vec!();
            while let Some(c) = chars.next() {
                match c {
                    '.' => walls.push(false),
                    '#' => walls.push(true),
                    _ => break
                }
            }

            maze.push(Row{offset, walls})
        }
    }

    (maze, moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
    "        ...#     ",
    "        .#..     ",
    "        #...     ",
    "        ....     ",
    "...#.......#     ",
    "........#...     ",
    "..#....#....     ",
    "..........#.     ",
    "        ...#....",
    "        .....#..",
    "        .#......",
    "        ......#.",
    "",
    "10R5L5R10L4R5L5",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn test_parse_input() {
        let input = example();
        let (rows, moves) = parse_input(&input);

        
        assert_eq!(rows[0], Row{offset: 8, walls: vec!(false, false, false, true)});
        assert_eq!(moves[0], Move::Forward(10));
        assert_eq!(moves[1], Move::Turn((0, 1)));
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
