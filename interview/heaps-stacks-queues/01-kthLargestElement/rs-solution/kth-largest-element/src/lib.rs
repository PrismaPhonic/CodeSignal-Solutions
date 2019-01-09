#![feature(test)]

extern crate test;

use std::collections::BinaryHeap;

pub fn kth_largest_element(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap = BinaryHeap::from(nums);
    
    for _ in 0..k-1 { heap.pop(); }
    
    heap.pop().unwrap()
}

pub fn kth_largest_element_sort(nums: &mut Vec<i32>, k: i32) -> i32 {
    nums[..].sort_unstable();

    nums[nums.len()-k as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_1() {
        assert_eq!(kth_largest_element(vec![7, 6, 5, 4, 3, 2, 1], 2), 6);
    }

    #[test]
    fn test_1_sort() {
        let mut vec = vec![7, 6, 5, 4, 3, 2, 1]; 
        assert_eq!(kth_largest_element_sort(&mut vec, 2), 6);
    }

    #[bench]
    fn bench_1(b: &mut Bencher) {
        b.iter(|| kth_largest_element(vec![7, 6, 5, 4, 3, 2, 1], 2))
    }

    #[bench]
    fn bench_1_sort(b: &mut Bencher) {
        let mut vec = vec![7, 6, 5, 4, 3, 2, 1]; 
        b.iter(|| kth_largest_element_sort(&mut vec, 2))
    }

    #[bench]
    fn bench_huge(b: &mut Bencher) {
        b.iter(|| kth_largest_element(vec![7, 2, 6, 5, 4, 8, 6, 7, 9, 4, 5, 6, 2, 3, 4,5, 7, 17, 29, 99, 65, 44, 33, 22, 1, 3, 4, 5, 6, 66, 77, 88, 99, 200, 500, 22, 11, 10, 9, 8, 3, 5, 6, 2, 8, 9], 10))
    }

    #[bench]
    fn bench_huge_sort(b: &mut Bencher) {
        let mut vec = vec![7, 2, 6, 5, 4, 8, 6, 7, 9, 4, 5, 6, 2, 3, 4,5, 7, 17, 29, 99, 65, 44, 33, 22, 1, 3, 4, 5, 6, 66, 77, 88, 99, 200, 500, 22, 11, 10, 9, 8, 3, 5, 6, 2, 8, 9]; 
        b.iter(|| kth_largest_element_sort(&mut vec, 10))
    }

    #[test]
    fn test_2() {
        assert_eq!(kth_largest_element(vec![99, 99], 1), 99);
    }

    #[test]
    fn test_3() {
        assert_eq!(kth_largest_element(vec![1], 1), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(kth_largest_element(vec![2, 1], 1), 2);
    }

    #[test]
    fn test_5() {
        assert_eq!(kth_largest_element(vec![-1, 2, 0], 2), 0);
    }

    #[test]
    fn test_6() {
        assert_eq!(kth_largest_element(vec![-1, 2, 0], 3), -1);
    }

    #[test]
    fn test_7() {
        assert_eq!(kth_largest_element(vec![3, 1, 2, 4], 2), 3);
    }

    #[test]
    fn test_8() {
        assert_eq!(kth_largest_element(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }

    #[test]
    fn test_9() {
        assert_eq!(kth_largest_element(vec![5, 2, 4, 1, 3, 6, 0], 2), 5);
    }
    
    #[test]
    fn test_10() {
        assert_eq!(kth_largest_element(vec![3, 3, 3, 3, 3, 3, 3, 3, 3], 8), 3);
    }
}
