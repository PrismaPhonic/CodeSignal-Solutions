fn climbing_stairs(n: i32) -> i32 {
    if n == 1 { return 1 };
    
    let mut ways_each = vec![0, 1, 2];
    
    for i in 3..=n as usize {
        let result = ways_each[i-1] + ways_each[i-2];
        ways_each.push(result);
    }
    
    ways_each[n as usize]
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn test_1() {
        assert_eq!(climbing_stairs(1), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(climbing_stairs(2), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(climbing_stairs(26), 196418);
    }

    #[test]
    fn test_4() {
        assert_eq!(climbing_stairs(38), 63245986);
    }

    #[test]
    fn test_5() {
        assert_eq!(climbing_stairs(28), 514229);
    }
}
