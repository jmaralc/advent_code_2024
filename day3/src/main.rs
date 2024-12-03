use std::fs::read_to_string;
use regex::Regex;

fn main() {
    println!("DAY 3 --- part 1");
    day3_part1_solve("./data/training_data.txt");
    day3_part1_solve("./data/data.txt");

    println!("DAY 3 --- part 2");
    day3_part2_solve("./data/training_data2.txt");
    day3_part2_solve("./data/data.txt");
}

pub fn day3_part1_solve(file_path: &str,){
    let sequence = read_to_string(file_path).unwrap();

    let result = detect_mul(&sequence);
    println!("The result for part1 file: {} is : {}", file_path, result);
}

pub fn day3_part2_solve(file_path: &str,){
    let sequence = read_to_string(file_path).unwrap().replace('\n', " ");
    let beginning = detect_beginning(&sequence);
    let end = detect_end(&sequence);
    let chunks_mul:u32 = get_valid_chunks(&sequence).iter().map(|chunk| detect_mul(chunk)).sum();
    let result = chunks_mul + beginning + end;
    println!("The result for part2 file: {} is : {}", file_path, result);
}

fn get_valid_chunks(sequence: &str) -> Vec<String> {
    let re = Regex::new(r"do\(\)(.*?)don't\(\)").unwrap();
    let mut result: Vec<String> = vec![];
    for (_, [chunk]) in re.captures_iter(sequence).map(|c| c.extract()) {
        result.push(chunk.to_string());
    }
    // re.captures(sequence).unwrap().iter().map(|x| x.unwrap().as_str().to_string()).collect::<Vec<String>>()
    result
}

fn detect_mul(sequence: &str) -> u32  {
    let re = Regex::new(r"mul\((\d|\d{2}|\d{3}),(\d|\d{2}|\d{3})\)").unwrap();
    let mut result: Vec<u32>= vec![];
    for (_, [left_term, right_term]) in re.captures_iter(sequence).map(|c| c.extract()) {
        result.push(left_term.parse::<u32>().unwrap() * right_term.parse::<u32>().unwrap());
    }

    result.iter().sum()
}

fn detect_beginning(sequence: &str) -> u32{
    let beginning = sequence.split("don't()").into_iter().next().unwrap()
        .split("do_not").into_iter().next().unwrap();

    detect_mul(beginning)
}

fn detect_end(sequence: &str) -> u32{
    let end = sequence.split("do()").into_iter().last().unwrap();
    if end.split("don't()").count() >1 {
        return 0;
    }
    detect_mul(end)
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn detect_beginning_should_get_from_string_the_occurrences_of_mul_commands_before_any_dont() {
        // Given
        let sequence = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let expected_result = 8;

        // When
        let result = detect_beginning(sequence);

        // Then
        assert_eq!(result, expected_result);
    }

    #[test]
    fn detect_end_should_get_from_string_the_occurrences_of_mul_commands_after_last_do_that_do_not_have_dont_inside() {
        // Given
        let sequence = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let expected_result = 40;

        // When
        let result = detect_end(sequence);

        // Then
        assert_eq!(result, expected_result);
    }
    #[test]

    fn detect_end_should_return_zero_if_after_the_last_do_there_is_still_any_dont() {
        // Given
        let sequence = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))don't()";

        let expected_result = 0;

        // When
        let result = detect_end(sequence);

        // Then
        assert_eq!(result, expected_result);
    }

    #[test]
    fn get_valid_chunks_should_get_from_string_the_occurrences_of_mul_commands_before_any_dont() {
        // Given
        let sequence = "xmul(2,4)&mul[3,7]!^do()_mul(5,5)+mul(32,64](mul(11,8)undon't()?mul(8,5))";

        let expected_result = vec!["_mul(5,5)+mul(32,64](mul(11,8)un"];

        // When
        let result = get_valid_chunks(sequence);

        // Then
        assert_eq!(result, expected_result);
    }
}