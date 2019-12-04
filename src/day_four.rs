use std::io::Error;

pub struct DayFour;

impl super::Solution for DayFour {
    fn solve(&self) -> Result<String, Error> {
        let start = 125730;
        let end = 579381;

        let mut a = 0;
        let mut b = 0;
        for i in start..end {
            if self.meets_criteria(i.to_string(), false) {
                a += 1;
            }
            if self.meets_criteria(i.to_string(), true) {
                b += 1;
            }
        }

        Ok(format!("Day 4a: {}\nDay 4b: {}", a, b))
    }
}

impl DayFour {
    fn meets_criteria(&self, input: String, no_adj: bool) -> bool {
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

            if no_adj {
                if count == 2 {
                    found = true;
                }
            } else if count >= 2 {
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
    fn test_meets_criteria_111111() {
        let day_four = DayFour;
        assert_eq!(day_four.meets_criteria(String::from("111111"), false), true);
    }

    #[test]
    fn test_meets_criteria_223450() {
        let day_four = DayFour;
        assert_eq!(
            day_four.meets_criteria(String::from("223450"), false),
            false
        );
    }

    #[test]
    fn test_meets_criteria_123789() {
        let day_four = DayFour;
        assert_eq!(
            day_four.meets_criteria(String::from("123789"), false),
            false
        );
    }

    #[test]
    fn test_meets_criteria_112233() {
        let day_four = DayFour;
        assert_eq!(day_four.meets_criteria(String::from("112233"), true), true);
    }

    #[test]
    fn test_meets_criteria_123444() {
        let day_four = DayFour;
        assert_eq!(day_four.meets_criteria(String::from("123444"), true), false);
    }

    #[test]
    fn test_meets_criteria_111122() {
        let day_four = DayFour;
        assert_eq!(day_four.meets_criteria(String::from("111122"), true), true);
    }
}
