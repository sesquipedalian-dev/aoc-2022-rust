pub fn first(input: &[String]) -> usize {
    0
}

pub fn second(input: &[String]) -> usize {
    0
}

pub fn parse_input(input: &[String]) -> usize {
0
}


#[derive(Debug, PartialEq, Hash, Clone, Eq)]
struct Move {
    from: usize,
    to: usize, 
    count: usize
}

struct CargoStacks {
    stacks: Vec<Vec<char>>
}

impl CargoStacks {
    /// parse the initial stack state until a blank line is hit; mutates the iterator so
    /// the move parser can go next
    fn from<'a, T>(input: &mut T) -> CargoStacks 
    where
        T: Iterator<Item = &'a String>
    {
        let mut stacks = CargoStacks { 
            stacks: vec![]
        };
        
        while let Some(line) = input.next() {
            if line.len() < 1 {
                break;
            }

            // initialize stack size based on the line length
            if stacks.stacks.len() == 0 {
                (0..line.len() / 4).for_each(|_| stacks.stacks.push(vec!()));
            }

            line.chars()
                .zip(0..line.len()) // pair chars with index
                .filter(|(_, index)| { index % 4 == 1 }) // each stack repr is "[X] ", the 2nd character is the char
                .for_each(|(item, index)| { match item {
                    item if '1' <= item && item <= '9' => (),
                    _ => stacks.stacks[index/4].insert(0, item), // if it's not the stack indices then it's to be inserted
                }})
                ;
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
        assert_eq!(result, 0);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 0);
    }
}
