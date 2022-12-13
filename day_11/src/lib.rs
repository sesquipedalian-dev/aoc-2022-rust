pub fn first(input: &[String]) -> usize {
    0
}

pub fn second(input: &[String]) -> usize {
    0
}

struct Monkey{
    // the items the monkey is holding
    items: Vec<usize>,
    // function to adjust the item values prior to evaluating
    adjustment: MonkeyWorryAdjustment,
    // is value divisible by divisor to evaluate which target
    divisor: usize,
    // target monkey index to throw to depending on evaluation
    true_target: usize,
    false_target: usize,
}

impl Monkey {
    fn evaluate_items(&mut self, monkey_collection: &mut[Monkey]) {
        while let Some(item) = self.items.pop() {
            let adjusted = self.adjustment.adjust(item);
            let bored = adjusted / 3;
            let divisible = bored % self.divisor == 0;
            let target = if divisible {self.true_target
            } else {
                self.false_target
            };
            
            monkey_collection[target].items.push(bored);
        }
    }
}

struct MonkeyWorryAdjustment {
    lhs: MonkeyOpValue,
    rhs: MonkeyOpValue,
    op: MonkeyOpOperation
}

impl MonkeyWorryAdjustment {
    fn adjust(&self, value: usize) -> usize {
        self.op.adjust(self.lhs.get(value), self.rhs.get(value))
    }
}

enum MonkeyOpOperation {
    Add,
    Multiply
}

impl MonkeyOpOperation {
    fn adjust(&self, lhs: usize, rhs:usize) -> usize {
        match self {
            MonkeyOpOperation::Add => lhs + rhs,
            MonkeyOpOperation::Multiply => lhs * rhs,
        }
    }
}

enum MonkeyOpValue {
    Old,
    Value(usize)
}

impl MonkeyOpValue {
    fn get(&self, item: usize) -> usize {
        match self {
            MonkeyOpValue::Old => item,
            MonkeyOpValue::Value(amount) => *amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![];
        input.iter().map(|s: &&str| String::from(*s)).collect()
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
        assert_eq!(0, 0);
    }
}
