use common::{input, Error};
use day_19::{first, second};

fn main() -> Result<(), Error> {
    let input = input().or(Error::new("Couldn't read input file"))?;
    println!("Using puzzle input {:?}", input);

    println!("First output: {}", first(&input));
    println!("Second output: {}", second(&input)); // 3186 is too high
    Ok(())
}
