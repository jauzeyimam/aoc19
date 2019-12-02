use super::util;
use math::round;
use std::fs::File;
use std::io::Error;


pub fn solve() -> Result<u32, Error> {

    let input = util::read(File::open("input/1.txt")?)?;

    let mut sum = 0;

    for number in input {
        let feul = calc_fuel(number);
        sum += feul;
    }

    Ok(sum)
}

fn calc_fuel(mass: u32) -> u32 {
    let divide_by_three = mass as f32 / 3.0;
    let floored = round::floor(divide_by_three as f64, 0) as i32;
    let subtract_two = floored - 2;

    if subtract_two <= 0 {
        return 0;
    }

    let result_feul = calc_fuel(subtract_two as u32);
    let result = subtract_two as u32 + result_feul;

    result
}