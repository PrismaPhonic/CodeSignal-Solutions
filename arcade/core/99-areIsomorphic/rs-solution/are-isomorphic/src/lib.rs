fn are_isomorphic(array1: Vec<Vec<i32>>, array2: Vec<Vec<i32>>) -> bool {
    if array1.len() != array2.len() { return false };
    
    for i in 0..array1.len() {
        if array1[i].len() != array2[i].len() {
            return false;
        }
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let array1 = vec![vec![], vec![1]];

        let array2 = vec![vec![], vec![6, 2, 5]];

        assert_eq!(are_isomorphic(array1, array2), false);
    }
}
