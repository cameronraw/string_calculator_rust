use num_traits::{cast, zero, NumCast, Zero};
use std::{any::type_name, ops::Add, str::FromStr};

pub struct StringCalculator {
    seperator: char,
}

pub trait NumericSummable: Add + Zero + FromStr + PartialOrd + NumCast {}

impl<T: Add + Zero + FromStr + PartialOrd + NumCast> NumericSummable for T {}

impl StringCalculator {
    pub fn new() -> Self {
        StringCalculator { seperator: ',' }
    }

    pub fn add<T>(mut self, numbers_as_string: String) -> T
    where
        T: NumericSummable,
    {
        self.seperator = ',';

        if numbers_as_string.starts_with("//") {
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

    fn handle_multiple_numbers<T>(self, numbers_as_string: String) -> T
    where
        T: NumericSummable,
    {
        let answer = self
            .map_string_to_number_vec(numbers_as_string)
            .into_iter()
            .reduce(|acc, parsed_number| acc + parsed_number);

        match answer {
            Some(sum) => sum,
            None => panic!("The iterator was empty"),
        }
    }

    fn handle_single_number<T>(self, number_as_string: String) -> T
    where
        T: NumericSummable,
    {
        match number_as_string.parse::<T>() {
            Ok(sum) => sum,
            Err(_) => panic!(
                "Could not parse value in given string to {}",
                type_name::<T>()
            ),
        }
    }

    fn map_string_to_number_vec<T>(&self, numbers: String) -> Vec<T>
    where
        T: NumericSummable,
    {
        let mut error_state = false;
        let mut negative_numbers = Vec::<&str>::new();

        let number_vec = numbers
            .split(self.seperator)
            .map(|num_string| {
                if num_string.contains('-') {
                    error_state = true;
                    negative_numbers.push(num_string)
                }
                self.parse_validated_number(num_string)
            })
            .collect();

        if error_state {
            panic!(
                "Negative numbers not allowed: {}",
                negative_numbers.join(" ")
            );
        }

        number_vec
    }

    fn parse_validated_number<T>(&self, num_string: &str) -> T
    where
        T: NumericSummable,
    {
        match cast::<u32, T>(1000) {
            Some(thousand) => {
                let parsed_number = self.parse_from_string::<T>(num_string);
                if parsed_number <= thousand {
                    return parsed_number;
                }
                zero()
            }
            None => panic!("Value 1000 could not be parsed into desired type"),
        }
    }

    fn parse_from_string<T>(&self, number_as_string: &str) -> T
    where
        T: NumericSummable,
    {
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

    #[test]
    #[should_panic(expected = "Negative numbers not allowed: -2 -4 -5")]
    fn should_not_accept_negative_numbers() {
        let string_calculator = StringCalculator::new();
        string_calculator.add::<u32>("1,-2,3,-4,-5".to_string());
    }

    #[test]
    fn should_ignore_numbers_larger_than_1000() {
        let string_calculator = StringCalculator::new();
        let response: u32 = string_calculator.add("1001,35".to_string());
        assert_eq!(response, 35);
    }
}
