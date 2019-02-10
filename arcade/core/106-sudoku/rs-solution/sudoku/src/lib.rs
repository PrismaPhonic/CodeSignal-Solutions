use std::collections::HashSet;

type Matrix = Vec<Vec<i32>>;

fn sudoku(grid: Matrix) -> bool {
    check_rows_cols(&grid) && check_sub_grids(&grid)
}

fn check_rows_cols(grid: &Matrix) -> bool {
    for i in 0..9 {
        let mut row_counts = HashSet::with_capacity(9);
        let mut col_counts = HashSet::with_capacity(9);
        for j in 0..9 {
            let e = grid[i][j];
            if !row_counts.insert(e) {
                return false;
            }
            let e = grid[j][i];
            if !col_counts.insert(e) {
                return false;
            }
        }
    }
    
    true
}

fn check_sub_grids(grid: &Matrix) -> bool {
    for row in 0..3 {
        for col in 0..3 {
            if !check_sub_grid(row * 3, col * 3, grid) {
                return false;
            }
        }
    }
    return true;
}

fn check_sub_grid(row: usize, col: usize, grid: &Matrix) -> bool {
    let mut counts = HashSet::with_capacity(9);
    for i in 0..3 {
        for j in 0..3 {
            let e = grid[row + i][col + j];
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

    #[test]
    fn test_0() {
        let grid = vec![
            vec![1,3,2,5,4,6,9,8,7], 
            vec![4,6,5,8,7,9,3,2,1], 
            vec![7,9,8,2,1,3,6,5,4], 
            vec![9,2,1,4,3,5,8,7,6], 
            vec![3,5,4,7,6,8,2,1,9], 
            vec![6,8,7,1,9,2,5,4,3], 
            vec![5,7,6,9,8,1,4,3,2], 
            vec![2,4,3,6,5,7,1,9,8], 
            vec![8,1,9,3,2,4,7,6,5]];
            assert!(sudoku(grid));
    }
}
