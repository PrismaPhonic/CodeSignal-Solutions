fn rotateImage(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let len = a.len();
    for layer in 0..len/2 {
        let first = layer;
        let last = len - 1 - layer;
        for i in first..last {
            let offset = i - first;
            let temp = a[first][i];
            
            a[first][i] = a[last-offset][first];
            a[last-offset][first] = a[last][last-offset];
            a[last][last-offset] = a[i][last];
            a[i][last] = temp;
        }
    }
    a
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn test_1() {
        assert_eq!(rotateImage(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }
}
