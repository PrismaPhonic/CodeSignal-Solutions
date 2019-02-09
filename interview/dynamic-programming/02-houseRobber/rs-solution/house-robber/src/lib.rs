fn house_robber(mut nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    };
    if nums.len() == 1 {
        return nums[0];
    };

    // get first out of the way
    if nums[0] > nums[1] {
        nums[1] = nums[0]
    };

    for i in 2..nums.len() {
        if nums[i - 1] > (nums[i - 2] + nums[i]) {
            nums[i] = nums[i - 1]
        } else {
            nums[i] += nums[i - 2]
        };
    }

    nums[nums.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let houses = vec![
            213, 59, 76, 87, 209, 98, 94, 175, 249, 123, 109, 233, 199, 162, 51, 92, 146, 154, 146,
            118, 183,
        ];
        assert_eq!(house_robber(houses), 1711);
    }
}
