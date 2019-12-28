use super::day_two;
use super::util;
use std::fs::File;
use std::io::{Error, ErrorKind};

pub struct DayNine;

impl super::Solution for DayNine {
    fn solve(&self) -> Result<String, Error> {
        let input = util::read_comma_delim(File::open("input/9.txt")?)?;

        let part_one = self.part_one(input.clone());
        let part_two = self.part_two(input);

        Ok(format!("Day 9a: {}\nDay 9b: {}", part_one, part_two))
    }
}

impl DayNine {
    fn part_one(&self, input: Vec<i64>) -> String {
        let day_two = day_two::DayTwo;
        let mut result = "".to_string();

        let input_fn = || -> i64 { 1 };

        let output_fn = |x: i64| {
            result = x.to_string();
        };

        let _ = day_two.run_instructions(input, input_fn, output_fn);

        // println!("Out: {:?}", out);

        result
    }

    fn part_two(&self, input: Vec<i64>) -> String {
        let day_two = day_two::DayTwo;
        let mut result = "".to_string();

        let input_fn = || -> i64 { 2 };

        let output_fn = |x: i64| {
            result = x.to_string();
        };

        let _ = day_two.run_instructions(input, input_fn, output_fn);

        // println!("Out: {:?}", out);
        result
    }
}
