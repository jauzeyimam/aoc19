use super::util;
use std::fs::File;
use std::io::{Error, ErrorKind};

#[allow(dead_code)]
pub fn solve() -> Result<(), Error> {
    let input = util::read_comma_delim(File::open("input/2.txt")?)?;

    let result_one = part_one(input.clone(), 12, 2)?;
    println!("Day 2a: {}", result_one);

    let result_two = part_two(input.clone(), 19690720)?;
    println!("Day 2b: {}", result_two);

    Ok(())
}

fn part_one(mut input: Vec<u64>, noun: u64, verb: u64) -> Result<u64, Error> {
    input[1] = noun;
    input[2] = verb;

    let result = run_instructions(input)?;

    Ok(result[0])
}

fn part_two(input: Vec<u64>, expected_result: u64) -> Result<u64, Error> {
    for noun in 0..99 {
        let mut attempt;
        for verb in 0..99 {
            attempt = part_one(input.clone(), noun, verb)?;
            if attempt == expected_result {

                return Ok(100 * noun + verb);
            }
        }
    }

    return Err(Error::new(
        ErrorKind::InvalidData,
        "Expected result not found in dataset",
    ));
}

fn run_instructions(mut input: Vec<u64>) -> Result<Vec<u64>, Error> {

    // println!("Input: {:?}", input);

    let mut x = 0;
    while x < input.len() {
        let op_code = input.get(x).unwrap();
        // println!("opcode: {}", op_code);
        match op_code {
            1 => {
                let pos_one = *input.get(x + 1).unwrap();
                let pos_two = *input.get(x + 2).unwrap();
                let value_one = *input.get(pos_one as usize).unwrap();
                let value_two = *input.get(pos_two as usize).unwrap();
                let pos_move = *input.get(x + 3).unwrap();

                let result = value_one + value_two;

                input[pos_move as usize] = result;
                x += 4;
            }
            2 => {
                let pos_one = *input.get(x + 1).unwrap();
                let pos_two = *input.get(x + 2).unwrap();
                let value_one = *input.get(pos_one as usize).unwrap();
                let value_two = *input.get(pos_two as usize).unwrap();
                let pos_move = *input.get(x + 3).unwrap();

                let result = value_one * value_two;

                input[pos_move as usize] = result;
                x += 4;
            }
            99 => {
                break;
            }
            _ => {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid Data!"));
            }
        }
        // println!("Result: {:?} | x: {}", input, x);
    }

    // println!("Final Result: {:?}", input);

    Ok(input)

}