use std::mem;

fn reverse_on_diagonal(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let len = matrix.len();
    
    for i in 0..matrix.len()/2 {
        let temp = matrix[i][i];
        matrix[i][i] = matrix[len-1-i][len-1-i];
        matrix[len-1-i][len-1-i] = temp;
        
        let temp = matrix[i][len-1-i];
        matrix[i][len-1-i] = matrix[len-1-i][i];
        matrix[len-1-i][i] = temp;
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
            vec![9,2,7],
            vec![4,5,6],
            vec![3,8,1],
        ];
 
        assert_eq!(reverse_on_diagonal(matrix), output);
    }
}
