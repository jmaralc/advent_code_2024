use std::env;
use std::fs::{read_to_string, File};
use anyhow::Result;
use csv::{ReaderBuilder, StringRecord};
fn main() {
    println!("DAY 2 --- part 1");
    day2_part1_solve("./data/training_data.csv");
    day2_part1_solve("./data/data.csv");
}

pub fn day2_part1_solve(file_path: &str,){
    let rows = read_lines(file_path);

    let count = rows.iter().filter(|&row| is_safe(row.to_vec())).count();
    println!("The result for part1 file: {} is : {}", file_path, count);
}
pub fn is_safe(row: Vec<i32>) -> bool{
    // println!("****Input {:?} ", row);
    is_all_decreasing(row.to_vec()) | is_all_increasing(row.to_vec())
}

fn read_lines(filename: &str) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let line_vector = line.split_whitespace().filter(|e| !e.is_empty()).map(str::to_string).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        result.push(line_vector)
    }
    // println!("{:?}", result);
    // println!("Read {} lines from file {:?}", result.len(), filename);
    result
}


pub fn decreasing_rule(n:&i32, n_plus_1: &i32) -> bool {
    // print!("n:{}, n+1:{}", n, n_plus_1);
    ((n- n_plus_1) >0)  & ((n- n_plus_1)<= 3)
}

pub fn is_all_decreasing(mut level: Vec<i32>) -> bool{
    let re = level.iter()
        .zip(level.iter().skip(1))
        .all(|(a, b)| decreasing_rule(a,b));
    // println!("For input {:?} is all decreasing {}", level, re);
    re
}

pub fn increasing_rule(n:&i32, n_plus_1: &i32) -> bool {
    // print!("n:{}, n+1:{}", n, n_plus_1);
    ((n_plus_1 -n )>0) & ((n_plus_1 - n)<=3)
}

pub fn is_all_increasing(mut level: Vec<i32>) -> bool{
    let re = level.iter()
        .zip(level.iter().skip(1))
        .all(|(a, b)| increasing_rule(a,b));
    // println!("For input {:?} is all increasing {}", level, re);
    re
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increasing_rule_should_return_false_when_n_is_smaller_than_n_plus_1() {
        // Given
        let n = 2;
        let n_plus_1 = 1;

        // When
        let result = increasing_rule(&n, &n_plus_1);

        // Then
        assert_eq!(result, false);
    }

    #[test]
    fn increasing_rule_should_return_false_when_n_is_larger_than_n_plus_1_in_more_than_three_units() {
        // Given
        let n = 1;
        let n_plus_1 = 5;

        // When
        let result = increasing_rule(&n, &n_plus_1);

        // Then
        assert_eq!(result, false);
    }


    #[test]
    fn decreasing_rule_should_return_false_when_n_is_smaller_than_n_plus_1() {
        // Given
        let n = 1;
        let n_plus_1 = 2;

        // When
        let result = decreasing_rule(&n, &n_plus_1);

        // Then
        assert_eq!(result, false);
    }

    #[test]
    fn decreasing_rule_should_return_false_when_n_is_larger_than_n_plus_1_in_more_than_three_units() {
        // Given
        let n = 5;
        let n_plus_1 = 1;

        // When
        let result = decreasing_rule(&n, &n_plus_1);

        // Then
        assert_eq!(result, false);
    }

    pub fn decreasing_rule(a:&i32, b: &i32) -> bool {
        a>b && (b - a).abs()<=3
    }
    #[test]
    fn is_decreasing_level_should_return_true_when_vector_is_all_decreasing() {
        // Given
        let vector = vec![7, 6, 4, 2, 1];

        // When
        let result = is_all_decreasing(vector);

        // Then
        assert!(result);
    }

    #[test]
    fn is_decreasing_level_should_return_false_when_vector_is_strictly_increasing() {
        // Given
        let vector = vec![1,2,7,8,9];

        // When
        let result = is_all_decreasing(vector);

        // Then
        assert_eq!(result, false);
    }

    #[test]
    fn is_decreasing_level_should_return_false_when_vector_is_not_strictly_decreasing() {
        // Given
        let vector = vec![8,6,4,4,1];

        // When
        let result = is_all_decreasing(vector);

        // Then
        assert_eq!(result, false);
    }

    #[test]
    fn is_decreasing_level_should_return_false_when_the_decrease_difference_is_larger_than_three() {
        // Given
        let vector = vec![9,7,6,2,1];

        // When
        let result = is_all_decreasing(vector);

        // Then
        assert_eq!(result, false);
    }

    #[test]
    fn is_increasing_level_should_return_true_when_vector_is_all_increasing() {
        // Given
        let vector = vec![1,3,6,7,9];

        // When
        let result = is_all_increasing(vector);

        // Then
        assert!(result);
    }
    #[test]
    fn is_increasing_level_should_return_false_when_vector_is_not_strictly_decreasing() {
        // Given
        let vector = vec![8,6,4,4,1];

        // When
        let result = is_all_increasing(vector);

        // Then
        assert_eq!(result, false);
    }
    #[test]
    fn is_increasing_level_should_return_false_when_the_decrease_difference_is_larger_than_three() {
        // Given
        let vector = vec![9,7,6,2,1];

        // When
        let result = is_all_increasing(vector);

        // Then
        assert_eq!(result, false);
    }

}
