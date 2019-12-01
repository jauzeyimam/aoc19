use math::round;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub fn solve_one() -> Result<u32, Error> {

    let input = read(File::open("input/1.txt")?)?;

    let mut sum: u32 = 0;

    for number in input {
        let feul = calc_feul(number as u32);
        sum += feul;
    }

    Ok(sum)
}

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn calc_feul(mass: u32) -> u32 {
    let divide_by_three = mass as f32 / 3.0;
    let floored = round::floor(divide_by_three as f64, 0) as i32;
    let subtract_two = floored - 2;

    if subtract_two <= 0 {
        return 0;
    }

    let result_feul = calc_feul(subtract_two as u32);
    let result = subtract_two as u32 + result_feul;

    result
}