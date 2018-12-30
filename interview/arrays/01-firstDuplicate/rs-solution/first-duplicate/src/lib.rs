#![feature(test)]

/**
 * Note: If you want to try benchmarking hashbrown crate
 * which uses googles swisstable hashmap just uncomment
 * extern crate and use lines for hashbrown and comment
 * out use line of stadard library hashset
 */

extern crate test;
// extern crate hashbrown;

use std::collections::HashSet;
// use hashbrown::HashSet;

fn firstDuplicate(a: Vec<i32>) -> i32 {
    let mut occurances: HashSet<i32> = HashSet::new();

    for num in a {
        if occurances.insert(num) == false {
            return num;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_1() {
        let input = vec![2, 1, 3, 5, 3, 2];
        
        assert_eq!(firstDuplicate(input), 3);
    }

    #[test]
    fn test_2() {
        let input = vec![2, 4, 3, 5, 1];
        
        assert_eq!(firstDuplicate(input), -1);
    }

    #[test]
    fn test_9() {
        let input = vec![8, 4, 6, 2, 6, 4, 7, 9, 5, 8];
        
        assert_eq!(firstDuplicate(input), 6);
    }

    #[bench]
    fn bench_test_9(b: &mut Bencher) {
        b.iter(|| firstDuplicate(vec![8, 4, 6, 2, 6, 4, 7, 9, 5, 8]));
    }

}
