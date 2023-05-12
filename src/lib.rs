use std::{ops::Add, str::FromStr, any::type_name};
use num_traits::Zero;

pub struct StringCalculator {
    seperator: char
}

pub trait NumericSummable: Add + Zero + FromStr {}

impl<T: Add + Zero + FromStr> NumericSummable for T {}

impl StringCalculator {
    pub fn new() -> Self {
        StringCalculator {
            seperator: ','
        }
    }

    pub fn add<T>(mut self, numbers_as_string: String) -> T where T: NumericSummable {

        self.seperator = ','; 

        if numbers_as_string.starts_with("//"){
            self.seperator = numbers_as_string.chars().nth(2).unwrap();
        };

        let numbers_as_string = numbers_as_string.replace("\n", &self.seperator.to_string());

        if numbers_as_string.eq("") {
            return T::zero();
        }

        match numbers_as_string.find(self.seperator) {
            Some(_) => self.handle_multiple_numbers::<T>(numbers_as_string),
            None => self.handle_single_number::<T>(numbers_as_string),
        }
    }

    fn handle_multiple_numbers<T>(self, numbers_as_string: String) -> T where T: NumericSummable {
        let answer = self
            .map_string_to_collection_of(numbers_as_string)
            .into_iter()
            .reduce(|acc, parsed_number| acc + parsed_number);

        match answer {
            Some(sum) => sum,
            None => panic!("The iterator was empty"),
        }
    }

    fn handle_single_number<T>(self, number_as_string: String) -> T where T: NumericSummable {
        match number_as_string.parse::<T>() {
            Ok(sum) => sum,
            Err(_) => panic!("Could not parse value in given string to {}", type_name::<T>()),
        }
    }

    fn map_string_to_collection_of<T>(&self, numbers: String) -> Vec<T> where T: NumericSummable {
        numbers
            .split(self.seperator)
            .map(|num_string| self.parse_from_string::<T>(num_string))
            .collect()
    }

    fn parse_from_string<T>(&self, number_as_string: &str) -> T where T: NumericSummable {
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

    #[test]
    fn should_treat_newlines_as_seperators() {
        let string_calculator = StringCalculator::new();
        let response: u32 = string_calculator.add("2,2,1\n3,6,6\n3,4".to_string());
        assert_eq!(response, 27);
    }

    #[test]
    fn should_accept_custom_seperators() {
        let string_calculator = StringCalculator::new();
        let response: u32 = string_calculator.add("//;\n2;2;2\n2;2;2\n2;2".to_string());
        assert_eq!(response, 16);
    }
}
