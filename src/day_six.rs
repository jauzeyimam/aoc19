use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader, Read};

pub struct DaySix;

impl super::Solution for DaySix {
    fn solve(&self) -> Result<String, Error> {
        let (orbits, reverse) = self.read_orbit_delim(File::open("input/6.txt")?);

        let n: u32 = orbits
            .keys()
            .map(|x| self.get_total_paths_from_node(String::from(x), &orbits))
            .sum();

        let m = self.min_jumps(reverse, "YOU".to_string(), "SAN".to_string());

        Ok(format!("Day 6a: {}\nDay 6b {}", n, m))
    }
}

impl DaySix {
    fn get_total_paths_from_node(&self, input: String, map: &HashMap<String, Vec<String>>) -> u32 {
        let node = map.get(&input);

        match node {
            None => {
                return 0;
            }
            Some(vec) => {
                let mut x = vec.len() as u32;
                for value in vec {
                    let count = self.get_total_paths_from_node(value.clone(), map);
                    x += count;
                    // println!("Count for {}: {}", value, count);
                    // println!("Total count for {}: {}", input, x);
                }
                return x;
            }
        }
    }

    fn list_of_parents(&self, map: &HashMap<String, String>, from: String) -> Vec<String> {
        let mut current = from;
        let mut parents = Vec::new();

        while current != "COM" {
            // println!("Current: {}", current);
            if let Some(parent) = map.get(&current) {
                // println!("Parent: {}", parent.clone());
                parents.push(parent.clone());
                current = parent.clone();
            }

            // println!("Parent map: {:?}", parents);
        }

        parents
    }

    fn common_element(&self, from: Vec<String>, to: Vec<String>) -> (usize, usize) {
        let to_map: HashMap<String, usize> = to
            .into_iter()
            .enumerate()
            .map(|(i, item)| (item, i))
            .collect();

        for (i, item) in from.iter().enumerate() {
            if let Some(j) = to_map.get(item) {
                return (i, *j);
            }
        }

        (0, 0)
    }

    fn min_jumps(&self, map: HashMap<String, String>, from: String, to: String) -> usize {
        println!("Calculating min jumps");

        let parents_from = self.list_of_parents(&map, from);
        let parents_to = self.list_of_parents(&map, to);

        let (steps_from, steps_to) = self.common_element(parents_from, parents_to);

        steps_from + steps_to
    }

    fn read_orbit_delim<R: Read>(
        &self,
        io: R,
    ) -> (HashMap<String, Vec<String>>, HashMap<String, String>) {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let mut rev: HashMap<String, String> = HashMap::new();

        let br = BufReader::new(io);
        for value in br.lines() {
            let value = value.unwrap();
            let split: Vec<&str> = value.split(")").collect();

            let curr_value = map.get_mut(&split[0].to_string());

            match curr_value {
                None => {
                    let parent = split[0].to_string();
                    let child = split[1].to_string();
                    rev.insert(child.clone(), parent.clone());
                    let x = vec![child];
                    map.insert(parent, x);
                }
                Some(x) => {
                    let parent = split[0].to_string();
                    let child = split[1].to_string();
                    rev.insert(child.clone(), parent);
                    x.push(child);
                }
            };
        }

        // println!("map: {:?}", map);
        // println!("rev: {:?}", rev);

        (map, rev)
    }
}
