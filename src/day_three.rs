use super::util;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Error;
use std::iter::FromIterator;

pub struct DayThree;

#[derive(Hash, Debug)]
struct Point(isize, isize);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 + self.1 == other.0 + other.1
    }
}

impl Eq for Point {}

impl super::Solution for DayThree {
    fn solve(&self) -> Result<String, Error> {
        let input = util::read_comma_delim_str(File::open("input/3.txt")?)?;

        let map_a = self.get_points(input[0].clone());
        let map_b = self.get_points(input[1].clone());

        let key_set_a: HashSet<&Point> = HashSet::from_iter(map_a.keys());
        let key_set_b = HashSet::from_iter(map_b.keys());

        let intersection = key_set_a.intersection(&key_set_b);

        let p1 = intersection
            .clone()
            .map(|p| p.0.abs() + p.1.abs())
            .min()
            .unwrap();
        let p2 = intersection
            .clone()
            .map(|p| map_a.get(*p).unwrap_or(&0) + map_b.get(*p).unwrap_or(&0))
            .min()
            .unwrap_or(0);

        Ok(format!("Day 3a: {}\nDay 3b: {}", p1, p2))
    }
}

impl DayThree {
    fn get_points(&self, input: Vec<String>) -> HashMap<Point, u64> {
        let delta_x: HashMap<&str, isize> = [("L", -1), ("R", 1), ("U", 0), ("D", 0)]
            .iter()
            .cloned()
            .collect();
        let delta_y: HashMap<&str, isize> = [("L", 0), ("R", 0), ("U", 1), ("D", -1)]
            .iter()
            .cloned()
            .collect();

        let mut x = 0;
        let mut y = 0;
        let mut length = 0;

        let mut answer: HashMap<Point, u64> = HashMap::new();

        for cmd in input {
            let op = cmd.get(0..1).unwrap();
            let num: usize = cmd.get(1..).unwrap().parse().unwrap();

            assert!(delta_x.contains_key(op));

            for _ in 0..num {
                x += delta_x.get(op).unwrap();
                y += delta_y.get(op).unwrap();

                length += 1;

                let point = Point(x, y);

                if !answer.contains_key(&point) {
                    answer.insert(point, length);
                }
            }
        }

        answer
    }
}
