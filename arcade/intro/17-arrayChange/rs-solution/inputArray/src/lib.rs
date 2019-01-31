use std::f32::NEG_INFINITY;

fn arrayChange(inputArray: Vec<i32>) -> i32 {
    let mut max = (NEG_INFINITY + 1.0) as i32;
    let mut round = 0;

    for num in &inputArray {
        max = if num > &max {
            *num
        } else {
            round += max - num + 1;
            max + 1
        };
    }

    round
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(arrayChange(vec![1,1,1]), 3);
    }
}
