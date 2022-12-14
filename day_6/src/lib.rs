pub fn first(input: &[String]) -> usize {
    find_string_of_distinct_characters(input, 4)
}

pub fn second(input: &[String]) -> usize {
    find_string_of_distinct_characters(input, 14)
}

fn find_string_of_distinct_characters(input: &[String], string_size: usize) -> usize {
    let mut i = string_size - 1; // start at 4th char so we have enough entries
    let input_chars: Vec<char> = input[0].chars().collect();
    while i < input_chars.len() {
        if input_chars
            .iter()
            .zip((i - (string_size - 1))..i)
            .fold(true, |accum, (_, j)| {
                accum
                    && ((j + 1)..=i)
                        .fold(true, |accum, k| accum && input_chars[j] != input_chars[k])
            })
        {
            break; // found at i
        }

        i += 1;
    }

    i + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn examples() -> Vec<String> {
        let input: Vec<&str> = vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn first_test() {
        let input = examples();
        let mut results = input.chunks(1).map(|slice| first(&slice));
        assert_eq!(results.next(), Some(7));
        assert_eq!(results.next(), Some(5));
        assert_eq!(results.next(), Some(6));
        assert_eq!(results.next(), Some(10));
        assert_eq!(results.next(), Some(11));
    }

    #[test]
    fn second_test() {
        let input = examples();
        let mut results = input.chunks(1).map(|slice| second(&slice));
        assert_eq!(results.next(), Some(19));
        assert_eq!(results.next(), Some(23));
        assert_eq!(results.next(), Some(23));
        assert_eq!(results.next(), Some(29));
        assert_eq!(results.next(), Some(26));
    }
}
