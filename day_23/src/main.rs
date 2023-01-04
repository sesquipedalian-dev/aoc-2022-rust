use common::{input, Error};
use day_23::{first, second};

fn main() -> Result<(), Error> {
    let input = input().or(Error::new("Couldn't read input file"))?;
    println!("Using puzzle input {:?}", input);

    println!("First output: {}", first(&input, 10)); //17298 is too high 
    println!("Second output: {}", second(&input)); 
    Ok(())
}
