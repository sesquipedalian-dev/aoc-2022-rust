pub fn first(input: &[String]) -> isize {
    let (rows, moves) = parse_input(&input);
    let mut location: (usize, usize) = (0, 0);
    let mut direction: (isize, isize) = (1, 0);

    for next_move in moves.iter() {
        println!(
            "current location ({:?}) in direction ({:?}) {:?}",
            location, direction, next_move
        );
        match next_move {
            Move::Forward(i) if direction.0 != 0 => {
                for _ in 0..*i {
                    let next_y: usize = location.1;
                    let next_x: i32 = (location.0 as isize + direction.0) as i32;
                    let next_x = next_x.rem_euclid(rows[next_y].walls.len() as i32) as usize;

                    if rows[next_y].walls[next_x] {
                        break;
                    } else {
                        location = (next_x, next_y);
                    }
                }
            }
            Move::Forward(i) if direction.1 != 0 => {
                let mut move_count: usize = 0;
                let mut current_y = location.1;
                let mut last_landing_y = current_y;
                loop {
                    if move_count >= *i {
                        break;
                    }

                    let normal_current_x = rows[location.1].offset + location.0;
                    let next_y: i32 = (current_y as isize + direction.1) as i32;
                    let next_y = next_y.rem_euclid(rows.len() as i32) as usize;

                    if normal_current_x >= rows[next_y].offset
                        && (normal_current_x - rows[next_y].offset) < rows[next_y].walls.len()
                    {
                        if rows[next_y].walls[normal_current_x - rows[next_y].offset] {
                            break;
                        }
                        current_y = next_y;
                        last_landing_y = next_y;
                        move_count += 1;
                    } else {
                        current_y = next_y;
                    }
                }

                println!(
                    " doing maths {} + {} - {}, {}",
                    location.0,
                    rows[location.1].offset,
                    rows[last_landing_y].offset,
                    last_landing_y
                );
                location = (
                    location.0 + rows[location.1].offset - rows[last_landing_y].offset,
                    last_landing_y,
                );
            }
            Move::TurnToward(d) => direction = *d,
            _ => panic!("unknown next move!"),
        }
    }

    let col_part: isize = 1000 * (location.1 as isize + 1);
    let row_part: isize = 4 * (location.0 as isize + 1);
    let direction_part: isize = match direction {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => panic!("unknown direction"),
    };
    col_part + row_part + direction_part
}

pub fn second(input: &[String]) -> usize {
    // to adjust the coordinates for the test input, need to adjust CUBE_SIZE = 3 and current face = 0.
    const CUBE_SIZE: usize = 49;

    let (mut faces, moves) = parse_input_part_2(&input);

    let mut location: Coord = Coord { x: 0, y: 0 };
    let mut direction: Direction = Direction { x: 1, y: 0 };
    let mut current_face = 0usize;

    for next_move in moves.iter() {
        println!(
            "current location {} ({:?}) in direction ({:?}) {:?}",
            current_face, location, direction, next_move
        );
        match next_move {
            Move::Forward(i) => {
                'move_steps: for step_num in 0..*i {
                    let next_x: isize = (location.x as isize) + direction.x;
                    let next_y: isize = (location.y as isize) + direction.y;
                    let mut next_face = current_face;

                    let possible_neighbor_change = if next_x < 0 {
                        Some(&mut faces[current_face].neighbors[0])
                    } else if (next_x as usize) > CUBE_SIZE {
                        Some(&mut faces[current_face].neighbors[1])
                    } else if next_y < 0 {
                        Some(&mut faces[current_face].neighbors[2])
                    } else if (next_y as usize) > CUBE_SIZE {
                        Some(&mut faces[current_face].neighbors[3])
                    } else {
                        None
                    };

                    let (next_location, next_face, next_direction) = match possible_neighbor_change
                    {
                        Some(neighbor) => {
                            next_face = neighbor.0;
                            let (new_coord, new_direction) = neighbor.1(location);
                            println!(
                                "moving between faces! {} from {} {:?} {:?} to {} {:?} {:?}",
                                step_num,
                                current_face,
                                location,
                                direction,
                                next_face,
                                new_coord,
                                new_direction
                            );
                            (new_coord, next_face, new_direction)
                        }
                        _ => (
                            Coord {
                                x: next_x as usize,
                                y: next_y as usize,
                            },
                            current_face,
                            direction,
                        ),
                    };

                    if faces[next_face].walls[next_location.y][next_location.x] {
                        // hit a wall, stop!
                        println!("Hit a wall at step {} of {}", step_num, *i);
                        break 'move_steps;
                    }

                    location = next_location;
                    direction = next_direction;
                    current_face = next_face;
                }
            }
            Move::Turn(TurnDirection::Right) => {
                direction = match direction {
                    Direction { x: 1, y: 0 } => Direction { x: 0, y: 1 },
                    Direction { x: 0, y: 1 } => Direction { x: -1, y: 0 },
                    Direction { x: -1, y: 0 } => Direction { x: 0, y: -1 },
                    Direction { x: 0, y: -1 } => Direction { x: 1, y: 0 },
                    _ => panic!("unknown direction turn right!"),
                }
            }
            Move::Turn(TurnDirection::Left) => {
                direction = match direction {
                    Direction { x: 1, y: 0 } => Direction { x: 0, y: -1 },
                    Direction { x: 0, y: 1 } => Direction { x: 1, y: 0 },
                    Direction { x: -1, y: 0 } => Direction { x: 0, y: 1 },
                    Direction { x: 0, y: -1 } => Direction { x: -1, y: 0 },
                    _ => panic!("unknown direction turn right!"),
                }
            }
            _ => panic!("unknown next move!"),
        }
    }

    println!(
        "current location {} ({:?}) in direction ({:?})",
        current_face, location, direction
    );
    // convert face x / y to absolute on our grid
    let x_offset = match current_face {
        0 => CUBE_SIZE + 1,
        1 => (CUBE_SIZE + 1) * 2,
        2 => CUBE_SIZE + 1,
        3 => 0,
        4 => CUBE_SIZE + 1,
        5 => 0,
        _ => 0,
    };
    let col_part = 4 * (location.x + 1 + x_offset); // 4 * 4 = 16

    let y_offset = match current_face {
        0 => 0usize,
        1 => 0usize,
        2 => (CUBE_SIZE + 1) + 1,
        3 => (CUBE_SIZE + 1) * 2,
        4 => (CUBE_SIZE + 1) * 2,
        5 => (CUBE_SIZE + 1) * 3,
        _ => 0,
    };
    let row_part = 1000 * (location.y + 1 + y_offset); // 153 k
    let direction_part = match direction {
        Direction { x: 1, y: 0 } => 0,
        Direction { x: 0, y: 1 } => 1,
        Direction { x: -1, y: 0 } => 2,
        Direction { x: 0, y: -1 } => 3,
        _ => panic!("unknown direction"),
    };
    col_part + row_part + direction_part
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Move {
    Forward(usize),             // number of tiles to move
    TurnToward((isize, isize)), // direction represents x / y deltas of a move; for example (0, 1) is moving downward
    Turn(TurnDirection),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TurnDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Row {
    offset: usize,
    walls: Vec<bool>, // true if blocked
}

fn parse_input(input: &[String]) -> (Vec<Row>, Vec<Move>) {
    let mut maze = vec![];
    let mut moves = vec![];
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

            let mut last_turn = Move::TurnToward((1, 0));
            while let Some(c) = chars.next() {
                match c {
                    'R' => {
                        if forward.len() > 0 {
                            moves.push(Move::Forward(forward.parse().unwrap()));
                            forward.clear();
                        }
                        let next_turn = Move::TurnToward(match last_turn {
                            Move::TurnToward((0, 1)) => (-1, 0),
                            Move::TurnToward((0, -1)) => (1, 0),
                            Move::TurnToward((1, 0)) => (0, 1),
                            Move::TurnToward((-1, 0)) => (0, -1),
                            _ => panic!("unknown direction!"),
                        });
                        last_turn = next_turn.clone();
                        moves.push(next_turn)
                    }
                    'L' => {
                        if forward.len() > 0 {
                            moves.push(Move::Forward(forward.parse().unwrap()));
                            forward.clear();
                        }
                        let next_turn = Move::TurnToward(match last_turn {
                            Move::TurnToward((0, 1)) => (1, 0),
                            Move::TurnToward((0, -1)) => (-1, 0),
                            Move::TurnToward((1, 0)) => (0, -1),
                            Move::TurnToward((-1, 0)) => (0, 1),
                            _ => panic!("unknown direction!"),
                        });
                        last_turn = next_turn.clone();
                        moves.push(next_turn)
                    }
                    c if '0' <= c && c <= '9' => forward.push(c),
                    _ => panic!("unknown move designation!"),
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

            let mut walls = vec![];
            while let Some(c) = chars.next() {
                match c {
                    '.' => walls.push(false),
                    '#' => walls.push(true),
                    _ => break,
                }
            }

            maze.push(Row { offset, walls })
        }
    }

    (maze, moves)
}

#[derive(Clone, Debug, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug, Copy)]
struct Direction {
    x: isize,
    y: isize,
}

// function takes the coords / direction in the current face and gives us the coords / direction one step into the neighboring face
type NeighborEntryFunction = dyn FnMut(Coord) -> (Coord, Direction);

// #[derive(Clone)]
struct Face {
    walls: Vec<Vec<bool>>,
    neighbors: Vec<(usize, Box<NeighborEntryFunction>)>, // L, R, U, D. usize is index in faces array
}

fn parse_input_part_2<'a>(input: &[String]) -> (Vec<Face>, Vec<Move>) {
    // was having trouble with a cube_size parameter to the function because it's captured in the closures
    // so adjust this 'constant' as needed to run the example or the real input
    const CUBE_SIZE: usize = 49;

    // assume net structure from  my input
    //    0 1
    //    2
    //  3 4
    //  5
    let mut faces = vec![
        Face {
            walls: vec![],
            neighbors: vec![],
        },
        Face {
            walls: vec![],
            neighbors: vec![],
        },
        Face {
            walls: vec![],
            neighbors: vec![],
        },
        Face {
            walls: vec![],
            neighbors: vec![],
        },
        Face {
            walls: vec![],
            neighbors: vec![],
        },
        Face {
            walls: vec![],
            neighbors: vec![],
        },
    ];

    faces[0].neighbors.push((
        3,
        Box::new(|Coord { x, y }| {
            (
                Coord {
                    x: 0,
                    y: CUBE_SIZE - y,
                },
                Direction { x: 1, y: 0 },
            )
        }),
    ));
    faces[0].neighbors.push((
        1,
        Box::new(|Coord { x, y }| (Coord { x: 0, y }, Direction { x: 1, y: 0 })),
    ));
    faces[0].neighbors.push((
        5,
        Box::new(|Coord { x, y }| (Coord { x: 0, y: x }, Direction { x: 1, y: 0 })),
    ));
    faces[0].neighbors.push((
        2,
        Box::new(|Coord { x, y }| (Coord { x, y: 0 }, Direction { x: 0, y: 1 })),
    ));

    faces[1].neighbors.push((
        0,
        Box::new(|Coord { x, y }| (Coord { x: CUBE_SIZE, y }, Direction { x: -1, y: 0 })),
    ));
    faces[1].neighbors.push((
        4,
        Box::new(|Coord { x, y }| {
            (
                Coord {
                    x: CUBE_SIZE,
                    y: CUBE_SIZE - y,
                },
                Direction { x: -1, y: 0 },
            )
        }),
    ));
    faces[1].neighbors.push((
        5,
        Box::new(|Coord { x, y }| (Coord { x: x, y: CUBE_SIZE }, Direction { x: 0, y: -1 })),
    ));
    faces[1].neighbors.push((
        2,
        Box::new(|Coord { x, y }| (Coord { x: CUBE_SIZE, y: x }, Direction { x: -1, y: 0 })),
    ));

    faces[2].neighbors.push((
        3,
        Box::new(|Coord { x, y }| (Coord { x: y, y: 0 }, Direction { x: 0, y: 1 })),
    ));
    faces[2].neighbors.push((
        1,
        Box::new(|Coord { x, y }| (Coord { x: y, y: CUBE_SIZE }, Direction { x: 0, y: -1 })),
    ));
    faces[2].neighbors.push((
        0,
        Box::new(|Coord { x, y }| (Coord { x: x, y: CUBE_SIZE }, Direction { x: 0, y: -1 })),
    ));
    faces[2].neighbors.push((
        4,
        Box::new(|Coord { x, y }| (Coord { x, y: 0 }, Direction { x: 0, y: 1 })),
    ));

    faces[3].neighbors.push((
        0,
        Box::new(|Coord { x, y }| {
            (
                Coord {
                    x: 0,
                    y: CUBE_SIZE - y,
                },
                Direction { x: 1, y: 0 },
            )
        }),
    ));
    faces[3].neighbors.push((
        4,
        Box::new(|Coord { x, y }| (Coord { x: 0, y }, Direction { x: 1, y: 0 })),
    ));
    faces[3].neighbors.push((
        2,
        Box::new(|Coord { x, y }| (Coord { x: 0, y: x }, Direction { x: 1, y: 0 })),
    ));
    faces[3].neighbors.push((
        5,
        Box::new(|Coord { x, y }| (Coord { x, y: 0 }, Direction { x: 0, y: 1 })),
    ));

    faces[4].neighbors.push((
        3,
        Box::new(|Coord { x, y }| (Coord { x: CUBE_SIZE, y: y }, Direction { x: -1, y: 0 })),
    ));
    faces[4].neighbors.push((
        1,
        Box::new(|Coord { x, y }| {
            (
                Coord {
                    x: CUBE_SIZE,
                    y: CUBE_SIZE - y,
                },
                Direction { x: -1, y: 0 },
            )
        }),
    ));
    faces[4].neighbors.push((
        2,
        Box::new(|Coord { x, y }| (Coord { x, y: CUBE_SIZE }, Direction { x: 0, y: -1 })),
    ));
    faces[4].neighbors.push((
        5,
        Box::new(|Coord { x, y }| (Coord { x: CUBE_SIZE, y: x }, Direction { x: -1, y: 0 })),
    ));

    faces[5].neighbors.push((
        0,
        Box::new(|Coord { x, y }| (Coord { x: y, y: 0 }, Direction { x: 0, y: 1 })),
    ));
    faces[5].neighbors.push((
        4,
        Box::new(|Coord { x, y }| (Coord { x: y, y: CUBE_SIZE }, Direction { x: 0, y: -1 })),
    ));
    faces[5].neighbors.push((
        3,
        Box::new(|Coord { x, y }| (Coord { x, y: CUBE_SIZE }, Direction { x: 0, y: -1 })),
    ));
    faces[5].neighbors.push((
        1,
        Box::new(|Coord { x, y }| (Coord { x, y: 0 }, Direction { x: 0, y: 1 })),
    ));

    // now parse the input to fill in the walls
    let mut moves = vec![];
    let mut looking_for_move = false;
    let mut maze_index = 0usize;
    let mut row_index = 0usize;
    for line in input {
        if line.len() < 1 {
            looking_for_move = true;
            continue;
        }

        if looking_for_move {
            // e.g. 10R5L5R10L4R5L5
            let mut forward = String::new();
            let mut chars = line.chars();

            while let Some(c) = chars.next() {
                match c {
                    'R' => {
                        if forward.len() > 0 {
                            moves.push(Move::Forward(forward.parse().unwrap()));
                            forward.clear();
                        }
                        moves.push(Move::Turn(TurnDirection::Right));
                    }
                    'L' => {
                        if forward.len() > 0 {
                            moves.push(Move::Forward(forward.parse().unwrap()));
                            forward.clear();
                        }

                        moves.push(Move::Turn(TurnDirection::Left));
                    }
                    c if '0' <= c && c <= '9' => forward.push(c),
                    _ => panic!("unknown move designation!"),
                }
            }
            if forward.len() > 0 {
                moves.push(Move::Forward(forward.parse().unwrap()));
                forward.clear();
            }
        } else {
            if row_index > CUBE_SIZE {
                println!("About to roll row {} {}", row_index, line);
                row_index = 0;
                maze_index = faces
                    .iter()
                    .enumerate()
                    .find(|(_, face)| face.walls.len() == 0)
                    .unwrap()
                    .0;
            }
            let mut chars = line.chars().peekable();
            while let Some(' ') = chars.peek() {
                chars.next();
            }

            let orig_maze_index = maze_index;
            faces[maze_index].walls.push(vec![]);
            let mut col_index = 0usize;
            let mut last_row_index = faces[maze_index].walls.len() - 1;
            // let mut walls = vec![];
            while let Some(c) = chars.next() {
                if col_index > CUBE_SIZE {
                    maze_index += 1;
                    faces[maze_index].walls.push(vec![]);
                    last_row_index = faces[maze_index].walls.len() - 1;
                    col_index = 0;
                }
                let val_to_push = match c {
                    '.' => false,
                    '#' => true,
                    _ => break,
                };

                faces[maze_index].walls[last_row_index].push(val_to_push);
                col_index += 1;
            }

            maze_index = orig_maze_index;
            row_index += 1;
        }
    }

    for (i, face) in faces.iter().enumerate() {
        println!("Face {}:", i);
        for row in face.walls.iter() {
            for wall in row.iter() {
                if *wall {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    (faces, moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
            "        ...#    ",
            "        .#..    ",
            "        #...    ",
            "        ....    ",
            "...#.......#    ",
            "........#...    ",
            "..#....#....    ",
            "..........#.    ",
            "        ...#....",
            "        .....#..",
            "        .#......",
            "        ......#.",
            "",
            "10R5L5R10L4R5L5",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    // rotating example() to match the flattened cube structure of my part 2 input
    fn example_rot_for_2() -> Vec<String> {
        let input: Vec<&str> = vec![
            "    .....#..",
            "    .#....#.",
            "    ........",
            "    #.......",
            "    ...#",
            "    .#..",
            "    #...",
            "    ....",
            ".......#",
            "....#...",
            "........",
            "...#..#.",
            "....",
            ".#..",
            "....",
            "...#",
            "",
            "10R5L5R10L4R5L5",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    fn example_3() -> Vec<String> {
        let input: Vec<&str> = vec![
            "    .....#..",
            "    .#....#.",
            "    ........",
            "    #.......",
            "    ...#",
            "    .#..",
            "    #...",
            "    ....",
            ".......#",
            "....#...",
            "........",
            "...#..#.",
            "....",
            ".#..",
            "....",
            "...#",
            "",
            "RR2",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn test_parse_input() {
        let input = example();
        let (rows, moves) = parse_input(&input);

        assert_eq!(
            rows[0],
            Row {
                offset: 8,
                walls: vec!(false, false, false, true)
            }
        );
        assert_eq!(moves[0], Move::Forward(10));
        assert_eq!(moves[1], Move::TurnToward((0, 1)));
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, 6032);
    }

    #[test]
    fn second_test() {
        // let input = example_rot_for_2();
        let input = example_3();
        let result = second(&input);
        assert_eq!(result, 5031);
    }
}
