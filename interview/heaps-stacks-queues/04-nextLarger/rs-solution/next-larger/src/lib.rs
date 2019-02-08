fn next_larger(a: Vec<i32>) -> Vec<i32> {
    let len = a.len();
    
    let mut output = vec![-1; len];
    let mut stack = Vec::new();
    
    for i in (0..len).rev() {
        while let Some(num) = stack.pop() {
            if num > a[i] {
                output[i] = num;
                stack.push(num);
                break;
            };
        }
        stack.push(a[i]);
    }
    
    output
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(next_larger(vec![6, 7, 3, 8]), vec![7, 8, 8, -1]);
    }
}
