use std::collections::HashMap;

fn contains_close_nums(nums: Vec<i32>, k: i32) -> bool {
    let mut count = HashMap::new();
    
    for (i, num) in nums.iter().enumerate() {
        let idx = *count.entry(num).or_insert(i);
        if idx != i {
            if (i - idx) as i32 <= k {
                return true;
            } else {
                count.entry(num).and_modify(|e| *e = i);
            }
        }
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert!(contains_close_nums(vec![4, 1, 2, 3, 4, 2], 3));
    }
}
