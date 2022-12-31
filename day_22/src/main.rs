use common::{input, Error};
use day_22::{first, second};

fn main() -> Result<(), Error> {
    let input = input().or(Error::new("Couldn't read input file"))?;
    println!("Using puzzle input {:?}", input);

    println!("First output: {}", first(&input));
    println!("Second output: {}", second(&input)); // 4602, 18433 is too low; 150018 is too high ; 4614 is wrong 15767 is wrong 41477 is wrong 134170?
    Ok(())
}
