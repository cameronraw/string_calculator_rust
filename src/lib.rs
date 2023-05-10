pub struct StringCalculator;

impl StringCalculator {

    pub fn new() -> Self {
        StringCalculator {}
    }

    pub fn add(self, numbers: String) -> u32{
        if numbers.eq("") {
            return 0;
        }
        match numbers.find(",") {
            Some(_) => {
                self.handle_multiple_numbers(numbers)
            },
            None => self.handle_single_number(numbers)         
        }
    }

    fn handle_multiple_numbers(self, numbers_as_string: String) -> u32 {

        let answer = self.map_string_collection_to_u32(numbers_as_string)
            .into_iter()
            .reduce(|acc, number| acc + number);

        match answer {
            Some(sum) => sum,
            None => panic!("The iterator was empty"),
        }
    }

    fn handle_single_number(self, number_as_string: String) -> u32 {
        match number_as_string.parse::<u32>() {
            Ok(sum) => sum,
            Err(_) => panic!("Could not parse values in given string to u32"),
        }
    }

    fn map_string_collection_to_u32(&self, numbers: String) -> Vec<u32> {
        numbers.split(",")
            .map(|num_string| self.parse_u32_from_string(num_string))
            .collect()
    }

    fn parse_u32_from_string(&self, number_as_string: &str) -> u32 {
        match number_as_string.parse::<u32>() {
            Ok(number) => number,
            Err(_) => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_0_when_passed_empty_string() {
        let string_calculator = StringCalculator::new();
        let response = string_calculator.add("".to_string());
        assert_eq!(response, 0);
    }

    #[test]
    fn should_return_1_when_passed_1() {
        let string_calculator = StringCalculator::new();
        let response = string_calculator.add("1".to_string());
        assert_eq!(response, 1);
    }

    #[test]
    fn should_return_2_when_passed_2() {
        let string_calculator = StringCalculator::new();
        let response = string_calculator.add("2".to_string());
        assert_eq!(response, 2);
    }

    #[test]
    fn should_return_2_when_passed_1_and_1() {
        let string_calculator = StringCalculator::new();
        let response = string_calculator.add("1,1".to_string());
        assert_eq!(response, 2);
    }

    #[test]
    fn should_return_3_when_passed_2_and_1() {
        let string_calculator = StringCalculator::new();
        let response = string_calculator.add("2,1".to_string());
        assert_eq!(response, 3);
    }

    #[test]
    fn should_return_4_when_passed_2_and_2() {
        let string_calculator = StringCalculator::new();
        let response = string_calculator.add("2,2".to_string());
        assert_eq!(response, 4);
    }

    #[test]
    fn should_return_5_when_passed_2_and_2_and_1() {
        let string_calculator = StringCalculator::new();
        let response = string_calculator.add("2,2,1".to_string());
        assert_eq!(response, 5);
    }
}
