use common::{input, Error};
use day_19::{first, second};

fn main() -> Result<(), Error> {
    // let input = input().or(Error::new("Couldn't read input file"))?;
    let blueprint1 = day_19::Blueprint{
        resource_needs: vec!(
            vec!(4, 0, 0 ,0), 
            vec!(2, 0, 0, 0),
            vec!(3, 14, 0, 0),
            vec!(2, 0, 7, 0),
        )
    };
    let blueprint2 = day_19::Blueprint{
        resource_needs: vec!(
            vec!(2, 0, 0 ,0), 
            vec!(3, 0, 0, 0),
            vec!(3, 8, 0, 0),
            vec!(3, 0, 12, 0),
        )
    };
    let blueprints = vec!(blueprint1, blueprint2);
    println!("Using puzzle input {:?}", blueprints);

    println!("First output: {}", first(&blueprints));
    // println!("Second output: {}", second(&blueprints)); 
    Ok(())
}
