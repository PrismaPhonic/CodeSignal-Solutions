use std::collections::HashSet;

struct Point(usize, usize);

fn minesweeper(matrix: Vec<Vec<bool>>) -> Vec<Vec<i32>> {
    let height = matrix.len();
    let width = matrix[0].len();
    
    let mut numbers = vec![vec![0; width]; height];
    
    for y in 0..height {
        for x in 0..width {
            if matrix[y][x] { 
                let directions = check_bounds(height, width, Point(x, y));
                
                if directions.contains("left") {
                    if directions.contains("bottom") {
                        numbers[y+1][x-1] += 1;
                    };
                    
                    numbers[y][x-1] += 1;
                    
                    if directions.contains("top") {
                        numbers[y-1][x-1] += 1;                       
                    };
                };
                
                if directions.contains("top") {
                    numbers[y-1][x] += 1;
                }

                if directions.contains("right") {
                    if directions.contains("top") {
                        numbers[y-1][x+1] += 1;
                    };
                    
                    numbers[y][x+1] += 1;
                    
                    if directions.contains("bottom") {
                        numbers[y+1][x+1] += 1;
                    };
                };
                
                if directions.contains("bottom") {
                    numbers[y+1][x] += 1;
                }
            }
        }
    }
    
    numbers
}

fn check_bounds(height: usize, width: usize, point: Point) -> HashSet<String> {
    let mut directions = HashSet::new();
    let Point(x, y) = point;
    
    if x > 0 {
        directions.insert("left".to_string());
    }
    if y > 0 {
        directions.insert("top".to_string());
    }
    if x < width-1 {
        directions.insert("right".to_string());
    }
    if y < height-1 {
        directions.insert("bottom".to_string());
    }
    
    directions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let matrix = vec![
            vec![true,false,false], 
            vec![false,true,false], 
            vec![false,false,false]];

        let output = vec![
            vec![1,2,1], 
            vec![2,1,1], 
            vec![1,1,1]];

        assert_eq!(minesweeper(matrix), output);
    }
}
