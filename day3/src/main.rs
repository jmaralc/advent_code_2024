use std::fs::read_to_string;
use regex::Regex;

fn main() {
    println!("DAY 3 --- part 1");
    day3_part1_solve("./data/training_data.txt");
    day3_part1_solve("./data/data.txt");
}

pub fn day3_part1_solve(file_path: &str,){
    let sequence = read_to_string(file_path).unwrap();

    let result = detect_mul(&sequence);
    println!("The result for part1 file: {} is : {}", file_path, result);
}


fn detect_mul(sequence: &str) -> u32  {
    let re = Regex::new(r"mul\((\d|\d{2}|\d{3}),(\d|\d{2}|\d{3})\)").unwrap();
    let mut result: Vec<u32>= vec![];
    for (_, [left_term, right_term]) in re.captures_iter(sequence).map(|c| c.extract()) {
        result.push(left_term.parse::<u32>().unwrap() * right_term.parse::<u32>().unwrap());
    }

    result.iter().sum()
}

pub fn apply_mul(mul_command: &String) -> u32 {
    let re = Regex::new(r"(\d+),(\d+)").unwrap();
    let mut result: u32= 0;
    for (_, [left_term, right_term]) in re.captures_iter(mul_command).map(|c| c.extract()) {
        result = left_term.parse::<u32>().unwrap() * right_term.parse::<u32>().unwrap();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_mull_should_returns_the_multiplication_of_a_valid_mul_command() {
        // Given
        let command = "mul(2,4)".to_string();

        // When
        let result = apply_mul(&command);

        // Then
        assert_eq!(result, 8);
    }

    #[test]
    fn apply_mull_should_returns_the_multiplication_of_a_valid_mul_command_with_numbers_of_three_digits() {
        // Given
        let command = "mul(824,660)".to_string();

        // When
        let result = apply_mul(&command);

        // Then
        assert_eq!(result, 824*660);
    }

    #[test]
    fn detect_mul_should_get_from_string_the_occurrences_of_mul_commands() {
        // Given
        let sequence = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let expected_result = 161;

        // When
        let result = detect_mul(sequence);

        // Then
        assert_eq!(result, expected_result);
    }
}