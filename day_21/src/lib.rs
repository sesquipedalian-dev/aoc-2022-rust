use std::collections::HashMap;

pub fn first(input: &[String]) -> usize {
    let nodes = parse_input(&input);
    nodes.get(&String::from("root")).unwrap().value(&nodes)
}

pub fn second(input: &[String]) -> usize {
    0
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Node {
    Leaf(usize),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
}

impl Node {
    fn value(&self, all_nodes: &HashMap<String, Node>) -> usize {
        match self {
            Node::Leaf(v) => *v,
            Node::Add(lhs_name, rhs_name) => {
                all_nodes.get(lhs_name).unwrap().value(&all_nodes)
                    + all_nodes.get(rhs_name).unwrap().value(&all_nodes)
            }
            Node::Subtract(lhs_name, rhs_name) => {
                all_nodes.get(lhs_name).unwrap().value(&all_nodes)
                    - all_nodes.get(rhs_name).unwrap().value(&all_nodes)
            }
            Node::Multiply(lhs_name, rhs_name) => {
                all_nodes.get(lhs_name).unwrap().value(&all_nodes)
                    * all_nodes.get(rhs_name).unwrap().value(&all_nodes)
            }
            Node::Divide(lhs_name, rhs_name) => {
                all_nodes.get(lhs_name).unwrap().value(&all_nodes)
                    / all_nodes.get(rhs_name).unwrap().value(&all_nodes)
            }
        }
    }

    fn new(input: &String) -> (String, Node) {
        let mut parts: Vec<&str> = input.split_whitespace().collect();
        let node = if parts.len() > 2 {
            match parts[2] {
                "+" => Node::Add(String::from(parts[1]), String::from(parts[3])),
                "-" => Node::Subtract(String::from(parts[1]), String::from(parts[3])),
                "*" => Node::Multiply(String::from(parts[1]), String::from(parts[3])),
                "/" => Node::Divide(String::from(parts[1]), String::from(parts[3])),
                op => panic!("unknown operation {}", op),
            }
        } else {
            Node::Leaf(parts[1].parse().unwrap())
        };
        let name = String::from(parts[0].strip_suffix(":").unwrap());
        (name, node)
    }
}

fn parse_input(input: &[String]) -> HashMap<String, Node> {
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut input_iter = input.iter();

    while let Some(input) = input_iter.next() {
        let (next_name, next_node) = Node::new(input);
        nodes.insert(next_name, next_node);
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
            "root: pppw + sjmn",
            "dbpl: 5",
            "cczh: sllz + lgvd",
            "zczc: 2",
            "ptdq: humn - dvpt",
            "dvpt: 3",
            "lfqf: 4",
            "humn: 5",
            "ljgn: 2",
            "sjmn: drzm * dbpl",
            "sllz: 4",
            "pppw: cczh / lfqf",
            "lgvd: ljgn * ptdq",
            "drzm: hmdt - zczc",
            "hmdt: 32",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn test_parse_input() {
        let input = example();
        let parsed = parse_input(&input);

        assert_eq!(
            parsed.get(&String::from("root")),
            Some(&Node::Add(String::from("pppw"), String::from("sjmn")))
        );
        assert_eq!(parsed.get(&String::from("sllz")), Some(&Node::Leaf(4)));
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, 152);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 0);
    }
}
