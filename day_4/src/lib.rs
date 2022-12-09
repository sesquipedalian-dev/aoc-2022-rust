pub fn first(input: &[String]) -> usize {
    let range_pairs = parse_input(input);
    range_pairs
        .fold(0, |accum, next| {
            if (next.0.start() <= next.1.start()) && (next.0.end() >= next.1.end()) ||
                (next.1.start() <= next.0.start()) && (next.1.end() >= next.0.end()){
                accum + 1
            } else {
                accum
            }
        })
}

pub fn second(input: &[String]) -> usize {
    0
}

pub fn parse_input(input: &[String]) -> impl Iterator<Item=(std::ops::RangeInclusive<usize>, std::ops::RangeInclusive<usize>)> +'_ {
    input.iter().filter_map(|line| {
        if line.len() < 1 {
            return None
        }

        let parts = line.split(',');
        let mut ranges = parts.map(|part| {
            let mut parts = part.split('-');
            let start: usize = parts.next().unwrap().parse().unwrap();
            let end: usize = parts.next().unwrap().parse().unwrap();
            start ..= end
        });
        
        Some((ranges.next().unwrap(), ranges.next().unwrap()))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn test_parse_input() {
        let input = example();
        let mut ranges = parse_input(&input);
        assert_eq!(ranges.next(), Some((2..=4, 6..=8)));
        assert_eq!(ranges.next(), Some((2..=3, 4..=5)));
        assert_eq!(ranges.next(), Some((5..=7, 7..=9)));
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 0);
    }
}
