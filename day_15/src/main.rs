use common::{input, Error};
use day_15::{first, second};

fn main() -> Result<(), Error> {
    let input = input().or(Error::new("Couldn't read input file"))?;
    println!("Using puzzle input {:?}", input);

    println!("First output: {}", first(&input, 2_000_000));
    println!("Second output: {}", second(&input, 4_000_000));
    Ok(())
}
