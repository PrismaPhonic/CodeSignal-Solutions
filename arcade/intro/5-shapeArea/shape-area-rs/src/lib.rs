// n^2 + (n-1)^2
fn shape_area(n: i32) -> i32 {
    (n * n) + ((n-1) * (n-1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_case() {
        assert_eq!(shape_area(2), 5);
    }

    #[test]
    fn five_case() {
        assert_eq!(shape_area(5), 41);
    }
}
