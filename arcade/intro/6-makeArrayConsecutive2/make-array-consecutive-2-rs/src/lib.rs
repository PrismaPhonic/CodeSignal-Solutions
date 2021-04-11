// Problem: You are given a list of integers representing statue sizes, which include
// the smallest and largest statue. They need to be arranged in order with each statue being exactly
// 1 size larger than the previous. Return how many statues are needed to fill the gaps.
//
// Solution: This seems like a simple problem of sorting and then counting gaps.

fn make_array_consecutive2(statues: Vec<i32>) -> i32 {
    if statues.len() == 1 {
        return 0;
    }
    let mut sorted_statues = statues.clone();
    sorted_statues.sort();
    sorted_statues.windows(2).map(|nums| {
        nums[1] - nums[0] - 1
    }).sum()
}

#[cfg(test)]
mod tests {
    use crate::make_array_consecutive2;

    #[test]
    fn it_works() {
        let statues = vec![6, 2, 3, 8];
        let want = 3;
        let got = make_array_consecutive2(statues);
        assert_eq!(want, got);
    }
}
