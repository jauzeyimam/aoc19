use super::day_two;
use super::util;
use std::fs::File;
use std::io::{Error, ErrorKind};
use text_io::*;

pub struct DayFive;

impl super::Solution for DayFive {
    fn solve(&self) -> Result<String, Error> {
        let day_two = day_two::DayTwo;

        let input = util::read_comma_delim(File::open("input/5.txt")?)?;

        let mut answer_one: i64 = 0;
        let mut answer_two: i64 = 0;

        let f1 = || -> i64 { 1 };
        let f2 = || -> i64 { 5 };

        let g1 = |x: i64| answer_one = x;
        let g2 = |x: i64| answer_two = x;

        let _ = day_two.run_instructions(input.clone(), f1, g1);
        let _ = day_two.run_instructions(input.clone(), f2, g2);

        Ok(format!("Day 5a: {}\nDay 5b: {}", answer_one, answer_two))
    }
}
