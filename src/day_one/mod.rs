use super::util;
use math::round;
use std::fs::File;
use std::io::Error;


pub fn solve() -> Result<(), Error> {

    let input = util::read(File::open("input/1.txt")?)?;

    println!("Day 1a: {}", part_one(input.clone()));
    println!("Day 1b: {}", part_two(input.clone()));

    Ok(())

}

fn part_one(input: Vec<u32>) -> u32 {
    let mut sum = 0;

    for number in input {
        let feul = calc_fuel(number, false);
        sum += feul;
    }

    sum
}

fn part_two(input: Vec<u32>) -> u32 {
    let mut sum = 0;

    for number in input {
        let feul = calc_fuel(number, true);
        sum += feul;
    }

    sum
}

fn calc_fuel(mass: u32, fuel_mass: bool) -> u32 {
    let divide_by_three = mass as f32 / 3.0;
    let floored = round::floor(divide_by_three as f64, 0) as i32;
    let subtract_two = floored - 2;

    if !fuel_mass {
        return subtract_two as u32;
    }

    if subtract_two <= 0 {
        return 0;
    }

    let result_feul = calc_fuel(subtract_two as u32, true);
    let result = subtract_two as u32 + result_feul;

    result
}