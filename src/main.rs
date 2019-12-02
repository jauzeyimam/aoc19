mod day_one;
mod day_two;
mod util;

use std::io::Error;

fn main() -> Result<(), Error> {
    day_one::solve()?;
    day_two::solve()?;

    Ok(())
}