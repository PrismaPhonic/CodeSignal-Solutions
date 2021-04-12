use std::collections::VecDeque;

// Using a VecDeque because this problem is done. There's already built in tooling for this.
fn remove_k_from_list(l: VecDeque<i32>, k: i32) -> VecDeque<i32> {
    let mut new = l.clone();
    new.retain(|num| {
        *num != k
    });

    new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = VecDeque::from(vec![3, 1, 2, 3, 4, 5]);
        let want = VecDeque::from(vec![1, 2, 4, 5]);
        let got = remove_k_from_list(input, 3);
        assert_eq!(got, want);
    }
}
