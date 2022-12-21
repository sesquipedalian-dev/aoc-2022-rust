use common::{input, Error};
use day_21::{first, second};

fn main() -> Result<(), Error> {
    let input = input().or(Error::new("Couldn't read input file"))?;
    println!("Using puzzle input {:?}", input);

    println!("First output: {}", first(&input)); // it's not 358
    println!("Second output: {}", second(&input));
    Ok(())
}
