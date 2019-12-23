use std::io::Error;

pub struct DayFour;

impl super::Solution for DayFour {
    fn solve(&self) -> Result<String, Error> {
        let start = 125730;
        let end = 579381;

        let mut a = 0;
        let mut b = 0;
        let fn_a = |x: i32| -> bool { x >= 2 };
        let fn_b = |x: i32| -> bool { x == 2 };

        for i in start..end {
            if self.meets_criteria(i.to_string(), fn_a) {
                a += 1;
            }

            if self.meets_criteria(i.to_string(), fn_b) {
                b += 1;
            }
        }

        Ok(format!("Day 4a: {}\nDay 4b: {}", a, b))
    }
}

impl DayFour {
    fn meets_criteria<F>(&self, input: String, allowed_adj: F) -> bool
    where
        F: Fn(i32) -> bool,
    {
        // 1) It is a six-digit number.
        if input.len() != 6 {
            return false;
        }
        // 2) The value is within the range given in your puzzle input.
        // implicitly true ^

        let mut found = false;
        let mut iter = input.chars().peekable();
        while let Some(current) = iter.next() {
            let mut count = 1;
            while let Some(next) = iter.peek() {
                if *next == current {
                    count += 1;
                    iter.next();
                } else if *next < current {
                    // 4) Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
                    return false;
                } else {
                    break;
                }
            }

            if allowed_adj(count) {
                found = true;
            }
        }
        if !found {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meets_criteria() {
        let day_four = DayFour;
        assert_eq!(
            day_four.meets_criteria(String::from("111111"), |x: i32| -> bool { x >= 2 }),
            true
        );
        assert_eq!(
            day_four.meets_criteria(String::from("223450"), |x: i32| -> bool { x >= 2 }),
            false
        );
        assert_eq!(
            day_four.meets_criteria(String::from("123789"), |x: i32| -> bool { x >= 2 }),
            false
        );
        assert_eq!(
            day_four.meets_criteria(String::from("112233"), |x: i32| -> bool { x == 2 }),
            true
        );
        assert_eq!(
            day_four.meets_criteria(String::from("123444"), |x: i32| -> bool { x == 2 }),
            false
        );
        assert_eq!(
            day_four.meets_criteria(String::from("111122"), |x: i32| -> bool { x == 2 }),
            true
        );
    }
}
