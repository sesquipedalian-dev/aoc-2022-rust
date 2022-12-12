pub fn first(input: &[String]) -> usize {
    let (mut count, mut grid) = parse_input(input);
    let max_y = grid.len() - 1;
    let max_x = grid[max_y].len() - 1;

    // left to right, right to left
    // up to down, down to up
    // skip 1st
    // stop when already seen
    // stop when an item is no longer visible
    // or end of row

    // left to right
    for y in 1..max_y {
        let mut last_seen = grid[y][0].height;
        for x in 1..max_x {
            if grid[y][x].x_seen {
                break;
            }
            grid[y][x].x_seen = true;

            match grid[y][x].height.cmp(&last_seen) {
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Less => break,
                std::cmp::Ordering::Greater => count += 1,
            }
            last_seen = grid[y][x].height;
        }
    }

    // right to left
    for y in 1..max_y {
        let mut last_seen = grid[y][max_x].height;
        for x in (1..max_x).rev() {
            if grid[y][x].x_seen {
                break;
            }
            grid[y][x].x_seen = true;

            match grid[y][x].height.cmp(&last_seen) {
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Less => break,
                std::cmp::Ordering::Greater => count += 1,
            }
            last_seen = grid[y][x].height;
        }
    }

    // top to bottom
    for x in 1..max_x {
        let mut last_seen = grid[0][x].height;
        for y in 1..max_y {
            if grid[y][x].y_seen {
                break;
            }
            grid[y][x].y_seen = true;

            match grid[y][x].height.cmp(&last_seen) {
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Less => break,
                std::cmp::Ordering::Greater => count += 1,
            }
            last_seen = grid[y][x].height;
        }
    }

    // bottom to top
    for x in 1..max_x {
        let mut last_seen = grid[max_y][x].height;
        for y in (1..max_y).rev() {
            if grid[y][x].x_seen {
                break;
            }
            grid[y][x].x_seen = true;

            match grid[y][x].height.cmp(&last_seen) {
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Less => break,
                std::cmp::Ordering::Greater => count += 1,
            }
            last_seen = grid[y][x].height;
        }
    }

    count
}

pub fn second(input: &[String]) -> usize {
    0
}

fn parse_input(input: &[String]) -> (usize, Vec<Vec<GridItem>>) {
    let mut seen_count = 0;
    let mut result = vec![];
    for (y, line) in input.iter().enumerate() {
        let mut new_row = vec![];
        for (x, c) in line.chars().enumerate() {
            let new_item = GridItem {
                height: (c as i8 - 48) as u8,
                x_seen: x == 0,
                y_seen: y == 0,
            };
            if new_item.x_seen || new_item.y_seen {
                seen_count += 1;
            }
            new_row.push(new_item);
        }
        result.push(new_row);
    }

    let max_x_y = result.len();
    for y in 1..max_x_y {
        result[y][max_x_y - 1].x_seen = true;
        result[y][max_x_y - 1].y_seen = true;
        seen_count += 1;
    }
    for x in 1..(max_x_y - 1) {
        result[max_x_y - 1][x].x_seen = true;
        result[max_x_y - 1][x].y_seen = true;
        seen_count += 1;
    }

    (seen_count, result)
}

#[derive(Debug, PartialEq, Hash, Clone, Eq)]
struct GridItem {
    height: u8,
    x_seen: bool,
    y_seen: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec!["30373", "25512", "65332", "33549", "35390"];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn test_parse_input() {
        let input = example();
        let (count, parsed) = parse_input(&input);
        assert_eq!(count, 16);
        assert_eq!(parsed[0][0].height, 3);
        assert_eq!(parsed[2][2].height, 3);
        assert_eq!(parsed[4][4].height, 0);
        assert_eq!(
            parsed[3][4],
            GridItem {
                height: 9,
                x_seen: true,
                y_seen: true
            }
        );
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, 21);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 0);
    }
}
