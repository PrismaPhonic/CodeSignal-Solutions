fn swap_diagonals(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let len = matrix.len();
    
    for i in 0..matrix.len() {
        let temp = matrix[i][i];
        matrix[i][i] = matrix[i][len-1-i];
        matrix[i][len-1-i] = temp;
    }
    
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let matrix = vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9],
        ];

        let output = vec![
            vec![3,2,1],
            vec![4,5,6],
            vec![9,8,7],
        ];
 
        assert_eq!(swap_diagonals(matrix), output);
    }
}
