use common::{input, Error};
use day_11::{first, second};

fn main() -> Result<(), Error> {
    let input = input().or(Error::new("Couldn't read input file"))?;
    println!("Using puzzle input {:?}", input);

    println!("First output: {}", first(&input));
    second(&input);
    Ok(())
}