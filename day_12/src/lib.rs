use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn first(input: &[String]) -> usize {
    let (start, end, grid) = parse_input(&input);
    djikstras(
        start,
        &grid,
        |lhs, rhs| lhs <= (rhs + 1),
        |potential_end| potential_end == end,
    )
}

pub fn second(input: &[String]) -> usize {
    let (_, end, grid) = parse_input(&input);
    // difference with the first is that the neighbor steps go in the other direction,
    // and we're at "the end" at any node that has 0 height
    // djikstras is always traversing the shortest path, so the first 'end' we find will be the minimum one
    djikstras(
        end,
        &grid,
        |neighbor_height, my_height| (neighbor_height + 1) >= my_height,
        |potential_end| grid[potential_end.0][potential_end.1] == 0,
    )
}

fn djikstras<P, Q>(start: Coord, grid: &Vec<Vec<u8>>, mut filter: P, mut end_cmp: Q) -> usize
where
    P: FnMut(u8, u8) -> bool,
    Q: FnMut(Coord) -> bool,
{
    // Djikstra's! https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
    let mut distances: HashMap<Coord, usize> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, _)| (Coord(y, x), usize::MAX))
        })
        .collect();
    distances.insert(start, 0);

    let mut unvisited = BinaryHeap::new();
    unvisited.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = unvisited.pop() {
        // if we happen to be at the end we're done
        if end_cmp(position) {
            return cost;
        }

        // we've gone down a bad path
        if cost > *distances.get(&position).unwrap() {
            continue;
        }

        // for each neighbor, see if we can find a way with a lower cost going through this node
        for neighbor in neighbors(position, &grid, &mut filter) {
            let next = State {
                cost: cost + 1,
                position: neighbor,
            };

            // if we have a cheaper way to get there from here
            if next.cost < *distances.get(&neighbor).unwrap() {
                unvisited.push(next);
                distances.insert(neighbor, next.cost);
            }
        }
    }

    // not reachable
    panic!("OH NO")
}

fn neighbors<P>(position: Coord, grid: &Vec<Vec<u8>>, mut filter: P) -> Vec<Coord>
where
    P: FnMut(u8, u8) -> bool,
{
    let max_y = grid.len() - 1;
    let max_x = grid[0].len() - 1;
    let current_height = grid[position.0][position.1];

    let mut candidate_neighbors: Vec<Coord> = vec![];
    if position.0 > 0 {
        candidate_neighbors.push(Coord(position.0 - 1, position.1))
    } // U
    if position.1 > 0 {
        candidate_neighbors.push(Coord(position.0, position.1 - 1))
    } // L
    if position.0 < max_y {
        candidate_neighbors.push(Coord(position.0 + 1, position.1))
    } // R
    if position.1 < max_x {
        candidate_neighbors.push(Coord(position.0, position.1 + 1))
    } // D

    candidate_neighbors
        .iter()
        .filter(|position| filter(grid[position.0][position.1], current_height))
        .map(|position| position.clone())
        .collect()
}

// from https://doc.rust-lang.org/std/collections/binary_heap/index.html
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Coord,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.0.cmp(&other.position.0))
            .then_with(|| self.position.1.cmp(&other.position.1))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
// end from https://doc.rust-lang.org/std/collections/binary_heap/index.html

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Coord(usize, usize);

fn parse_input(input: &[String]) -> (Coord, Coord, Vec<Vec<u8>>) {
    let mut grid = vec![vec![]; input.len()];
    let mut start = Coord(0, 0);
    let mut end = Coord(0, 0);
    for (y, row) in input.iter().enumerate() {
        for (x, item) in row.chars().enumerate() {
            match item {
                'S' => {
                    start = Coord(y, x);
                    grid[y].push(0);
                }
                'E' => {
                    end = Coord(y, x);
                    grid[y].push(25);
                }
                _ => {
                    grid[y].push((item as u8) - 97);
                }
            }
        }
    }
    (start, end, grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec!["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn test_parse_input() {
        let input = example();
        let (start, end, parsed) = parse_input(&input);

        assert_eq!(start, Coord(0, 0));
        assert_eq!(end, Coord(2, 5));
        assert_eq!(parsed[0][0], 0);
        assert_eq!(parsed[2][5], 25);
        assert_eq!(parsed[4][7], 8);
        assert_eq!(parsed[4][0], 0);
    }

    #[test]
    fn test_neighbors() {
        let input = example();
        let (start, end, grid) = parse_input(&input);

        let neighbors1 = neighbors(start, &grid, |lhs, rhs| lhs <= (rhs + 1));
        assert_eq!(neighbors1[0], Coord(1, 0));
        assert_eq!(neighbors1[1], Coord(0, 1));

        // U L R D
        let neighbors2 = neighbors(Coord(2, 2), &grid, |lhs, rhs| lhs <= (rhs + 1));
        assert_eq!(neighbors2[0], Coord(1, 2));
        assert_eq!(neighbors2[1], Coord(2, 1));
        assert_eq!(neighbors2[2], Coord(3, 2));
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, 31);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 29);
    }
}
