use super::util;
use std::fs::File;
use std::io::Error;

pub struct DayThree;

impl super::Solution for DayThree {
    fn solve(&self) -> Result<String, Error> {
        let size: usize = 40000;
        let start: usize = size / 2;

        let mut distance = std::usize::MAX;
        let mut least_steps = std::usize::MAX;

        let mut matrix = vec![vec![0; size]; size];

        let input = util::read_comma_delim_str(File::open("input/3.txt")?)?;

        let step_size = 1000000000;
        let mut line = 1;
        loop {
            let instruction;
            match input.get(line - 1) {
                Some(x) => instruction = x,
                None => break,
            }
            let mut steps = line * step_size;
            let mut current_x = start;
            let mut current_y = start;
            for val in instruction {
                let op = val.get(0..1).unwrap();
                let num: usize = val.get(1..).unwrap().parse().unwrap();
                // println!("Op: {} | Num: {}", op, num);
                match op {
                    "U" => {
                        let finish = current_y + num;
                        while current_y < finish {
                            steps += 1;
                            current_y += 1;
                            let current = matrix[current_y][current_x];
                            if current == 0 {
                                matrix[current_y][current_x] = steps;
                            } else if current % (line * step_size) > step_size {
                                if current % (3 * step_size) < step_size {
                                    // don't overwrite if it's already a sum --
                                    // will def be less than what you're writing
                                    break;
                                }
                                let curr_steps = current + steps;
                                // println!(
                                //     "X: {} | Y: {} | Curr: {} | CurrSteps: {}",
                                //     current_x, current_y, current, curr_steps
                                // );

                                matrix[current_y][current_x] = curr_steps;
                                if curr_steps < least_steps {
                                    least_steps = curr_steps;
                                }
                                let x_dist: isize = current_x as isize - start as isize;
                                let y_dist: isize = current_y as isize - start as isize;
                                let curr_dist = x_dist.abs() as usize + y_dist.abs() as usize;
                                if curr_dist < distance {
                                    distance = curr_dist;
                                    // println!("Distance changed: {}", distance);
                                }
                            }
                        }
                    }
                    "D" => {
                        let finish = current_y - num;
                        while current_y > finish {
                            steps += 1;
                            current_y -= 1;
                            let current = matrix[current_y][current_x];
                            if current == 0 {
                                matrix[current_y][current_x] = steps;
                            } else if current % (line * step_size) > step_size {
                                if current % (3 * step_size) < step_size {
                                    break;
                                }
                                let curr_steps = current + steps;
                                // println!(
                                //     "X: {} | Y: {} | Curr: {} | CurrSteps: {}",
                                //     current_x, current_y, current, curr_steps
                                // );

                                matrix[current_y][current_x] = curr_steps;
                                if curr_steps < least_steps {
                                    least_steps = curr_steps;
                                }
                                // println!("X: {} | Y: {}", current_x, current_y);
                                let x_dist: isize = current_x as isize - start as isize;
                                let y_dist: isize = current_y as isize - start as isize;
                                let curr_dist = x_dist.abs() as usize + y_dist.abs() as usize;
                                if curr_dist < distance {
                                    distance = curr_dist;
                                    // println!("Distance changed: {}", distance);
                                }
                            }
                        }
                    }
                    "R" => {
                        let finish = current_x + num;
                        while current_x < finish {
                            steps += 1;

                            current_x += 1;
                            let current = matrix[current_y][current_x];
                            if current == 0 {
                                matrix[current_y][current_x] = steps;
                            } else if current % (line * step_size) > step_size {
                                if current % (3 * step_size) < step_size {
                                    break;
                                }
                                let curr_steps = current + steps;
                                // println!(
                                //     "X: {} | Y: {} | Curr: {} | CurrSteps: {}",
                                //     current_x, current_y, current, curr_steps
                                // );
                                matrix[current_y][current_x] = curr_steps;
                                if curr_steps < least_steps {
                                    least_steps = curr_steps;
                                }
                                let x_dist: isize = current_x as isize - start as isize;
                                let y_dist: isize = current_y as isize - start as isize;
                                let curr_dist = x_dist.abs() as usize + y_dist.abs() as usize;
                                if curr_dist < distance {
                                    distance = curr_dist;
                                    // println!("Distance changed: {}", distance);
                                }
                            }
                        }
                    }
                    "L" => {
                        let finish = current_x - num;
                        while current_x > finish {
                            steps += 1;

                            current_x -= 1;
                            let current = matrix[current_y][current_x];
                            if current == 0 {
                                matrix[current_y][current_x] = current + steps;
                            } else if current % (line * step_size) > step_size {
                                if current % (3 * step_size) < step_size {
                                    break;
                                }
                                let curr_steps = current + steps;
                                // println!(
                                //     "X: {} | Y: {} | Curr: {} | CurrSteps: {}",
                                //     current_x, current_y, current, curr_steps
                                // );
                                matrix[current_y][current_x] = curr_steps;
                                if curr_steps < least_steps {
                                    least_steps = curr_steps;
                                }
                                let x_dist: isize = current_x as isize - start as isize;
                                let y_dist: isize = current_y as isize - start as isize;
                                let curr_dist = x_dist.abs() as usize + y_dist.abs() as usize;
                                if curr_dist < distance {
                                    distance = curr_dist;
                                    // println!("Distance changed: {}", distance);
                                }
                            }
                        }
                    }
                    _ => {
                        // println!("Something unexpected happened!");
                    }
                }
            }
            line += 1;
        }

        // print_matrix(matrix.clone());

        // println!("Closest distance: {}", distance);
        let least_steps = least_steps % (3 * step_size);
        // println!("Least steps: {}", least_steps);

        Ok(format!("Day 3a: {}\nDay 3b: {}", distance, least_steps))
    }
}

#[allow(dead_code)]
fn print_matrix(matrix: Vec<Vec<usize>>) {
    for row in matrix {
        // println!("{:?}", row);
    }
}
