use std::{ops::Add, str::FromStr};
use num_traits::Zero;

pub struct StringCalculator;

pub trait NumericSummable: Add + Zero + FromStr {}

impl StringCalculator {
    pub fn new() -> Self {
        StringCalculator {}
    }

    pub fn add<T: Add + Zero + FromStr>(self, numbers: String) -> T {
        if numbers.eq("") {
            return T::zero();
        }
        match numbers.find(",") {
            Some(_) => self.handle_multiple_numbers::<T>(numbers),
            None => self.handle_single_number::<T>(numbers),
        }
    }

    fn handle_multiple_numbers<T: Add + Zero + FromStr>(self, numbers_as_string: String) -> T {
        let answer = self
            .map_string_to_collection_of(numbers_as_string)
            .into_iter()
            .reduce(|acc, number| acc + number);

        match answer {
            Some(sum) => sum,
            None => panic!("The iterator was empty"),
        }
    }

    fn handle_single_number<T: Add + Zero + FromStr>(self, number_as_string: String) -> T {
        match number_as_string.parse::<T>() {
            Ok(sum) => sum,
            Err(_) => panic!("Could not parse values in given string to u32"),
        }
    }

    fn map_string_to_collection_of<T>(&self, numbers: String) -> Vec<T> where T: Add + Zero + FromStr {
        numbers
            .split(",")
            .map(|num_string| self.parse_from_string::<T>(num_string))
            .collect()
    }

    fn parse_from_string<T: Add + Zero + FromStr>(&self, number_as_string: &str) -> T {
        match number_as_string.parse::<T>() {
            Ok(number) => number,
            Err(_) => T::zero(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_0_when_passed_empty_string() {
        let string_calculator = StringCalculator::new();
        let response: u32 = string_calculator.add("".to_string());
        assert_eq!(response, 0);
    }

    #[test]
    fn should_return_1_when_passed_1() {
        let string_calculator = StringCalculator::new();
        let response: u32 = string_calculator.add("1".to_string());
        assert_eq!(response, 1);
    }

    #[test]
    fn should_return_2_when_passed_2() {
        let string_calculator = StringCalculator::new();
        let response: u32 = string_calculator.add("2".to_string());
        assert_eq!(response, 2);
    }

    #[test]
    fn should_return_2_when_passed_1_and_1() {
        let string_calculator = StringCalculator::new();
        let response: u32 = string_calculator.add("1,1".to_string());
        assert_eq!(response, 2);
    }

    #[test]
    fn should_return_3_when_passed_2_and_1() {
        let string_calculator = StringCalculator::new();
        let response: u32 = string_calculator.add("2,1".to_string());
        assert_eq!(response, 3);
    }

    #[test]
    fn should_return_4_when_passed_2_and_2() {
        let string_calculator = StringCalculator::new();
        let response: u32 = string_calculator.add("2,2".to_string());
        assert_eq!(response, 4);
    }

    #[test]
    fn should_return_5_when_passed_2_and_2_and_1() {
        let string_calculator = StringCalculator::new();
        let response: u32 = string_calculator.add("2,2,1".to_string());
        assert_eq!(response, 5);
    }
}
