use std::collections::HashMap;
use std::collections::BTreeSet;

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
        // let mut set = BTreeSet::new();
        let diffs: Vec<Vec<(usize, usize, usize)>> = vec!( // too lazy to automate this
            vec!((0, 0, 0), (1, 0, 0), (1, 1, 0), (0, 1, 0)), // front
            vec!((1, 1, 0), (1, 1, 1), (0, 1, 1), (0, 1, 0)), // top
            vec!((1, 0, 0), (1, 1, 0), (1, 1, 1), (1, 0, 1)), // right
            vec!((0, 0, 0), (1, 0, 0), (0, 0, 1), (1, 0, 1)), // bottom
            vec!((0, 0, 0), (0, 0, 1), (0, 1, 1), (0, 1, 0)), // left
            vec!((0, 0, 1), (1, 0, 1), (1, 1, 1), (0, 1, 1)), // back
        );
        diffs.iter().for_each(|diffs| {
            let mut set = BTreeSet::new();
            diffs.iter().for_each(|diff| {
                set.insert((coords[0] + diff.0, coords[1] + diff.1, coords[2] + diff.2));
            });
            *(facet_counts.entry(set).or_insert(0)) += 1;
        });
  
    }

    // count up all the hash entries that we've seen exactly once
    // if it's some other number two cubes shared the same face
    facet_counts.iter().fold(0, |accum, (k , next)| {
        println!("facet count? {:?} {}", k, next);
        if *next == 1 {
            accum + 1
        } else {
            accum
        }
    })
}

pub fn second(input: &[String]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
            "2,2,2",
            "1,2,2",
            "3,2,2",
            "2,1,2",
            "2,3,2",
            "2,2,1",
            "2,2,3",
            "2,2,4",
            "2,2,6",
            "1,2,5",
            "3,2,5",
            "2,1,5",
            "2,3,5",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    fn example2() -> Vec<String> {
        let input: Vec<&str> = vec![
            "1,1,1",
            "2,1,1",
        ];
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
        assert_eq!(result, 0);
    }
}
