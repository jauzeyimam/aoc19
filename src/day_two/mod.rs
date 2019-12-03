use super::util;
use std::fs::File;
use std::io::{Error, ErrorKind};

pub struct DayTwo;

impl super::Solution for DayTwo {
    fn solve(&self) -> Result<String, Error> {
        let input = util::read_comma_delim(File::open("input/2.txt")?)?;

        let result = format!(
            "Day 2a: {}\nDay 2b: {}",
            self.part_one(input.clone(), 12, 2)?,
            self.part_two(input.clone(), 19690720)?
        );

        Ok(result)
    }
}

impl DayTwo {
    fn part_one(&self, mut input: Vec<u64>, noun: u64, verb: u64) -> Result<u64, Error> {
        input[1] = noun;
        input[2] = verb;

        let result = self.run_instructions(input)?;

        Ok(result[0])
    }

    fn part_two(&self, input: Vec<u64>, expected_result: u64) -> Result<u64, Error> {
        for noun in 0..99 {
            for verb in 0..99 {
                let attempt = self.part_one(input.clone(), noun, verb)?;
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

    fn run_instructions(&self, mut input: Vec<u64>) -> Result<Vec<u64>, Error> {
        // println!("Input: {:?}", input);

        let mut x = 0;
        while x < input.len() {
            let op_code = input.get(x).unwrap();
            // println!("opcode: {}", op_code);
            match op_code {
                1 | 2 => {
                    let pos_one = *input.get(x + 1).unwrap();
                    let pos_two = *input.get(x + 2).unwrap();
                    let value_one = *input.get(pos_one as usize).unwrap();
                    let value_two = *input.get(pos_two as usize).unwrap();
                    let pos_move = *input.get(x + 3).unwrap();

                    let result;
                    match op_code {
                        1 => {
                            result = value_one + value_two;
                        }
                        2 => {
                            result = value_one * value_two;
                        }
                        _ => {
                            break;
                        }
                    }

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
}
