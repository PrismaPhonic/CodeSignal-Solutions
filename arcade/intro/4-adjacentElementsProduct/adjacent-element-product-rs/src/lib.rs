fn adjacent_elements_product(input: Vec<i32>) -> i32 {
    input.windows(2).map(|nums|{
        nums[0] * nums[1]
    }).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![3, 6, -2, -5, 7, 3];
        let want = 21;
        let got = adjacent_elements_product(input);
        assert_eq!(got, want)
    }
}
