mod day_eight;
mod day_five;
mod day_four;
mod day_one;
mod day_seven;
mod day_six;
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
    solutions.push(Box::new(day_four::DayFour));
    solutions.push(Box::new(day_five::DayFive));
    solutions.push(Box::new(day_six::DaySix));
    solutions.push(Box::new(day_seven::DaySeven));
    solutions.push(Box::new(day_eight::DayEight));

    for solution in solutions {
        let result = solution.solve()?;
        println!("{}", result);
    }

    Ok(())
}
