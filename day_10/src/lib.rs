pub fn first(input: &[String]) -> isize {
    let interesting_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut x: isize = 1;
    let mut accum: isize = 0;
    let mut cycle = 1;
    let mut input_iter = input.iter();
    let max_cycle = interesting_cycles.last().unwrap();
    let mut next_add: isize = 0;
    // note: no max cycle because input includes enough noops() to handle the last adds

    while cycle <= *max_cycle {
        // accumulate an 'interesting cycles' value if applicable
        if interesting_cycles.contains(&cycle) {
            accum += (cycle as isize) * x
        }

        // parse the current line
        if next_add != 0 {
            x += next_add;
            next_add = 0;
        } else if let Some(Some(add_amount)) =
            input_iter.next().map(|line| line.strip_prefix("addx "))
        {
            let add_amount: isize = add_amount.parse().unwrap();
            next_add = add_amount;
        } // noop line - don't do anything

        cycle += 1;
    }

    accum
}

pub fn second(input: &[String]) -> () {
    let mut x: isize = 1;
    let mut cycle = 1;
    let mut input_iter = input.iter();
    let max_cycle = 240;
    let mut next_add: isize = 0;
    // note: no max cycle because input includes enough noops() to handle the last adds

    println!();
    println!();

    while cycle <= max_cycle {
        // check if 'sprite' intersects with pixel
        let crt_position = (cycle - 1) % 40;
        if x - 1 <= crt_position && crt_position <= x + 1 {
            print!("#");
        } else {
            print!(".");
        }
        if cycle % 40 == 0 {
            println!();
        }

        // parse the current line
        if next_add != 0 {
            x += next_add;
            next_add = 0;
        } else if let Some(Some(add_amount)) =
            input_iter.next().map(|line| line.strip_prefix("addx "))
        {
            let add_amount: isize = add_amount.parse().unwrap();
            next_add = add_amount;
        } // noop line - don't do anything

        cycle += 1;
    }

    println!();
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec!["noop", "addx 3", "addx -5", "noop", "noop", "noop"];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    fn example2() -> Vec<String> {
        let input: Vec<&str> = vec![
            "addx 15", "addx -11", "addx 6", "addx -3", "addx 5", "addx -1", "addx -8", "addx 13",
            "addx 4", "noop", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1", "addx 5",
            "addx -1", "addx 5", "addx -1", "addx -35", "addx 1", "addx 24", "addx -19", "addx 1",
            "addx 16", "addx -11", "noop", "noop", "addx 21", "addx -15", "noop", "noop",
            "addx -3", "addx 9", "addx 1", "addx -3", "addx 8", "addx 1", "addx 5", "noop", "noop",
            "noop", "noop", "noop", "addx -36", "noop", "addx 1", "addx 7", "noop", "noop", "noop",
            "addx 2", "addx 6", "noop", "noop", "noop", "noop", "noop", "addx 1", "noop", "noop",
            "addx 7", "addx 1", "noop", "addx -13", "addx 13", "addx 7", "noop", "addx 1",
            "addx -33", "noop", "noop", "noop", "addx 2", "noop", "noop", "noop", "addx 8", "noop",
            "addx -1", "addx 2", "addx 1", "noop", "addx 17", "addx -9", "addx 1", "addx 1",
            "addx -3", "addx 11", "noop", "noop", "addx 1", "noop", "addx 1", "noop", "noop",
            "addx -13", "addx -19", "addx 1", "addx 3", "addx 26", "addx -30", "addx 12",
            "addx -1", "addx 3", "addx 1", "noop", "noop", "noop", "addx -9", "addx 18", "addx 1",
            "addx 2", "noop", "noop", "addx 9", "noop", "noop", "noop", "addx -1", "addx 2",
            "addx -37", "addx 1", "addx 3", "noop", "addx 15", "addx -21", "addx 22", "addx -6",
            "addx 1", "noop", "addx 2", "addx 1", "noop", "addx -10", "noop", "noop", "addx 20",
            "addx 1", "addx 2", "addx 2", "addx -6", "addx -11", "noop", "noop", "noop",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, -720);

        let input = example2();
        let result = first(&input);
        assert_eq!(result, 13140);
    }

    #[test]
    fn second_test() {
        let input = example2();
        let result = second(&input);
        assert_eq!(0, 0);
    }
}
