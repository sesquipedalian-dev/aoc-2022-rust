use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Coord(i32, i32);

pub fn first(input: &[String]) -> usize {
    let mut tail_seen_locations: HashSet<Coord> = HashSet::new();
    let mut head_location = Coord(0, 0);
    let mut last_head_location = Coord(0, 0);
    let mut tail_location = Coord(0, 0);
    tail_seen_locations.insert(tail_location);

    for direction in input {
        let mut parts = direction.split_whitespace();
        let direction = parts.next().unwrap();
        let distance: usize = parts.next().unwrap().parse().unwrap();

        for _ in 0..distance {
            head_location = match direction {
                "R" => Coord(head_location.0 + 1, head_location.1),
                "L" => Coord(head_location.0 - 1, head_location.1),
                "U" => Coord(head_location.0, head_location.1 + 1),
                "D" => Coord(head_location.0, head_location.1 - 1),
                _ => head_location,
            };

            if (tail_location.0 - head_location.0).abs() > 1
                || (tail_location.1 - head_location.1).abs() > 1
            {
                // tail catch up
                tail_location = last_head_location;
                tail_seen_locations.insert(tail_location);
            }

            last_head_location = head_location;
        }
    }

    tail_seen_locations.iter().count()
}

pub fn second(input: &[String]) -> usize {
    let mut tail_seen_locations: HashSet<Coord> = HashSet::new();

    let mut locations: Vec<Coord> = vec![Coord(0, 0); 10];

    for direction in input {
        let mut parts = direction.split_whitespace();
        let direction = parts.next().unwrap();
        let distance: usize = parts.next().unwrap().parse().unwrap();

        for _ in 0..distance {
            let head_location = locations[0];
            let head_location = match direction {
                "R" => Coord(head_location.0 + 1, head_location.1),
                "L" => Coord(head_location.0 - 1, head_location.1),
                "U" => Coord(head_location.0, head_location.1 - 1),
                "D" => Coord(head_location.0, head_location.1 + 1),
                _ => head_location,
            };
            locations[0] = head_location;

            // now all the tails follow
            for i in 1..locations.len() {
                let head_location = locations[i - 1];
                let tail_location = locations[i];

                let x_diff = tail_location.0 - head_location.0;
                let y_diff = tail_location.1 - head_location.1;

                if x_diff.abs() > 1 && y_diff == 0 {
                    // two away horizontal - catch up
                    locations[i] = Coord(locations[i].0 - (x_diff / 2), locations[i].1);
                } else if y_diff.abs() > 1 && x_diff == 0 {
                    // two away vertical - catch up
                    locations[i] = Coord(locations[i].0, locations[i].1 - (y_diff / 2));
                } else if x_diff.abs() > 1 || y_diff.abs() > 1 {
                    // diagonal movement required
                    let x_neg = x_diff.is_negative();
                    let y_neg = y_diff.is_negative();

                    locations[i] = match (x_neg, y_neg) {
                        (true, false) => Coord(locations[i].0 + 1, locations[i].1 - 1),
                        (true, true) => Coord(locations[i].0 + 1, locations[i].1 + 1),
                        (false, false) => Coord(locations[i].0 - 1, locations[i].1 - 1),
                        (false, true) => Coord(locations[i].0 - 1, locations[i].1 + 1),
                    };
                }
                if i == 9 {
                    tail_seen_locations.insert(locations[i]);
                }
            }
        }
    }

    tail_seen_locations.iter().count()
}

fn print_locations(locations: &Vec<Coord>) {
    println!();
    for y in -10..10 {
        for x in -5..15 {
            // print!("{}", x);
            let mut found = false;
            for (i, coords) in locations.iter().enumerate().rev() {
                if coords.0 == x && coords.1 == y {
                    print!("{:?}", i);
                    found = true;
                    break;
                }
            }

            if !found {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec!["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    fn example2() -> Vec<String> {
        let input: Vec<&str> = vec!["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, 13);
    }

    #[test]
    fn second_test() {
        let input = example2();
        let result = second(&input);
        assert_eq!(result, 36);
    }
}
