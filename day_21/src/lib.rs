use std::collections::HashMap;

pub fn first(input: &[String]) -> usize {
    let nodes = parse_input(&input);
    nodes.get(&String::from("root")).unwrap().value(&nodes)
}

pub fn second(input: &[String]) -> usize {
    // ok for part 2
    // we need to fix a value somewhere in the tree ('humn:') such that it makes
    // the side of the tree under root that it is part of equal
    // to the other side of the tree.
    //
    // so the first question would be which of root's branches is humn, then calculate the value
    // of the other branch.
    //
    // then we can bubble down the humn branch by doing inverse operations
    // for example, for root: abcd == efgh, abcd: aacd * humn, aacd: 5, humn: ?
    // efgh: 10
    // at abcd we can say what * 5 = 10?  2.
    // if humn is further down the tree from there we can keep 'bubbling down' those inverse operations to set it.
    //

    let nodes = parse_input(&input);
    nodes
        .get(&String::from("root"))
        .unwrap()
        .find_human(&String::from("root"), 0, &nodes)
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
    fn find_human(
        &self,
        my_name: &String,
        target_value: usize,
        nodes: &HashMap<String, Node>,
    ) -> usize {
        if my_name == &String::from("humn") {
            return target_value;
        }

        let (lhs_name, rhs_name) = self.child_names();
        let lhs_node = nodes.get(lhs_name).unwrap();
        let rhs_node = nodes.get(rhs_name).unwrap();

        let (other_side_node, fix_me_name, fix_me_node) = if lhs_name == &String::from("humn")
            || lhs_node.contains_name(&String::from("humn"), &nodes)
        {
            (rhs_node, lhs_name, lhs_node)
        } else {
            (lhs_node, rhs_name, rhs_node)
        };

        let other_side_value = other_side_node.value(&nodes);
        let new_target_value = self.invert(my_name, fix_me_name, target_value, other_side_value);
        fix_me_node.find_human(fix_me_name, new_target_value, &nodes)
    }

    fn invert(
        &self,
        my_name: &String,
        name: &String,
        target_value: usize,
        other_side_value: usize,
    ) -> usize {
        match self {
            _ if my_name == &String::from("root") => other_side_value,
            Node::Add(_, _) => target_value - other_side_value,
            Node::Multiply(_, _) => target_value / other_side_value,
            // ? - a = b ? - 5 = 298
            // ? = b + a
            // a - ? = b
            // a - b = ?
            Node::Subtract(n, _) if n == name => target_value + other_side_value,
            Node::Subtract(_, n) if n == name => other_side_value - target_value,
            // ? / a = c
            // ? = ca
            // a / ? = c
            // ? = a/c
            Node::Divide(n, _) if n == name => target_value * other_side_value,
            Node::Divide(_, n) if n == name => other_side_value / target_value,
            _ => panic!("wrong node type!"),
        }
    }

    fn child_names(&self) -> (&String, &String) {
        match self {
            Node::Add(lhs_name, rhs_name) => (lhs_name, rhs_name),
            Node::Subtract(lhs_name, rhs_name) => (lhs_name, rhs_name),
            Node::Multiply(lhs_name, rhs_name) => (lhs_name, rhs_name),
            Node::Divide(lhs_name, rhs_name) => (lhs_name, rhs_name),
            _ => panic!("unknown node type"),
        }
    }

    fn contains_name(&self, name: &String, all_nodes: &HashMap<String, Node>) -> bool {
        if let Node::Leaf(_) = self {
            return false;
        }

        let (lhs_name, rhs_name) = self.child_names();

        lhs_name == name
            || rhs_name == name
            || all_nodes
                .get(lhs_name)
                .unwrap()
                .contains_name(name, &all_nodes)
            || all_nodes
                .get(rhs_name)
                .unwrap()
                .contains_name(name, all_nodes)
    }

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
        assert_eq!(result, 301);
    }
}
