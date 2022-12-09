pub fn first(input: &[String]) -> usize {
    let compartments = compartments_from_input(input);
    let matching_prioritites = compartments.iter().filter_map(|(first, second)| {
        (1..=52).find(|priority| first.count(*priority) > 0 && second.count(*priority) > 0)
    });
    matching_prioritites.reduce(|lhs, rhs| lhs + rhs).unwrap()
}

pub fn second(input: &[String]) -> usize {
    let compartments = compartments_from_input(input);
    let matching_priorities = compartments.chunks(3).filter_map(|group| {
        (1..=52).find(|priority| {
            group.iter().fold(true, |accum, (first_half, second_half)| {
                accum && (first_half.count(*priority) > 0 || second_half.count(*priority) > 0)
            })
        })
    });
    matching_priorities.reduce(|lhs, rhs| lhs + rhs).unwrap()
}

fn compartments_from_input(input: &[String]) -> Vec<(Compartment, Compartment)> {
    input
        .iter()
        .filter_map(|line| {
            let len = line.len();
            if (len < 2) || ((len % 2) != 0) {
                return None;
            }

            let part1: String = line.chars().take(len / 2).collect();
            let compartment1 = Compartment::from(&part1);

            let part2: String = line.chars().skip(len / 2).collect();
            let compartment2 = Compartment::from(&part2);

            Some((compartment1, compartment2))
        })
        .collect()
}

#[derive(Debug, PartialEq, Hash, Clone, Eq)]
struct Compartment {
    items: Vec<usize>,
}

fn ascii_to_priority(ascii: char) -> usize {
    let ascii = (ascii as usize) - 64; // A to Z to 1 - 26
    if ascii < 27 {
        // A-Z need to be moved to 27-52
        ascii + 26
    } else {
        // a-z need to moved down to 1-27
        ascii - 32 // A-Z + 6 additional characters
    }
}

impl Compartment {
    fn new() -> Compartment {
        Compartment { items: vec![0; 52] }
    }

    fn from(input: &String) -> Compartment {
        let mut compartment = Compartment::new();
        input
            .chars()
            .for_each(|c| compartment.add(ascii_to_priority(c)));
        compartment
    }

    fn add(&mut self, priority: usize) {
        self.items[priority - 1] += 1;
    }

    fn count(&self, priority: usize) -> usize {
        self.items[priority - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn test_compartments_from_input() {
        let compartments = compartments_from_input(&example());
        assert_eq!(compartments[0].0.count(ascii_to_priority('J')), 2);
        assert_eq!(compartments[1].0.count(ascii_to_priority('H')), 1);
    }

    #[test]
    fn test_string_to_compartment() {
        let compartment = Compartment::from(&String::from("vJrwpWtwJgWr"));
        println!("compartment {:?}", compartment);
        assert_eq!(compartment.count(ascii_to_priority('J')), 2);
        assert_eq!(compartment.count(ascii_to_priority('z')), 0);
        assert_eq!(compartment.count(ascii_to_priority('W')), 2);
        assert_eq!(compartment.count(ascii_to_priority('v')), 1);
    }

    #[test]
    fn test_ascii_to_priority() {
        assert_eq!(ascii_to_priority('a'), 1);
        assert_eq!(ascii_to_priority('b'), 2);
        assert_eq!(ascii_to_priority('z'), 26);
        assert_eq!(ascii_to_priority('A'), 27);
        assert_eq!(ascii_to_priority('B'), 28);
        assert_eq!(ascii_to_priority('Z'), 52);

        assert_eq!(ascii_to_priority('p'), 16);
        assert_eq!(ascii_to_priority('L'), 38);
        assert_eq!(ascii_to_priority('P'), 42);
        assert_eq!(ascii_to_priority('v'), 22);
        assert_eq!(ascii_to_priority('t'), 20);
        assert_eq!(ascii_to_priority('s'), 19);
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, 157);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 70);
    }
}
