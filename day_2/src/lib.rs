pub fn first(input: &[String]) -> i32 {
    let move_pairs = input_to_ascii_val(&input);

    move_pairs
        .iter()
        .fold(0i32, |total_score, (their_move, my_move)| {
            // score per round - to be multiplied by 3 later
            // 3 - 1 + 1 = 3 % 3 = 0  lose
            // 3 - 2 + 1 = 2 % 3 = 2  win
            // 3 - 3 + 1 = 1 % 3 = 1  draw
            let outcome_score = (my_move - their_move + 1).rem_euclid(3);
            total_score + my_move + (outcome_score * 3)
        })
}

pub fn second(input: &[String]) -> i32 {
    let move_pairs = input_to_ascii_val(input);

    move_pairs
        .iter()
        .fold(0i32, |total_score, (their_move, my_directive)| {
            let outcome_score = (my_directive - 1) * 3;

            let play_score = match my_directive {
                // lose
                1 => match their_move {
                    // TODO formula for this gr
                    1 => 3,
                    2 => 1,
                    _ => 2,
                },
                // draw
                2 => *their_move,
                // win
                _ => match their_move {
                    // TODO formula for this gr
                    1 => 2,
                    2 => 3,
                    _ => 1,
                },
            };
            total_score + outcome_score + play_score
        })
}

fn input_to_ascii_val(input: &[String]) -> Vec<(i32, i32)> {
    input
        .iter()
        .filter_map(|line| {
            if line.len() < 1 {
                return None;
            }

            let mut parts = line.split_whitespace();
            let their_move = (parts.next().unwrap().chars().next().unwrap() as i32) - 64; // ASCII A,B,C to 1,2,3
            let my_move = (parts.next().unwrap().chars().next().unwrap() as i32) - 87; // ASCII X,Y,Z to 1,2,3

            Some((their_move, my_move))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec!["A Y", "B X", "C Z", ""];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    fn all_pairs() -> Vec<String> {
        let input: Vec<&str> = vec![
            "A X", "A Y", "A Z", "B X", "B Y", "B Z", "C X", "C Y", "C Z", "",
        ];

        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn test_input_to_ascii_val() {
        let input: Vec<String> = example();
        let ascii_input = input_to_ascii_val(&input);
        let mut iter = ascii_input.iter();

        assert_eq!(iter.next(), Some(&(1, 2)));
        assert_eq!(iter.next(), Some(&(2, 1)));
        assert_eq!(iter.next(), Some(&(3, 3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_first_on_all_pairs() {
        let input: Vec<String> = all_pairs();
        let result: i32 = first(&input);
        assert_eq!(result, 45)
    }
    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, 15);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 12);
    }
}
