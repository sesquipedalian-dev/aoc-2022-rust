pub fn first(input: &[String]) -> usize {
    let mut pairs = parse_input(&input);
    pairs
        .iter_mut()
        .enumerate()
        .fold(0, |accum, (i, (lhs, rhs))| {
            let mut item1_list = vec![];
            item1_list.append(lhs);
            let item1 = PacketItem {
                integer: None,
                list: Some(item1_list),
            };

            let mut item2_list = vec![];
            item2_list.append(rhs);
            let item2 = PacketItem {
                integer: None,
                list: Some(item2_list),
            };

            if in_right_order((&item1, &item2)) == InRightOrder::RightOrder {
                accum + i + 1
            } else {
                accum
            }
        })
}

pub fn second(input: &[String]) -> usize {
    // flatten the pairs into a flat list of packets items with lists
    let mut pairs = parse_input(&input);
    let mut packets: Vec<PacketItem> = vec![];
    for (item1, item2) in pairs.iter_mut() {
        let mut item1_list = vec![];
        item1_list.append(item1);
        let item1 = PacketItem {
            integer: None,
            list: Some(item1_list),
        };
        packets.push(item1);

        let mut item2_list = vec![];
        item2_list.append(item2);
        let item2 = PacketItem {
            integer: None,
            list: Some(item2_list),
        };
        packets.push(item2);
    }

    // add [[6]] and [[2]]
    packets.push(PacketItem {
        integer: None,
        list: Some(vec![PacketItem {
            integer: Some(2),
            list: None,
        }]),
    });
    packets.push(PacketItem {
        integer: None,
        list: Some(vec![PacketItem {
            integer: Some(6),
            list: None,
        }]),
    });

    // sort using our compare function
    packets.sort_by(|a, b| match in_right_order((&a, &b)) {
        InRightOrder::RightOrder => std::cmp::Ordering::Less,
        InRightOrder::Unknown => std::cmp::Ordering::Equal,
        InRightOrder::WrongOrder => std::cmp::Ordering::Greater,
    });

    // find index of [[2]] and [[6]] and multiply them
    packets
        .iter()
        .enumerate()
        .filter(|(_, item)| {
            item.list
                .as_ref()
                .filter(|first_list| {
                    first_list.len() == 1
                        && first_list[0]
                            .integer
                            .filter(|v| *v == 2 || *v == 6)
                            .is_some()
                })
                .is_some()
        })
        .map(|(index, _)| index + 1)
        .reduce(|a, b| a * b)
        .unwrap_or_default()
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum InRightOrder {
    RightOrder,
    WrongOrder,
    Unknown,
}

fn in_right_order(pair: (&PacketItem, &PacketItem)) -> InRightOrder {
    match pair {
        // two ints
        (
            PacketItem {
                integer: Some(lhs),
                list: _,
            },
            PacketItem {
                integer: Some(rhs),
                list: _,
            },
        ) => match lhs.cmp(&rhs) {
            std::cmp::Ordering::Less => InRightOrder::RightOrder,
            std::cmp::Ordering::Equal => InRightOrder::Unknown,
            std::cmp::Ordering::Greater => InRightOrder::WrongOrder,
        },
        // two lists
        (
            PacketItem {
                integer: _,
                list: Some(lhs),
            },
            PacketItem {
                integer: _,
                list: Some(rhs),
            },
        ) => {
            let compare_pairwise = {
                let mut pair_iter = lhs.iter().zip(rhs.iter());
                loop {
                    match pair_iter.next() {
                        Some(pair) => match in_right_order(pair) {
                            InRightOrder::Unknown => (),
                            v => break v,
                        },
                        _ => break InRightOrder::Unknown,
                    }
                }
            };

            if compare_pairwise != InRightOrder::Unknown {
                return compare_pairwise;
            }

            // someone ran out of items
            match lhs.len().cmp(&rhs.len()) {
                std::cmp::Ordering::Less => InRightOrder::RightOrder,
                std::cmp::Ordering::Equal => InRightOrder::Unknown,
                std::cmp::Ordering::Greater => InRightOrder::WrongOrder,
            }
        }
        // one list and one int
        _ => {
            if pair.0.integer.is_some() {
                let new_lhs = PacketItem {
                    integer: None,
                    list: Some(vec![PacketItem {
                        integer: pair.0.integer,
                        list: None,
                    }]),
                };
                in_right_order((&new_lhs, pair.1))
            } else {
                let new_rhs = PacketItem {
                    integer: None,
                    list: Some(vec![PacketItem {
                        integer: pair.1.integer,
                        list: None,
                    }]),
                };
                in_right_order((pair.0, &new_rhs))
            }
        }
    }
}

fn parse_input(input: &[String]) -> Vec<(Vec<PacketItem>, Vec<PacketItem>)> {
    let mut pairs: Vec<(Vec<PacketItem>, Vec<PacketItem>)> = vec![];
    let mut current_pair: Vec<Vec<PacketItem>> = vec![];

    for line in input {
        if line.len() < 1 {
            let mut first_member: Vec<PacketItem> = vec![];
            first_member.append(&mut current_pair[0]);
            let mut second_member: Vec<PacketItem> = vec![];
            second_member.append(&mut current_pair[1]);
            pairs.push((first_member, second_member));
            current_pair.clear();
            continue;
        }

        let mut items: Vec<PacketItem> = vec![PacketItem {
            integer: None,
            list: Some(vec![]),
        }];
        let mut current_integer = String::new();

        for c in line.chars().skip(1).take(line.len() - 2) {
            // skip first and last chars
            match c {
                '[' => items.push(PacketItem {
                    integer: None,
                    list: Some(vec![]),
                }),
                ']' => {
                    let mut next_item = items.pop().unwrap();
                    if current_integer.len() > 0 {
                        next_item.list.as_mut().unwrap().push(PacketItem {
                            integer: Some(current_integer.parse().unwrap()),
                            list: None,
                        });
                        current_integer.clear();
                    }
                    let parent_list = items.last_mut().unwrap();
                    let parent_list: &mut Vec<PacketItem> = parent_list.list.as_mut().unwrap();
                    parent_list.push(next_item);
                }
                ',' => {
                    if current_integer.len() > 0 {
                        items
                            .last_mut()
                            .unwrap()
                            .list
                            .as_mut()
                            .unwrap()
                            .push(PacketItem {
                                integer: Some(current_integer.parse().unwrap()),
                                list: None,
                            });
                        current_integer.clear()
                    }
                }
                // accumulate a number
                c => {
                    current_integer.push(c);
                }
            }
        }

        let top_item = items.pop().unwrap();
        let mut top_item_list = top_item.list.unwrap();
        if current_integer.len() > 0 {
            top_item_list.push(PacketItem {
                integer: Some(current_integer.parse().unwrap()),
                list: None,
            })
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
    fn test_in_order() {
        let input = example();
        let mut pairs = parse_input(&input);

        let mut item1_list = vec![];
        item1_list.append(&mut pairs[0].0);
        let item1 = PacketItem {
            integer: None,
            list: Some(item1_list),
        };

        let mut item2_list = vec![];
        item2_list.append(&mut pairs[0].1);
        let item2 = PacketItem {
            integer: None,
            list: Some(item2_list),
        };

        assert_eq!(in_right_order((&item1, &item2)), InRightOrder::RightOrder);

        let mut item1_list = vec![];
        item1_list.append(&mut pairs[2].0);
        let item1 = PacketItem {
            integer: None,
            list: Some(item1_list),
        };

        let mut item2_list = vec![];
        item2_list.append(&mut pairs[2].1);
        let item2 = PacketItem {
            integer: None,
            list: Some(item2_list),
        };

        assert_eq!(in_right_order((&item1, &item2)), InRightOrder::WrongOrder);
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, 13);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 140);
    }
}
