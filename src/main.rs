use std::io::Error;
mod day_one;

fn main() -> Result<(), Error> {
    let result_1 = day_one::solve()?;
    println!("Day 1: {}", result_1);

    Ok(())
}