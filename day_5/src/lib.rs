pub fn first(input: &[String]) -> String {
    let mut input_iter = input.iter();
    let mut stacks = CargoStacks::from(&mut input_iter);
    let moves = Move::from(&mut input_iter);

    moves.iter().for_each(|m| {
        (0..m.count).for_each(|i| {
            let moved = stacks.stacks[m.from - 1].pop().unwrap();
            stacks.stacks[m.to - 1].push(moved);
        })
    });

    stacks
        .stacks
        .iter_mut()
        .filter_map(|stack| stack.pop())
        .collect()
}

pub fn second(input: &[String]) -> usize {
    0
}

#[derive(Debug, PartialEq, Hash, Clone, Eq)]
struct Move {
    from: usize,
    to: usize,
    count: usize,
}

impl Move {
    fn from<'a, T>(input: &mut T) -> Vec<Move>
    where
        T: Iterator<Item = &'a String>,
    {
        input
            .map(|line| {
                // e.g. move 1 from 2 to 1
                let line_parts: Vec<usize> = line
                    .split_whitespace()
                    .filter_map(|part| part.parse::<usize>().ok())
                    .collect();
                Move {
                    count: line_parts[0],
                    from: line_parts[1],
                    to: line_parts[2],
                }
            })
            .collect()
    }
}

struct CargoStacks {
    stacks: Vec<Vec<char>>,
}

impl CargoStacks {
    /// parse the initial stack state until a blank line is hit; mutates the iterator so
    /// the move parser can go next
    fn from<'a, T>(input: &mut T) -> CargoStacks
    where
        T: Iterator<Item = &'a String>,
    {
        let mut stacks = CargoStacks { stacks: vec![] };

        while let Some(line) = input.next() {
            if line.len() < 1 {
                break;
            }

            // initialize stack size based on the line length
            if stacks.stacks.len() == 0 {
                (0..line.len() / 4).for_each(|_| stacks.stacks.push(vec![]));
            }

            line.chars()
                .zip(0..line.len()) // pair chars with index
                .filter(|(_, index)| index % 4 == 1) // each stack repr is "[X] ", the 2nd character is the char
                .for_each(|(item, index)| {
                    match item {
                        item if '1' <= item && item <= '9' => (),
                        item if item == ' ' => (),
                        _ => stacks.stacks[index / 4].insert(0, item), // if it's not the stack indices then it's to be inserted
                    }
                });
        }

        stacks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
            "    [D]     ",
            "[N] [C]     ",
            "[Z] [M] [P] ",
            " 1   2   3  ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn test_moves_from() {
        let input: Vec<String> = example();
        let iter = input.iter();
        let mut iter = iter.skip_while(|line| !line.starts_with("move"));
        let moves = Move::from(&mut iter);
        assert_eq!(
            moves[0],
            Move {
                from: 2,
                to: 1,
                count: 1
            }
        );
        assert_eq!(
            moves[1],
            Move {
                from: 1,
                to: 3,
                count: 3
            }
        );
    }

    #[test]
    fn test_cargo_stacks_from() {
        let input = example();
        let mut iter = input.iter();
        let stacks = CargoStacks::from(&mut iter);
        assert_eq!(iter.next(), Some(&String::from("move 1 from 2 to 1")));
        assert_eq!(stacks.stacks[0][0], 'Z');
        assert_eq!(stacks.stacks[0][1], 'N');
        assert_eq!(stacks.stacks[1][2], 'D');
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, String::from("CMZ"));
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 0);
    }
}
