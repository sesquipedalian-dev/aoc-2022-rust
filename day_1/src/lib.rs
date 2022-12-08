use common::*;

pub fn first(input: &[String]) -> u32 {
    let input = input_to_nums(input);

    let (_, max_cals) = input.iter().fold((0, 0), |(cur_cals, max_cals), item| {
        match item {
            // new elf
            0 if cur_cals > max_cals => (0, cur_cals),
            0 => (0, max_cals),
            // accumulate weight in current elf
            _ => (cur_cals + item, max_cals),
        }
    });

    return max_cals;
}

pub fn second(input: &[String]) -> u32 {
    let input = input_to_nums(input);

    let (_, max_cals) = input
        .iter()
        .fold((0, (vec![0, 0, 0])), |(cur_cals, max_cals), item| {
            match item {
                // new elf
                0 => {
                    let mut new_max: Vec<u32> = vec![];
                    let mut i = 0;
                    let mut fill_mode = false;
                    while new_max.len() < 3 && i < 3 {
                        // if this is the spot to insert cur_cals
                        if !fill_mode && (max_cals[i] < cur_cals) {
                            new_max.push(cur_cals);
                            fill_mode = true;
                        } else {
                            // otherwise grab an entry from the current max_cals
                            new_max.push(max_cals[i]);
                            i = i + 1;
                        }
                    }
                    (0, new_max)
                }
                // accumulate weight in current elf
                _ => (cur_cals + item, max_cals),
            }
        });

    return max_cals.iter().fold(0, |cur, next| cur + next);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000", "",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }
    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, 24000);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 45000);
    }
}
