// Problem: Given a sequence of integers as an array, determine whether it is possible
// to obtain a strictly increasing sequence by removing no more than one element from the array.

fn almost_increasing_sequence(sequence: Vec<i32>) -> bool {
    let count1 = sequence.windows(2).filter(|p| p[0] >= p[1]).count();
    let count2 = sequence.windows(3).filter(|t| t[0] >= t[2]).count();
    count1 < 2 && count2 < 2
}

#[cfg(test)]
mod tests {
    use crate::almost_increasing_sequence;

    #[test]
    fn it_works() {
        let sequence = vec![1, 3, 2, 1];
        let want = false;
        let got = almost_increasing_sequence(sequence);
        assert_eq!(got, want);
    }
}
