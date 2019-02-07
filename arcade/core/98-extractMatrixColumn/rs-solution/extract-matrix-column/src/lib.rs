fn extract_matrix_column(matrix: Vec<Vec<i32>>, column: i32) -> Vec<i32> {
    matrix.iter().map(|row| {
        row[column as usize]
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let matrix = vec![
            vec![1,1,1,2],
            vec![0,5,0,4],
            vec![2,1,3,6]];
        assert_eq!(extract_matrix_column(matrix, 2), vec![1, 0, 3]);
    }
}
