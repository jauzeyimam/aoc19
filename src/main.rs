mod day_one;
mod day_three;
mod day_two;
mod util;

use std::io::Error;

trait Solution {
    fn solve(&self) -> Result<String, Error>;
}

fn main() -> Result<(), Error> {
    let mut solutions: Vec<Box<dyn Solution>> = Vec::new();
    solutions.push(Box::new(day_one::DayOne));
    solutions.push(Box::new(day_two::DayTwo));
    solutions.push(Box::new(day_three::DayThree));

    for solution in solutions {
        let result = solution.solve()?;
        println!("{}", result);
    }

    Ok(())
}
