use std::collections::HashSet;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
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

            if (tail_location.0 - head_location.0 as i32).abs() > 1
                || (tail_location.1 - head_location.1 as i32).abs() > 1
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
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec!["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];
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
        let input = example();
        let result = second(&input);
        assert_eq!(result, 0);
    }
}
