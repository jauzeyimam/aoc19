use super::util;
use std::fmt::Write;
use std::fs::File;
use std::io::Error;

pub struct DayEight;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

const PIXELS: [&str; 3] = ["  ", "██", "  "];

impl super::Solution for DayEight {
    fn solve(&self) -> Result<String, Error> {
        let input = util::read_list_of_ints(File::open("input/8.txt")?)?;

        let part_1 = self.part_1(input.clone());
        let part_2 = self.part_2(input);

        Ok(format!(
            "Day 8a: {}\nDay 8b: \n{}",
            part_1,
            part_2
                .chunks(WIDTH)
                .map(join)
                .collect::<Vec<String>>()
                .join("\n")
        ))
    }
}

fn join(a: &[u32]) -> String {
    a.iter().fold(String::new(), |mut s, &n| {
        write!(s, "{}", PIXELS.get(n as usize).unwrap()).ok();
        s
    })
}

impl DayEight {
    fn part_1(&self, input: Vec<u32>) -> usize {
        let mut index = 1;

        let mut min_zeroes = WIDTH * HEIGHT + 1;
        let mut ones_times_twos = 0;
        for window in input.chunks(WIDTH * HEIGHT) {
            let zeroes = window.iter().filter(|&x| *x == 0).count();
            // println!("index: {} zeroes: {}", index, zeroes);
            if zeroes <= min_zeroes {
                min_zeroes = zeroes;
                let ones = window.iter().filter(|&x| *x == 1).count();
                let twos = window.iter().filter(|&x| *x == 2).count();
                ones_times_twos = ones * twos;
                // println!("index: {}, new result: {}", index, ones_times_twos);
            }
            index += 1;
        }

        ones_times_twos
    }

    fn part_2(&self, input: Vec<u32>) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();
        let iter = input.chunks(HEIGHT * WIDTH);

        for window in iter {
            if result.is_empty() {
                let mut window_vec = window.to_vec();
                result.append(&mut window_vec);
                // println!("{:?}", result);
                // break;
                continue;
            }

            for (i, element) in window.iter().enumerate() {
                let curr_element = result.get(i).unwrap();
                match (curr_element, element) {
                    (2, _) => {
                        // println!("overwriting {} with {}", element, curr_element);
                        result[i] = *element;
                    }
                    (1, _) | (0, _) => {}
                    (_, _) => {}
                }
            }
        }

        result
    }
}
