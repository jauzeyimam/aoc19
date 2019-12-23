use super::day_two;
use super::util;
use permutator::Permutation;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::rc::Rc;

pub struct DaySeven;

impl super::Solution for DaySeven {
    fn solve(&self) -> Result<String, Error> {
        let day_two = day_two::DayTwo;

        let input = util::read_comma_delim(File::open("input/7.txt")?)?;

        let mut results: Vec<i64> = vec![];

        get_permutations(0, 5).iter().for_each(|p| {
            let mut input_a: RefCell<VecDeque<i64>> =
                RefCell::new(vec![p[0], 0].into_iter().collect());
            let mut input_b: RefCell<VecDeque<i64>> =
                RefCell::new(vec![p[1]].into_iter().collect());
            let mut input_c: RefCell<VecDeque<i64>> =
                RefCell::new(vec![p[2]].into_iter().collect());
            let mut input_d: RefCell<VecDeque<i64>> =
                RefCell::new(vec![p[3]].into_iter().collect());
            let mut input_e: RefCell<VecDeque<i64>> =
                RefCell::new(vec![p[4]].into_iter().collect());

            let f1 = || -> i64 {
                return input_a.borrow_mut().pop_front().unwrap();
            }; //input 1 & 2 on Amp A
            let f2 = || -> i64 {
                return input_b.borrow_mut().pop_front().unwrap();
            };
            let f3 = || -> i64 {
                return input_c.borrow_mut().pop_front().unwrap();
            };
            let f4 = || -> i64 {
                return input_d.borrow_mut().pop_front().unwrap();
            };
            let f5 = || -> i64 {
                return input_e.borrow_mut().pop_front().unwrap();
            };

            let g1 = |x: i64| {
                input_b.borrow_mut().push_back(x);
                // println!("answer_one: {:?}", answer_one);
            };
            let g2 = |x: i64| {
                input_c.borrow_mut().push_back(x)
                // println!("answer_two: {:?}", answer_two);
            };
            let g3 = |x: i64| {
                input_d.borrow_mut().push_back(x)
                // println!("answer_three: {:?}", answer_three);
            };
            let g4 = |x: i64| {
                input_e.borrow_mut().push_back(x)
                // println!("answer_four: {:?}", answer_four);
            };
            let g5 = |x: i64| {
                results.push(x)
                // println!("answer_five: {:?}", answer_five);
            };

            let _ = day_two.run_instructions(input.clone(), f1, g1);
            let _ = day_two.run_instructions(input.clone(), f2, g2);
            let _ = day_two.run_instructions(input.clone(), f3, g3);
            let _ = day_two.run_instructions(input.clone(), f4, g4);
            let _ = day_two.run_instructions(input.clone(), f5, g5);
        });

        let max = results.iter().max().unwrap();

        println!("Max: {}", max);

        Ok("".to_string())
    }
}

fn get_permutations(from: i64, to: i64) -> Vec<[i64; 5]> {
    let mut permutations = Vec::new();
    for i1 in from..to {
        for i2 in (from..to).filter(|x| *x != i1) {
            for i3 in (from..to).filter(|x| *x != i2 && *x != i1) {
                for i4 in (from..to).filter(|x| *x != i3 && *x != i1 && *x != i2) {
                    for i5 in (from..to).filter(|x| *x != i4 && *x != i1 && *x != i2 && *x != i3) {
                        permutations.push([i1, i2, i3, i4, i5]);
                    }
                }
            }
        }
    }
    permutations
}
