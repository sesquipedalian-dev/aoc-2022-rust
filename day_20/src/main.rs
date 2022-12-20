use common::{input, Error};
use day_20::{first, second};

fn main() -> Result<(), Error> {
    let input = input().or(Error::new("Couldn't read input file"))?;
    println!("Using puzzle input {:?}", input);

    println!("First output: {}", first(&input)); // -1281, -9138, -3750 is not the right answer
    println!("Second output: {}", second(&input));
    Ok(())
}
