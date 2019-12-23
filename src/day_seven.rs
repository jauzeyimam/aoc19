use super::day_two;
use super::util;
use permutator::Permutation;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::rc::Rc;
use std::sync::mpsc::channel;
// use std::sync::mpsc::sync_channel;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct DaySeven;

impl super::Solution for DaySeven {
    fn solve(&self) -> Result<String, Error> {
        let input = util::read_comma_delim(File::open("input/7.txt")?)?;

        let part_1 = self.part_1(input.clone());
        let part_2 = self.part_2(input);

        Ok(format!("Day 7a: {}\nDay 7b: {}", part_1, part_2))
    }
}

impl DaySeven {
    fn part_1(&self, input: Vec<i64>) -> String {
        let day_two = day_two::DayTwo;

        let mut results: Vec<i64> = vec![];

        get_permutations(0, 5).iter().for_each(|p| {
            let input_a: RefCell<VecDeque<i64>> = RefCell::new(vec![p[0], 0].into_iter().collect());
            let input_b: RefCell<VecDeque<i64>> = RefCell::new(vec![p[1]].into_iter().collect());
            let input_c: RefCell<VecDeque<i64>> = RefCell::new(vec![p[2]].into_iter().collect());
            let input_d: RefCell<VecDeque<i64>> = RefCell::new(vec![p[3]].into_iter().collect());
            let input_e: RefCell<VecDeque<i64>> = RefCell::new(vec![p[4]].into_iter().collect());

            let f1 = || -> i64 {
                return input_a.borrow_mut().pop_front().unwrap();
            };
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

        max.to_string()
    }

    fn part_2(&self, input_orig: Vec<i64>) -> String {
        let (result_tx, result_rx) = channel();

        get_permutations(5, 10).iter().for_each(|p| {
            let (tx1, rx1) = channel();
            let (tx2, rx2) = channel();
            let (tx3, rx3) = channel();
            let (tx4, rx4) = channel();
            let (tx5, rx5) = channel();

            let f1 = move || -> i64 {
                let result = rx1.recv().unwrap();
                // println!("input 1: {}", result);
                result
            };
            let f2 = move || -> i64 {
                let result = rx2.recv().unwrap();
                // println!("input 2: {}", result);
                result
            };
            let f3 = move || -> i64 {
                let result = rx3.recv().unwrap();
                // println!("input 3: {}", result);
                result
            };
            let f4 = move || -> i64 {
                let result = rx4.recv().unwrap();
                // println!("input 4: {}", result);
                result
            };
            let f5 = move || -> i64 {
                let result = rx5.recv().unwrap();
                // println!("input 5: {}", result);
                result
            };

            let tx2_clone = tx2.clone();
            let g1 = move |x: i64| {
                // println!("output 1: {}", x);
                tx2_clone.send(x).unwrap();
            };

            let tx3_clone = tx3.clone();
            let g2 = move |x: i64| {
                // println!("output 2: {}", x);
                tx3_clone.send(x).unwrap();
            };

            let tx4_clone = tx4.clone();
            let g3 = move |x: i64| {
                // println!("output 3: {}", x);
                tx4_clone.send(x).unwrap();
            };

            let tx5_clone = tx5.clone();
            let g4 = move |x: i64| {
                // println!("output 4: {}", x);
                tx5_clone.send(x).unwrap();
            };

            let tx1_clone = tx1.clone();
            let result_tx = result_tx.clone();
            let g5 = move |x: i64| {
                // println!("output 5: {}", x);
                // result_tx.send(x).unwrap();
                let result = tx1_clone.send(x);
                match result {
                    Err(err) => result_tx.send(x).unwrap(),
                    _ => {}
                }
            };

            let input = input_orig.clone();
            let amp1 = thread::spawn(move || {
                let day_two = day_two::DayTwo;
                day_two.run_instructions(input, f1, g1);
            });

            let input = input_orig.clone();
            let amp2 = thread::spawn(move || {
                let day_two = day_two::DayTwo;
                day_two.run_instructions(input, f2, g2);
            });

            let input = input_orig.clone();
            let amp3 = thread::spawn(move || {
                let day_two = day_two::DayTwo;
                day_two.run_instructions(input, f3, g3);
            });

            let input = input_orig.clone();
            let amp4 = thread::spawn(move || {
                let day_two = day_two::DayTwo;
                day_two.run_instructions(input, f4, g4);
            });

            let input = input_orig.clone();
            let amp5 = thread::spawn(move || {
                let day_two = day_two::DayTwo;
                day_two.run_instructions(input, f5, g5);
            });

            tx1.send(p[0]).unwrap();
            tx1.send(0).unwrap();
            tx2.send(p[1]).unwrap();
            tx3.send(p[2]).unwrap();
            tx4.send(p[3]).unwrap();
            tx5.send(p[4]).unwrap();

            // wait for all to finish
            amp1.join();
            amp2.join();
            amp3.join();
            amp4.join();
            amp5.join();
        });

        drop(result_tx);

        let result_vec: Vec<i64> = result_rx.iter().collect();

        result_vec.iter().max().unwrap().to_string()
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
