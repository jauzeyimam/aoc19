use super::util;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind};
use text_io::*;

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
    fn part_one(&self, mut input: Vec<i64>, noun: i64, verb: i64) -> Result<i64, Error> {
        input[1] = noun;
        input[2] = verb;

        let f = || -> i64 { read!("{}\n") };
        let g = |x: i64| println!("{}", x);

        let result = self.run_instructions(input, f, g)?;

        Ok(*result.get(&0).unwrap())
    }

    fn part_two(&self, input: Vec<i64>, expected_result: i64) -> Result<i64, Error> {
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

    pub fn run_instructions<F, G>(
        &self,
        mut input: Vec<i64>,
        mut input_cmd: F,
        mut output_cmd: G,
    ) -> Result<HashMap<usize, i64>, Error>
    where
        F: FnMut() -> i64,
        G: FnMut(i64),
    {
        let mut input: HashMap<_, _> = input
            .iter()
            .enumerate()
            .map(|(k, v)| (k, *v))
            .into_iter()
            .collect();
        let mut register = 0;
        let mut x = 0;
        while x < input.len() {
            let op = input.get(&x).unwrap();

            let (op_code, mode_one, mode_two, mode_three) = parse_opcode(*op);

            match op_code {
                1 | 2 => {
                    let value_one = get_value(mode_one, &input, x + 1, register);
                    let value_two = get_value(mode_two, &input, x + 2, register);

                    // let pos_value = *input.get(&(x + 1)).unwrap();
                    // if mode_one == 1 {
                    //     value_one = pos_value;
                    // } else if mode_one == 2 {
                    //     value_one = *input.get(&((pos_value + register) as usize)).unwrap_or(&&0);
                    // } else {
                    //     value_one = *input.get(&(pos_value as usize)).unwrap_or(&&0);
                    // }

                    // let pos_value = *input.get(&(x + 2)).unwrap();
                    // if mode_two == 1 {
                    //     value_two = pos_value
                    // } else if mode_one == 2 {
                    //     value_two = *input.get(&((pos_value + register) as usize)).unwrap_or(&&0);
                    // } else {
                    //     value_two = *input.get(&(pos_value as usize)).unwrap_or(&&0);
                    // }

                    let pos_move = *input.get(&(x + 3)).unwrap_or(&&0);

                    let result;
                    match op_code {
                        1 => {
                            result = value_one + value_two;
                        }
                        2 => {
                            result = value_one * value_two;
                        }
                        _ => {
                            continue;
                        }
                    }

                    if mode_three == 0 {
                        input.insert(pos_move as usize, result);
                    } else {
                        input.insert((pos_move + register) as usize, result);
                    }
                    x += 4;
                }
                3 => {
                    let i: i64 = input_cmd();

                    // let value_one = get_value(mode_one, &input, x + 1, register);
                    let value_one;

                    let pos_value = *input.get(&(x + 1)).unwrap();
                    if mode_one == 2 {
                        value_one = pos_value + register;
                    } else {
                        value_one = pos_value;
                    }
                    input.insert(value_one as usize, i);
                    x += 2;
                }
                4 => {
                    let value_one = get_value(mode_one, &input, x + 1, register);

                    // let pos_value = *input.get(&(x + 1)).unwrap();
                    // if mode_one == 1 {
                    //     value_one = pos_value;
                    // } else if mode_one == 2 {
                    //     value_one = *input.get(&((pos_value + register) as usize)).unwrap_or(&&0);
                    // } else {
                    //     value_one = *input.get(&(pos_value as usize)).unwrap_or(&&0);
                    // }

                    output_cmd(value_one);
                    x += 2;
                }
                5 | 6 => {
                    let value_one = get_value(mode_one, &input, x + 1, register);
                    let value_two = get_value(mode_two, &input, x + 2, register);

                    // let pos_value = *input.get(&(x + 1)).unwrap();
                    // if mode_one == 1 {
                    //     value_one = pos_value;
                    // } else if mode_one == 2 {
                    //     value_one = *input.get(&((pos_value + register) as usize)).unwrap_or(&&0);
                    // } else {
                    //     value_one = *input.get(&(pos_value as usize)).unwrap_or(&&0);
                    // }

                    // let pos_value = *input.get(&(x + 2)).unwrap();
                    // if mode_two == 1 {
                    //     value_two = pos_value
                    // } else if mode_one == 2 {
                    //     value_two = *input.get(&((pos_value + register) as usize)).unwrap_or(&&0);
                    // } else {
                    //     value_two = *input.get(&(pos_value as usize)).unwrap_or(&&0);
                    // }

                    let set = match op_code {
                        5 => value_one != 0,
                        6 => value_one == 0,
                        _ => {
                            return Err(Error::new(ErrorKind::InvalidData, "Invalid Data!"));
                        }
                    };

                    if set {
                        x = value_two as usize;
                    } else {
                        x += 3;
                    }
                }
                7 | 8 => {
                    let value_one = get_value(mode_one, &input, x + 1, register);
                    let value_two = get_value(mode_two, &input, x + 2, register);

                    // let pos_value = *input.get(&(x + 1)).unwrap();
                    // if mode_one == 1 {
                    //     value_one = pos_value;
                    // } else if mode_one == 2 {
                    //     value_one = *input.get(&((pos_value + register) as usize)).unwrap_or(&&0);
                    // } else {
                    //     value_one = *input.get(&(pos_value as usize)).unwrap_or(&&0);
                    // }

                    // let pos_value = *input.get(&(x + 2)).unwrap();
                    // if mode_two == 1 {
                    //     value_two = pos_value
                    // } else if mode_one == 2 {
                    //     value_two = *input.get(&((pos_value + register) as usize)).unwrap_or(&&0);
                    // } else {
                    //     value_two = *input.get(&(pos_value as usize)).unwrap_or(&&0);
                    // }
                    let pos_move = *input.get(&(x + 3)).unwrap_or(&&0);

                    let set = match op_code {
                        7 => {
                            if value_one < value_two {
                                1
                            } else {
                                0
                            }
                        }
                        8 => {
                            if value_one == value_two {
                                1
                            } else {
                                0
                            }
                        }
                        _ => {
                            return Err(Error::new(ErrorKind::InvalidData, "Invalid Data!"));
                        }
                    };

                    if mode_three == 0 {
                        input.insert(pos_move as usize, set);
                    } else {
                        input.insert((pos_move + register) as usize, set);
                    }
                    x += 4;
                }
                9 => {
                    let value_one = get_value(mode_one, &input, x + 1, register);

                    // let pos_value = *input.get(&(x + 1)).unwrap();
                    // if mode_one == 1 {
                    //     value_one = pos_value;
                    // } else if mode_one == 2 {
                    //     value_one = *input.get(&((pos_value + register) as usize)).unwrap_or(&&0);
                    // } else {
                    //     value_one = *input.get(&(pos_value as usize)).unwrap_or(&&0);
                    // }

                    register += value_one;
                    x += 2;
                }
                99 => {
                    break;
                }
                _ => {
                    return Err(Error::new(ErrorKind::InvalidData, "Invalid Data!"));
                }
            }
        }

        Ok(input)
    }
}

fn get_value(mode: i64, map: &HashMap<usize, i64>, position: usize, register: i64) -> i64 {
    let value = map.get(&position).unwrap_or(&0);
    match mode {
        0 => return *map.get(&(*value as usize)).unwrap_or(&0),
        1 => return *value,
        2 => return *map.get(&((register + *value) as usize)).unwrap_or(&0),
        _ => {
            panic!("Unexpected mode!");
        }
    }
}

fn parse_opcode(op: i64) -> (i64, i64, i64, i64) {
    let op_code = op % 100;
    let mode_one = (op / 100) % 10;
    let mode_two = (op / 1000) % 10;
    let mode_three = (op / 10000) % 10;

    return (op_code, mode_one, mode_two, mode_three);
}

mod test {
    use super::*;

    #[test]
    fn test_mod_parsing() {
        let ops = parse_opcode(10199);
        assert_eq!(ops, (99, 1, 0, 1));

        let ops = parse_opcode(102);
        assert_eq!(ops, (2, 1, 0, 0));
    }
}
