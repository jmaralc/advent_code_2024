fn main() {
    println!("Hello, world!");
}


pub fn short_list(mut list: Vec<i32>) -> Vec<i32> {
    list.sort();
    list
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_list_should_order_a_list_of_location_id_in_ascending_order() {
        let vector =  vec![3,4,2,1,3,3];
        let expected_result = vec![1,2,3,3,3,4];
        let result = short_list(vector);
        assert_eq!(result, expected_result);
    }
}