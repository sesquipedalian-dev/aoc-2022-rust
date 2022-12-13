pub fn first(input: &[String]) -> usize {
    let mut input_iter = input.iter();
    let mut monkeys: Vec<Monkey> = vec![];
    while let Some(monkey) = Monkey::from(&mut input_iter) {
        monkeys.push(monkey);
    }

    for _ in 0..20 {
        let max_monkey = monkeys.len();
        for monkey_index in 0..max_monkey {
            while let Some(item) = monkeys[monkey_index].items.pop() {
                let adjusted = monkeys[monkey_index].adjustment.adjust(item);
                let bored = adjusted / 3;
                let divisible = bored % monkeys[monkey_index].divisor == 0;
                let target = if divisible {
                    monkeys[monkey_index].true_target
                } else {
                    monkeys[monkey_index].false_target
                };

                monkeys[target].items.push(bored);
                monkeys[monkey_index].inspection_count += 1;
            }
        }

        for monkey in monkeys.iter() {
            monkey.print()
        }
    }

    // find biggest monkeys
    monkeys.sort_by(|a, b| a.inspection_count.partial_cmp(&b.inspection_count).unwrap());
    monkeys[monkeys.len() - 1].inspection_count * monkeys[monkeys.len() - 2].inspection_count
}

pub fn second(input: &[String]) -> usize {
    let mut input_iter = input.iter();
    let mut monkeys: Vec<Monkey> = vec![];
    while let Some(monkey) = Monkey::from(&mut input_iter) {
        monkeys.push(monkey);
    }

    let a_common_divisor = monkeys
        .iter()
        .fold(1, |accum, monkey| accum * monkey.divisor);

    for round in 0..10_000 {
        let max_monkey = monkeys.len();
        for monkey_index in 0..max_monkey {
            while let Some(item) = monkeys[monkey_index].items.pop() {
                let adjusted = monkeys[monkey_index]
                    .adjustment
                    .adjust_with_modulus(item, a_common_divisor);
                let divisible = adjusted % monkeys[monkey_index].divisor == 0;
                let target = if divisible {
                    monkeys[monkey_index].true_target
                } else {
                    monkeys[monkey_index].false_target
                };

                monkeys[target].items.push(adjusted);
                monkeys[monkey_index].inspection_count += 1;
            }
        }

        println!("round {}", round);
        for monkey in monkeys.iter() {
            monkey.print()
        }
        println!();
    }

    // find biggest monkeys
    monkeys.sort_by(|a, b| a.inspection_count.partial_cmp(&b.inspection_count).unwrap());
    monkeys[monkeys.len() - 1].inspection_count * monkeys[monkeys.len() - 2].inspection_count
}

#[derive(Default)]
struct Monkey {
    // the items the monkey is holding
    items: Vec<usize>,
    // function to adjust the item values prior to evaluating
    adjustment: MonkeyWorryAdjustment,
    // is value divisible by divisor to evaluate which target
    divisor: usize,
    // target monkey index to throw to depending on evaluation
    true_target: usize,
    false_target: usize,
    // times it  inspected something
    inspection_count: usize,
}

impl Monkey {
    fn from<'a, T>(input: &mut T) -> Option<Monkey>
    where
        T: Iterator<Item = &'a String>,
    {
        let mut found = false;
        let mut result: Monkey = core::default::Default::default();
        let mut index = 0;
        while let Some(line) = input.next() {
            if line.len() < 1 {
                break;
            }
            found = true;

            match index {
                1 => {
                    let parts = line.split_whitespace();
                    result.items = parts
                        .skip(2)
                        .map(|item| {
                            item.strip_suffix(",")
                                .or(Some(item))
                                .unwrap()
                                .parse()
                                .unwrap()
                        })
                        .collect();
                }
                2 => {
                    let mut parts = line.split_whitespace().skip(3);
                    result.adjustment = MonkeyWorryAdjustment {
                        lhs: MonkeyOpValue::from(parts.next().unwrap()).unwrap(),
                        op: MonkeyOpOperation::from(parts.next().unwrap()).unwrap(),
                        rhs: MonkeyOpValue::from(parts.next().unwrap()).unwrap(),
                    }
                }
                3 => {
                    let parts = line.split_whitespace();
                    result.divisor = parts.skip(3).next().unwrap().parse().unwrap();
                }
                4 => {
                    let parts = line.split_whitespace();
                    result.true_target = parts.skip(5).next().unwrap().parse().unwrap();
                }
                5 => {
                    let parts = line.split_whitespace();
                    result.false_target = parts.skip(5).next().unwrap().parse().unwrap();
                }
                _ => (),
            }
            index += 1;
        }

        if found {
            Some(result)
        } else {
            None
        }
    }

    fn print(&self) {
        println!("Items: {:?}", self.items);
        self.adjustment.print();
        println!("Test: divisible by {}", self.divisor);
        println!("if true: throw to monkey {}", self.true_target);
        println!("if false: throw to monkey {}", self.false_target);
        println!("# of items inspected {}", self.inspection_count);
    }
}

struct MonkeyWorryAdjustment {
    lhs: MonkeyOpValue,
    rhs: MonkeyOpValue,
    op: MonkeyOpOperation,
}

impl MonkeyWorryAdjustment {
    fn adjust(&self, value: usize) -> usize {
        self.op.adjust(self.lhs.get(value), self.rhs.get(value))
    }

    fn adjust_with_modulus(&self, value: usize, divisor: usize) -> usize {
        self.op
            .adjust_with_modulus(self.lhs.get(value), self.rhs.get(value), divisor)
    }

    fn print(&self) {
        print!("Operation: new = ");
        self.lhs.print();
        print!(" ");
        self.op.print();
        print!(" ");
        self.rhs.print();
        println!();
    }
}

impl Default for MonkeyWorryAdjustment {
    fn default() -> MonkeyWorryAdjustment {
        MonkeyWorryAdjustment {
            lhs: MonkeyOpValue::Old,
            rhs: MonkeyOpValue::Old,
            op: MonkeyOpOperation::Add,
        }
    }
}

enum MonkeyOpOperation {
    Add,
    Multiply,
}

impl MonkeyOpOperation {
    fn from(string: &str) -> Option<MonkeyOpOperation> {
        match string {
            "*" => Some(MonkeyOpOperation::Multiply),
            "+" => Some(MonkeyOpOperation::Add),
            _ => None,
        }
    }

    fn adjust(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            MonkeyOpOperation::Add => lhs + rhs,
            MonkeyOpOperation::Multiply => lhs * rhs,
        }
    }

    fn adjust_with_modulus(&self, lhs: usize, rhs: usize, divisor: usize) -> usize {
        match self {
            MonkeyOpOperation::Add => lhs + rhs,
            MonkeyOpOperation::Multiply => match (lhs * rhs) % divisor {
                0 => divisor,
                result => result,
            },
        }
    }

    fn print(&self) {
        match self {
            MonkeyOpOperation::Add => print!("+"),
            MonkeyOpOperation::Multiply => print!("*"),
        }
    }
}

enum MonkeyOpValue {
    Old,
    Value(usize),
}

impl MonkeyOpValue {
    fn from(string: &str) -> Option<MonkeyOpValue> {
        match string {
            "old" => Some(MonkeyOpValue::Old),
            string => string
                .parse::<usize>()
                .map(|v| MonkeyOpValue::Value(v))
                .ok(),
        }
    }
    fn get(&self, item: usize) -> usize {
        match self {
            MonkeyOpValue::Old => item,
            MonkeyOpValue::Value(amount) => *amount,
        }
    }
    fn print(&self) {
        match self {
            MonkeyOpValue::Old => print!("old"),
            MonkeyOpValue::Value(amount) => print!("{}", amount),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
            "Monkey 0:",
            "  Starting items: 79, 98",
            "  Operation: new = old * 19",
            "  Test: divisible by 23",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 3",
            "",
            "Monkey 1:",
            "  Starting items: 54, 65, 75, 74",
            "  Operation: new = old + 6",
            "  Test: divisible by 19",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 0",
            "",
            "Monkey 2:",
            "  Starting items: 79, 60, 97",
            "  Operation: new = old * old",
            "  Test: divisible by 13",
            "    If true: throw to monkey 1",
            "    If false: throw to monkey 3",
            "",
            "Monkey 3:",
            "  Starting items: 74",
            "  Operation: new = old + 3",
            "  Test: divisible by 17",
            "    If true: throw to monkey 0",
            "    If false: throw to monkey 1",
            "",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn test_parsing() {
        let input = example();
        let mut input_iter = input.iter();
        while let Some(monkey) = Monkey::from(&mut input_iter) {
            monkey.print()
        }

        assert_eq!(0, 0);
    }
    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, 10605);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 2713310158);
    }
}
