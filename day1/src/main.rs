use std::env;
use std::fs::File;


fn main() {
    day1_part1_solve("./data/test_data.csv");
    day1_part1_solve("./data/training_data.csv");
    day1_part1_solve("./data/data.csv");

    day1_part2_solve("./data/test_data.csv");
    day1_part2_solve("./data/training_data.csv");
    day1_part2_solve("./data/data.csv");
}

pub fn day1_part1_solve(file_path: &str,){
    let has_headers = false;
    let delimiter = b',';

    let vectors = match read_location_id_file(file_path, has_headers, delimiter) {
        Ok(rows) => {
            // println!("\nTotal rows: {}", &rows.len());
            // println!("Done!");
            get_vectors(rows)
        }
        Err(e) => panic!(),
    };


    let vector1 = sort_vector(vectors.0);
    let vector2 = sort_vector(vectors.1);
    let result = sum_vector(element_wise_distance(vector1, vector2));

    println!("The result for part1 file: {} is : {}", file_path, result);
}

pub fn day1_part2_solve(file_path: &str,){
    let has_headers = false;
    let delimiter = b',';

    let vectors = match read_location_id_file(file_path, has_headers, delimiter) {
        Ok(rows) => {
            // println!("\nTotal rows: {}", &rows.len());
            // println!("Done!");
            get_vectors(rows)
        }
        Err(e) => panic!(),
    };


    let vector1 = sort_vector(vectors.0);
    let vector2 = sort_vector(vectors.1);
    let result = similarity_score(vector1, &vector2);

    println!("The result for part2 file: {} is : {}", file_path, result);
}

pub fn read_location_id_file(file_name: &str, has_headers: bool, delimiter: u8) -> Result<Vec<StringRecord>> {
    // Open the CSV file
    let dir = env::current_dir()?;
    let file = File::open(file_name)?;

    let mut reader = ReaderBuilder::new()
        .has_headers(has_headers)
        .delimiter(delimiter)
        .from_reader(file);

    Ok(reader.records()
        .filter_map(|row| row.ok())
        .collect())
}

pub fn get_vectors(rows: Vec<StringRecord>) -> (Vec<i32>, Vec<i32>){
    let mut vector1 = Vec::new();
    let mut vector2 = Vec::new();

    for row in &rows {
        vector1.push(row[0].parse::<i32>().unwrap());
        vector2.push(row[1].parse::<i32>().unwrap());
    }

    (vector1, vector2)
}

pub fn sort_vector(mut list: Vec<i32>) -> Vec<i32> {
    list.sort();
    list
}
pub fn element_wise_distance(vec1: Vec<i32>, vec2: Vec<i32>,) -> Vec<i32> {
    vec1.iter().zip(vec2.iter()).map(|(&b, &v)| (b - v).abs()).collect()
}

pub fn similarity_score(vec1: Vec<i32>, vec2: &Vec<i32>,) -> i32 {
    vec1.iter().map(|b| b* occurences_in_vector(b, &vec2)).sum()
}

pub fn sum_vector(vec: Vec<i32>) -> i32 {
    vec.iter().sum()
}

pub fn occurences_in_vector(value: &i32, vector: &Vec<i32>,) -> i32 {
    vector.iter().filter(|(&b)| b==value ).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn similarity_score_should_return_the_similarity_of_two_vectors() {
        // Given
        let vector1 =  vec![1,2,3,3,3,4];
        let vector2 =  vec![3,3,3,4,5,9];

        let expected_result = 31;

        // When
        let result = similarity_score(vector1, &vector2);

        // Then
        assert_eq!(result, expected_result);
    }

    #[test]
    fn occurences_in_vector_should_return_the_amount_of_times_the_value_is_in_the_vector() {
        // Given
        let value =  3;
        let vector2 =  vec![3,3,3,4,5,9];

        let expected_result = 3;

        // When
        let result = occurences_in_vector(&value, &vector2);

        // Then
        assert_eq!(result, expected_result);
    }

    #[test]
    fn short_list_should_order_a_list_of_location_id_in_ascending_order(){
        // Given
        let vector =  vec![3,4,2,1,3,3];
        let expected_result = vec![1,2,3,3,3,4];

        // When
        let result = sort_vector(vector);

        // Then
        assert_eq!(result, expected_result);
    }


    #[test]
    fn element_wise_distance_should_return_the_element_wise_substraction() {
        // Given
        let vector1 =  vec![1,2,3,3,3,4];
        let vector2 =  vec![3,3,3,4,5,9];

        let expected_result = vec![2,1,0,1,2,5];

        // When
        let result = element_wise_distance(vector1, vector2);

        // Then
        assert_eq!(result, expected_result);
    }

    #[test]
    fn sum_vector_should_sum_all_the_elements_of_a_vector() {
        // Given
        let vector =  vec![2,1,0,1,2,5];

        let expected_result = 11;

        // When
        let result = sum_vector(vector);

        // Then
        assert_eq!(result, expected_result);
    }

    #[test]
    fn read_location_id_file_should_read_csv_file(){
        // Given
        let file_path =  "./data/test_data.csv";
        let has_headers = false;
        let delimiter = b',';

        // When

        // Then
        assert!( read_location_id_file(file_path, has_headers, delimiter).is_ok());
    }

    #[test]
    fn get_vectors_should_return_two_vectors_with_the_content_of_the_string_record(){
        // Given
        let vector =  vec![
            StringRecord::from(vec!["1", "2"]),
            StringRecord::from(vec!["3", "4"]),
        ];

        let expected_result = vec![1, 3];

        // When
        let result = get_vectors(vector);

        // Then
        assert_eq!(result.0, expected_result);
    }
}