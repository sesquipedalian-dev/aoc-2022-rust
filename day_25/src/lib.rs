pub fn first(input: &[String]) -> String {
    let summed: usize = input.iter().map(|s| snafu_to_decimal(s)).sum();
    decimal_to_snafu(summed)
}

pub fn second(input: &[String]) -> usize {
    0
}

fn snafu_to_decimal(input: &String) -> usize {
    let mut decimal = 0;
    let l = input.len();
    for (i, c) in input.chars().enumerate() {
        let exponent: u32 = (l - i - 1) as u32;
        let place_value: isize = 5isize.pow(exponent);
        let this_value: isize = match c {
            '0' => 0,
            '1' => place_value,
            '2' => place_value * 2,
            '-' => place_value * -1,
            '=' => place_value * -2,
            _ => panic!("unknown character {}", c),
        };
        decimal += this_value;
    }
    decimal as usize
}

fn decimal_to_snafu(input: usize) -> String {
    const MAX_LEN: usize = 24;
    let mut digits = [0usize; MAX_LEN];

    // step 1: set the initial base 5 digits using normal 0-4 digits
    let mut remaining_input = input;
    for exponent in (0..(MAX_LEN -1)).rev() {
        let place_value = 5usize.pow(exponent as u32);
        let quotient = remaining_input / place_value;
        remaining_input -= quotient * place_value;
        digits[exponent] = quotient;
    }

    // step 2: shift left any digits 3 or 4 - since they are represented as 1= and 1- respectively.
    let mut carry = false;
    for i in 0..(MAX_LEN - 1) {
        if carry {
            digits[i] += 1;
        }
        carry = digits[i] > 2;
    }

    // step 3: convert digits to snafu digits
    let mut sb = String::from("");
    let mut found_first_non_zero = false;
    for digit in digits.iter().rev() {
        if *digit != 0 { 
            found_first_non_zero = true;
        }
        match digit {
            0 if !found_first_non_zero => (),
            x if *x < 3 => sb.push_str(digit.to_string().as_str()),
            3 => sb.push('='),
            4 => sb.push('-'),
            5 => sb.push('0'),
            _ => panic!("unknown digit!"),
        }
    }
    sb
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec!(
            "1=-0-2",
            "12111",
            "2=0=",
            "21",
            "2=01",
            "111",
            "20012",
            "112",
            "1=-1=",
            "1-12",
            "12",
            "1=",
            "122",
        );
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn test_snafu_to_decimal() {
        assert_eq!(snafu_to_decimal(&String::from("1=-0-2")), 1747);
        assert_eq!(snafu_to_decimal(&String::from("12111")), 906);
        assert_eq!(snafu_to_decimal(&String::from("2=0=")), 198);
        assert_eq!(snafu_to_decimal(&String::from("1121-1110-1=0")), 314159265);
    }

    #[test]
    fn test_decimal_to_snafu() { 
        assert_eq!(decimal_to_snafu(1747), String::from("1=-0-2"));
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, String::from("2=-1=0"));
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 0);
    }
}
