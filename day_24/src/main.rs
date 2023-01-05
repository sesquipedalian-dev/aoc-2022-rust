use common::{input, Error};
use day_24::{first, second};

fn main() -> Result<(), Error> {
    let input = input().or(Error::new("Couldn't read input file"))?;
    println!("Using puzzle input {:?}", input);

    println!("First output: {}", first(&input, 20, 150)); // 257, 335 is too low 343 is right!
    println!("Second output: {}", second(&input, 20, 150));
    Ok(())
}
