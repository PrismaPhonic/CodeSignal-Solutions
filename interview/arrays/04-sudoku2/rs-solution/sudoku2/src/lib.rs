#![feature(test)]

extern crate test;

use std::collections::HashSet;

type Matrix = Vec<Vec<char>>;

fn sudoku2(matrix: &Matrix) -> bool {
    check_rows_cols(matrix) && check_sub_matrices(matrix)
}

fn check_rows_cols(matrix: &Matrix) -> bool {
    for i in 0..9 {
        let mut row_counts = HashSet::with_capacity(9);
        let mut col_counts = HashSet::with_capacity(9);
        for j in 0..9 {
            let e = matrix[i][j];
            if e != '.' {
                if !row_counts.insert(e) {
                    return false;
                }
            }
            let e = matrix[j][i];
            if e == '.' {
                continue;
            }
            if !col_counts.insert(e) {
                return false;
            }
        }
    }
    true
}

fn check_sub_matrices(matrix: &Matrix) -> bool {
    for row in 0..3 {
        for col in 0..3 {
            if !check_sub_matrix(row * 3, col * 3, matrix) {
                return false;
            }
        }
    }
    return true;
}

fn check_sub_matrix(row: usize, col: usize, matrix: &Matrix) -> bool {
    let mut counts = HashSet::with_capacity(9);
    for i in 0..3 {
        for j in 0..3 {
            let e = matrix[row + i][col + j];
            if e == '.' {
                continue;
            }
            if !counts.insert(e) {
                return false;
            }
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_0() {
        let matrix: Matrix = vec![
            vec!['.', '.', '.', '1', '4', '.', '.', '2', '.'],
            vec!['.', '.', '6', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '1', '.', '.', '.', '.', '.', '.'],
            vec!['.', '6', '7', '.', '.', '.', '.', '.', '9'],
            vec!['.', '.', '.', '.', '.', '.', '8', '1', '.'],
            vec!['.', '3', '.', '.', '.', '.', '.', '.', '6'],
            vec!['.', '.', '.', '.', '.', '7', '.', '.', '.'],
            vec!['.', '.', '.', '5', '.', '.', '.', '7', '.'],
        ];
        assert_eq!(sudoku2(&matrix), true);
    }

    #[bench]
    fn bench_vec_method(b: &mut Bencher) {
        let matrix: Matrix = vec![
            vec!['.', '.', '.', '1', '4', '.', '.', '2', '.'],
            vec!['.', '.', '6', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '1', '.', '.', '.', '.', '.', '.'],
            vec!['.', '6', '7', '.', '.', '.', '.', '.', '9'],
            vec!['.', '.', '.', '.', '.', '.', '8', '1', '.'],
            vec!['.', '3', '.', '.', '.', '.', '.', '.', '6'],
            vec!['.', '.', '.', '.', '.', '7', '.', '.', '.'],
            vec!['.', '.', '.', '5', '.', '.', '.', '7', '.'],
        ];
        b.iter(|| sudoku2(&matrix));
    }
}
