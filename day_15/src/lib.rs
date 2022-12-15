pub fn first(input: &[String], target_y: isize) -> usize {
    // for each sensor, calculate its sensing distance
    let sensors = parse_input(&input);

    // determine its footprint on the y = whatever row
    //   for a sensor at x, y, it's footprint is centered at x, whatever on the target row and the width is max(0, 1 + (range - (whatever - y).abs()) * 2)
    let mut ranges: Vec<(isize, isize)> = sensors
        .iter()
        .flat_map(|sensor| {
            let width_on_y =
                (sensor.sensing_distance as isize) - (target_y - sensor.location.y).abs();
            if width_on_y < 0 {
                None
            } else {
                Some((
                    sensor.location.x - width_on_y,
                    sensor.location.x + width_on_y,
                ))
            }
        })
        .collect();

    // sort ranges by min x
    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    // combine overlapping ranges like in the day 4 puzzle
    let first_range = ranges.remove(0);
    let mut combined_ranges: Vec<(isize, isize)> = vec![first_range];
    for next_range in ranges {
        let mut current_range = combined_ranges.pop().unwrap();
        if (current_range.0 <= next_range.0 && current_range.1 >= next_range.0)
            || (current_range.0 <= next_range.1 && current_range.1 >= next_range.0)
            || (current_range.1 + 1 == next_range.0)
        {
            // overlap
            current_range.0 = current_range.0.min(next_range.0);
            current_range.1 = current_range.1.max(next_range.1);
            combined_ranges.push(current_range);
        } else {
            // new range
            combined_ranges.push(current_range);
            combined_ranges.push(next_range);
        }
    }

    // sum the width of the ranges
    combined_ranges
        .iter()
        .fold(0isize, |accum, next| accum + (next.1 - next.0)) as usize
}

pub fn second(input: &[String], max_x_y: isize) -> usize {
    // let max_x_y = 4_000_000;

    // for each sensor, calculate its sensing distance
    let sensors = parse_input(&input);

    // iterate row from 0 4_000_000
    let mut found: Option<(isize, isize)> = None;
    'target_y: for target_y in 0..=max_x_y {
        // determine its footprint on the y = target row
        // same as part 1 but clamp ranges to 0 - 4_000_000
        let mut ranges: Vec<(isize, isize)> = sensors
            .iter()
            .flat_map(|sensor| {
                let width_on_y =
                    (sensor.sensing_distance as isize) - (target_y - sensor.location.y).abs();
                if width_on_y < 0 {
                    None
                } else {
                    let min_x = (sensor.location.x - width_on_y).max(0);
                    let max_x = (sensor.location.x + width_on_y).min(max_x_y);

                    if (max_x - min_x) < 1 {
                        None
                    } else {
                        Some((min_x, max_x))
                    }
                }
            })
            .collect();

        // sort ranges by min x
        ranges.sort_by(|a, b| a.0.cmp(&b.0));

        if ranges.is_empty() {
            continue;
        }

        // combine overlapping ranges like in the day 4 puzzle
        // if we find any range that is NOT overlapping, the beacon is in between the current range and the start of the next range
        let first_range = ranges.remove(0);
        let mut combined_ranges: Vec<(isize, isize)> = vec![first_range];
        for next_range in ranges {
            let mut current_range = combined_ranges.pop().unwrap();
            if (current_range.0 <= next_range.0 && current_range.1 >= next_range.0)
                || (current_range.0 <= next_range.1 && current_range.1 >= next_range.0)
                || (current_range.1 + 1 == next_range.0)
            {
                // overlap
                current_range.0 = current_range.0.min(next_range.0);
                current_range.1 = current_range.1.max(next_range.1);
                combined_ranges.push(current_range);
            } else {
                // non-overlapping range - since the problem statement says there's only one spot,
                // the gap between the ranges must only be 1 and so the spot is after the current_range
                // println!("found non-overlapping range {:?} {:?} {}", current_range, next_range, target_y);
                found = Some((current_range.1 + 1, target_y));
                break 'target_y;
            }
        }
    }

    found
        .map(|(x, y)| ((x * 4_000_000) + y) as usize)
        .unwrap_or_default()
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Coord {
    x: isize,
    y: isize,
}

fn manhattan(a: &Coord, b: &Coord) -> usize {
    ((a.x - b.x).abs() as usize) + ((a.y - b.y).abs() as usize)
}

struct Sensor {
    location: Coord,
    sensing_distance: usize,
}

impl Sensor {
    fn from(input: &String) -> Sensor {
        // for example: Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let mut parts = input.split_whitespace();
        let mut digits = parts.map(|p| p.split("=")).filter_map(|mut split| {
            split
                .next()
                .and_then(|_| split.next())
                .and_then(|digit_str| {
                    let strip_non_digits: String = digit_str
                        .chars()
                        .filter(|c| ('0' <= *c && *c <= '9') || *c == '-')
                        .collect();
                    strip_non_digits.parse::<isize>().ok()
                })
        });
        let location = Coord {
            x: digits.next().unwrap(),
            y: digits.next().unwrap(),
        };
        let nearest_beacon = Coord {
            x: digits.next().unwrap(),
            y: digits.next().unwrap(),
        };
        let sensing_distance = manhattan(&location, &nearest_beacon);
        Sensor {
            location,
            sensing_distance,
        }
    }
}

fn parse_input(input: &[String]) -> Vec<Sensor> {
    input.iter().map(|s| Sensor::from(s)).collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
            "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
            "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
            "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
            "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
            "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
            "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
            "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
            "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
            "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
            "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
            "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
            "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
            "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn test_manhattan() {
        assert_eq!(
            manhattan(&Coord { x: 9, y: 16 }, &Coord { x: 10, y: 16 }),
            1
        );
        assert_eq!(manhattan(&Coord { x: 8, y: 7 }, &Coord { x: 2, y: 10 }), 9);
    }

    #[test]
    fn test_parse_input() {
        let input = example();
        let parsed = parse_input(&input);

        assert_eq!(parsed[0].location, Coord { x: 2, y: 18 });
        assert_eq!(parsed[0].sensing_distance, 7);

        assert_eq!(parsed[6].location, Coord { x: 8, y: 7 });
        assert_eq!(parsed[6].sensing_distance, 9);
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input, 10);
        assert_eq!(result, 26);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input, 20);
        assert_eq!(result, 56000011);
    }
}
