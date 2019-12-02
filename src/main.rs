mod day_one;
mod day_two;
mod util;

use std::io::Error;

fn main() -> Result<(), Error> {
    let result_1 = day_one::solve()?;
    println!("Day 1: {}", result_1);

    let result_2 = day_two::solve()?;

    Ok(())
}