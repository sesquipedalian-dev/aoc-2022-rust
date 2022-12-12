pub fn first(input: &[String]) -> usize {
    let (mut count, mut grid) = parse_input(input);
    let max_y = grid.len() - 1;
    let max_x = grid[max_y].len() - 1;

    // left to right
    for y in 1..max_y {
        let mut last_seen = grid[y][0].height;
        for x in 1..max_x {
            match grid[y][x].height.cmp(&last_seen) {
                std::cmp::Ordering::Less => continue,
                std::cmp::Ordering::Greater if !grid[y][x].counted => {
                    grid[y][x].counted = true;
                    count += 1;
                }
                _ => (),
            }
            last_seen = grid[y][x].height;
        }
    }

    // right to left
    for y in 1..max_y {
        let mut last_seen = grid[y][max_x].height;
        for x in (1..max_x).rev() {
            match grid[y][x].height.cmp(&last_seen) {
                std::cmp::Ordering::Less => continue,
                std::cmp::Ordering::Greater if !grid[y][x].counted => {
                    grid[y][x].counted = true;
                    count += 1;
                }
                _ => (),
            }
            last_seen = grid[y][x].height;
        }
    }

    // top to bottom
    for x in 1..max_x {
        let mut last_seen = grid[0][x].height;
        for y in 1..max_y {
            match grid[y][x].height.cmp(&last_seen) {
                std::cmp::Ordering::Less => continue,
                std::cmp::Ordering::Greater if !grid[y][x].counted => {
                    grid[y][x].counted = true;
                    count += 1;
                }
                _ => (),
            }
            last_seen = grid[y][x].height;
        }
    }

    // bottom to top
    for x in 1..max_x {
        let mut last_seen = grid[max_y][x].height;
        for y in (1..max_y).rev() {
            match grid[y][x].height.cmp(&last_seen) {
                std::cmp::Ordering::Less => continue,
                std::cmp::Ordering::Greater if !grid[y][x].counted => {
                    grid[y][x].counted = true;
                    count += 1;
                }
                _ => (),
            }
            last_seen = grid[y][x].height;
        }
    }

    count
}

pub fn second(input: &[String]) -> usize {
    let (_, mut grid) = parse_input(input);
    let max_i = grid.len() - 1;
    let mut max_scenic_score = 0;

    // brute force it

    for (y, row) in grid.iter().enumerate() {
        if y == 0 || y == max_i {
            continue; // all edges have a score of 0
        }

        for (x, item) in row.iter().enumerate() {
            if x == 0 || x == max_i {
                continue; // all edges have a score of 0
            }

            // count left
            let left_count = (0..x)
                .rev()
                .find(|cmp_x| grid[y][*cmp_x].height >= item.height)
                .map(|cmp_x| x - cmp_x)
                .unwrap_or(x);
            // count right
            let right_count = ((x + 1)..=max_i)
                .find(|cmp_x| grid[y][*cmp_x].height >= item.height)
                .map(|cmp_x| cmp_x - x)
                .unwrap_or(max_i - x);
            // count down
            let down_count = ((y + 1)..=max_i)
                .find(|cmp_y| grid[*cmp_y][x].height >= item.height)
                .map(|cmp_y| cmp_y - y)
                .unwrap_or(max_i - y);
            // count up
            let up_count = (0..y)
                .rev()
                .find(|cmp_y| grid[*cmp_y][x].height >= item.height)
                .map(|cmp_y| y - cmp_y)
                .unwrap_or(y);

            // multiply together
            let scenic_score = left_count * right_count * down_count * up_count;

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score
            }
        }
    }

    max_scenic_score
}

fn parse_input(input: &[String]) -> (usize, Vec<Vec<GridItem>>) {
    let max_x_y = input.len() - 1;
    let mut seen_count = 0;
    let mut result = vec![];
    for (y, line) in input.iter().enumerate() {
        let mut new_row = vec![];
        for (x, c) in line.chars().enumerate() {
            let new_item = GridItem {
                height: (c as i8 - 48) as u8,
                counted: ((x == 0) || (x == max_x_y)) || ((y == 0) || (y == max_x_y)),
            };
            if new_item.counted {
                seen_count += 1;
            }
            new_row.push(new_item);
        }
        result.push(new_row);
    }

    (seen_count, result)
}

#[derive(Debug, PartialEq, Hash, Clone, Eq)]
struct GridItem {
    height: u8,
    counted: bool,
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
                counted: true,
            },
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
        assert_eq!(result, 8);
    }
}
