type Matrix = Vec<Vec<i32>>;

fn crossing_sum(matrix: Matrix, r: i32, c: i32) -> i32 {
    let r = r as usize;
    let c = c as usize;

    sum_column(&matrix, c) + sum_row(&matrix, r) - matrix[r][c]
}

fn sum_column(matrix: &Matrix, c: usize) -> i32 {
    matrix.iter().map(|row| {
        row[c]
    }).sum()
}

fn sum_row(matrix: &Matrix, r: usize) -> i32 {
    matrix[r].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let matrix: Matrix = vec![
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
            vec![3, 3, 3, 3],
        ];
        assert_eq!(crossing_sum(matrix, 1, 3), 12);
    }
}
