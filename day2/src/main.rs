fn main() {
    println!("Hello, world!");
}


pub fn is_decreasing(mut level: Vec<i32>) -> bool{
    level.iter()
        .zip(level.iter().skip(1))
        .all(|(a, b)| a>b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_decreasing_level_should_return_true_when_vector_is_decreasing() {
        // Given
        let vector = vec![7, 6, 4, 2, 1];

        // When
        let result = is_decreasing(vector);

        // Then
        assert!(result);
    }

    #[test]
    fn is_decreasing_level_should_return_false_when_vector_is_strictly_increasing() {
        // Given
        let vector = vec![1,2,7,8,9];

        // When
        let result = is_decreasing(vector);

        // Then
        assert_eq!(result, false);
    }

    #[test]
    fn is_decreasing_level_should_return_false_when_vector_is_not_strictly_decreasing() {
        // Given
        let vector = vec![8,6,4,4,1];

        // When
        let result = is_decreasing(vector);

        // Then
        assert_eq!(result, false);
    }
}
