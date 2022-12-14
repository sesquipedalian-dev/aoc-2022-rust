use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn first(input: &[String]) -> usize {
0
}

pub fn second(input: &[String]) -> usize {
0
}

fn parse_input(input: &[String]) -> Vec<(Vec<PacketItem>, Vec<PacketItem>)> {
    let mut pairs: Vec<(Vec<PacketItem>, Vec<PacketItem>)> = vec!();
    let mut current_pair: Vec<Vec<PacketItem>> = vec!();

    for line in input {
        if line.len() < 1 { 
            let mut first_member: Vec<PacketItem> = vec!();
            first_member.append(&mut current_pair[0]);
            let mut second_member: Vec<PacketItem> = vec!();
            second_member.append(&mut current_pair[1]);
            pairs.push((first_member, second_member));
            current_pair.clear();
            continue 
        }

        let mut items: Vec<PacketItem> = vec!(PacketItem{integer: None, list: Some(vec!())});
        let mut current_integer = String::new();

        for c in line.chars().skip(1).take(line.len() - 2) { // skip first and last chars
            print!("{}", c);
            match c {
                '[' => {
                    items.push(PacketItem{integer: None, list: Some(vec!())})
                },
                ']' => {
 
                    let mut next_item = items.pop().unwrap();
                    if current_integer.len() > 0 {
                        next_item.list.as_mut().unwrap().push(PacketItem{integer: Some(current_integer.parse().unwrap()), list: None});
                        current_integer.clear();
                    }
                    let parent_list = items.last_mut().unwrap();
                    let parent_list: &mut Vec<PacketItem> = parent_list.list.as_mut().unwrap();
                    parent_list.push(next_item);
                },
                ',' => {
                    if current_integer.len() > 0 {
                        items.last_mut().unwrap().list.as_mut().unwrap().push(PacketItem{integer: Some(current_integer.parse().unwrap()), list: None});
                        current_integer.clear()
                    }
                },
                // accumulate a number
                c => {
                    current_integer.push(c);
                }
            }
        }
        println!();

        let mut top_item = items.pop().unwrap();
        let mut top_item_list = top_item.list.unwrap();
        if current_integer.len() > 0 { 
            top_item_list.push(PacketItem{integer: Some(current_integer.parse().unwrap()), list: None})
        }
        current_pair.push(top_item_list);
    }

    pairs
}

#[derive(Debug)]
struct PacketItem {
    integer: Option<usize>, 
    list: Option<Vec<PacketItem>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
            "[1,1,3,1,1]",
            "[1,1,5,1,1]",
            "",
            "[[1],[2,3,4]]",
            "[[1],4]",
            "",
            "[9]",
            "[[8,7,6]]",
            "",
            "[[4,4],4,4]",
            "[[4,4],4,4,4]",
            "",
            "[7,7,7,7]",
            "[7,7,7]",
            "",
            "[]",
            "[3]",
            "",
            "[[[]]]",
            "[[]]",
            "",
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
            "",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn test_parse_input() {
        let input = example(); 
        let pairs = parse_input(&input);

        println!("first pair ({:?}) vs ({:?})", pairs[0].0, pairs[0].1);
        println!("second pair ({:?}) vs", pairs[1].0);
        println!("{:?}", pairs[1].1);
        // assert_eq!(1, 0);
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
