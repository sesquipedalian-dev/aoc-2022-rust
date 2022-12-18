use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn first(input: &[String]) -> usize {
    // what are the sides of a 1x1x1 voxel at x,y,z

    // what are the lines of a 1x1 square at x, y

    // x, y -> x +1,y -> x+1, y+1 -> x, y+1 -> x, y

    // then the sides of a voxel are a set of squares
    // x,y,z -> x +  1, y, z -> x+1, y+1, z -> x, y+1, z -> x, y, z

    //   __
    //  /  /|
    // /__/ |
    // |  | /
    // |  |/
    // ___

    // x+1, y +1, z -> x+1, y +1, z +1 -> x, y +1, z+1 -> x, y+1, z ->
    //

    // so for each input coord
    //   for each face
    //      calculate the set of coords making up that face
    //      add them to a hash map, counting how many times we've seen a face
    //   end
    // end
    //

    let mut facet_counts: HashMap<BTreeSet<(usize, usize, usize)>, usize> = HashMap::new();
    for line in input.iter() {
        let coords: Vec<usize> = line.split(",").map(|p| p.parse().unwrap()).collect();
        insert_facets(&mut facet_counts, &coords[0], &coords[1], &coords[2]);
    }

    // count up all the hash entries that we've seen exactly once
    // if it's some other number two cubes shared the same face
    facet_counts.iter().fold(0, |accum, (k, next)| {
        println!("facet count? {:?} {}", k, next);
        if *next == 1 {
            accum + 1
        } else {
            accum
        }
    })
}

fn insert_facets(
    facets: &mut HashMap<BTreeSet<(usize, usize, usize)>, usize>,
    x: &usize,
    y: &usize,
    z: &usize,
) {
    let diffs: Vec<Vec<(usize, usize, usize)>> = vec![
        // too lazy to automate this
        vec![(0, 0, 0), (1, 0, 0), (1, 1, 0), (0, 1, 0)], // front
        vec![(1, 1, 0), (1, 1, 1), (0, 1, 1), (0, 1, 0)], // top
        vec![(1, 0, 0), (1, 1, 0), (1, 1, 1), (1, 0, 1)], // right
        vec![(0, 0, 0), (1, 0, 0), (0, 0, 1), (1, 0, 1)], // bottom
        vec![(0, 0, 0), (0, 0, 1), (0, 1, 1), (0, 1, 0)], // left
        vec![(0, 0, 1), (1, 0, 1), (1, 1, 1), (0, 1, 1)], // back
    ];
    diffs.iter().for_each(|diffs| {
        let mut set = BTreeSet::new();
        diffs.iter().for_each(|diff| {
            set.insert((*x + diff.0, *y + diff.1, *z + diff.2));
        });
        *(facets.entry(set).or_insert(0)) += 1;
    });
}

pub fn second(input: &[String]) -> usize {
    // so internal voids that detract from the surface area would have to be wholy contained
    // within the shape, otherwise they would have a path to the outside and not be interior
    //
    // so if we had something that notated the voxels that form the surface of the shape
    // we could search all possible coords inside that surface to see if they are part of the set
    // if not their sides should be subtracted from the exterior surface area.
    //
    // one thought is that interior voids would share all 6 sides with some voxel in the shape.
    // but that ignores the case that multiple voxels combine into a single internal void
    //
    // maybe something like ray tracing? for each possible internal void, send a ray in all 6 directions
    // if all of them eventually hit a voxel in the shape (stop when the coord is > than the max coords of the shape),
    // then it is an internal void.  And its sides should that intersect with the voxels should be subtracted from the surface area.
    //
    // building on that idea, we could 'grow' potential internal voids by increasing its size until we hit
    // 'air' (outside the max bounds of the shape) or a voxel.  If the growing internal shape never hits air
    // then its faces that intersect with the shape would be the interior surface area. 
    //

    let mut facet_counts: HashMap<BTreeSet<(usize, usize, usize)>, usize> = HashMap::new();
    let mut voxels_in_shape: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut shape_mins = (1000, 1000, 1000);
    let mut shape_maxes = (0, 0, 0);

    for line in input.iter() {
        let coords: Vec<usize> = line.split(",").map(|p| p.parse().unwrap()).collect();
        // shift to avoid underflow
        let x = coords[0] + 1;
        let y = coords[1] + 1;
        let z = coords[2] + 1;
        voxels_in_shape.insert((x, y, z));

        // running total of shape boundaries
        shape_mins = (
            shape_mins.0.min(x),
            shape_mins.1.min(y),
            shape_mins.2.min(z),
        );
        shape_maxes = (
            shape_maxes.0.max(x),
            shape_maxes.1.max(y),
            shape_maxes.2.max(z),
        );

        let diffs: Vec<Vec<(usize, usize, usize)>> = vec![
            // too lazy to automate this
            vec![(0, 0, 0), (1, 0, 0), (1, 1, 0), (0, 1, 0)], // front
            vec![(1, 1, 0), (1, 1, 1), (0, 1, 1), (0, 1, 0)], // top
            vec![(1, 0, 0), (1, 1, 0), (1, 1, 1), (1, 0, 1)], // right
            vec![(0, 0, 0), (1, 0, 0), (0, 0, 1), (1, 0, 1)], // bottom
            vec![(0, 0, 0), (0, 0, 1), (0, 1, 1), (0, 1, 0)], // left
            vec![(0, 0, 1), (1, 0, 1), (1, 1, 1), (0, 1, 1)], // back
        ];
        diffs.iter().for_each(|diffs| {
            let mut set = BTreeSet::new();
            diffs.iter().for_each(|diff| {
                set.insert((x + diff.0, y + diff.1, z + diff.2));
            });
            *(facet_counts.entry(set).or_insert(0)) += 1;
        });
    }

    // search for internal voids
    let mut internal_void_visited: HashSet<(usize, usize, usize)> = HashSet::new();
    for x in shape_mins.0..=shape_maxes.0 {
        for y in shape_mins.1..=shape_maxes.1 {
            for z in shape_mins.2..=shape_maxes.2 {
                if internal_void_visited.contains(&(x, y, z)) {
                    continue;
                }

                // BFS for air
                let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();
                let mut unvisited: VecDeque<(usize, usize, usize)> = VecDeque::new();
                unvisited.push_back((x, y, z));

                let mut found_air = false;
                while let Some((x, y, z)) = unvisited.pop_front() {
                    if x < shape_mins.0
                        || y < shape_mins.1
                        || z < shape_mins.2
                        || x > shape_maxes.0
                        || y > shape_maxes.1
                        || z > shape_maxes.2
                    {
                        found_air = true;
                        break;
                    }

                    if voxels_in_shape.contains(&(x, y, z)) {
                        continue;
                    }

                    if visited.contains(&(x, y, z)) {
                        continue;
                    }

                    visited.insert((x, y, z));
                    internal_void_visited.insert((x, y, z));

                    // neighbors - fortunately it looks like all the coords are minimum 1 so we shouldn't stray < 1 on any coords
                    unvisited.push_back((x + 1, y, z));
                    unvisited.push_back((x - 1, y, z));
                    unvisited.push_back((x, y + 1, z));
                    unvisited.push_back((x, y - 1, z));
                    unvisited.push_back((x, y, z + 1));
                    unvisited.push_back((x, y, z - 1));
                }

                if !found_air {
                    visited
                        .iter()
                        .for_each(|(x, y, z)| insert_facets(&mut facet_counts, x, y, z));
                }
            }
        }
    }

    // count up all the hash entries that we've seen exactly once
    // if it's some other number two cubes shared the same face
    facet_counts.iter().fold(
        0,
        |accum, (k, next)| {
            if *next == 1 {
                accum + 1
            } else {
                accum
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
            "2,2,2", "1,2,2", "3,2,2", "2,1,2", "2,3,2", "2,2,1", "2,2,3", "2,2,4", "2,2,6",
            "1,2,5", "3,2,5", "2,1,5", "2,3,5",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    fn example2() -> Vec<String> {
        let input: Vec<&str> = vec!["1,1,1", "2,1,1"];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }


    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, 64);

        let result = first(&example2());
        assert_eq!(result, 10);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 58);
    }
}
