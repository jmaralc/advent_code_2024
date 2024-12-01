fn main() {
    println!("Hello, world!");
}


pub fn short_list(mut list: Vec<i32>) -> Vec<i32> {
    list.sort();
    list
}
pub fn element_wise_distance(mut vec1: Vec<i32>, mut vec2: Vec<i32>,) -> Vec<i32> {
    vec1.iter().zip(vec2.iter()).map(|(&b, &v)| (b - v).abs()).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_list_should_order_a_list_of_location_id_in_ascending_order(){
        // Given
        let vector =  vec![3,4,2,1,3,3];
        let expected_result = vec![1,2,3,3,3,4];

        // When
        let result = short_list(vector);

        // Then
        assert_eq!(result, expected_result);
    }


    #[test]
    fn substract_vectors_should_return_the_element_wise_substraction() {
        // Given
        let vector1 =  vec![1,2,3,3,3,4];
        let vector2 =  vec![3,3,3,4,5,9];

        let expected_result = vec![2,1,0,1,2,5];

        // When
        let result = element_wise_distance(vector1, vector2);

        // Then
        assert_eq!(result, expected_result);
    }
}