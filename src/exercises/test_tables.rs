#[cfg(test)]
mod tests {
    use crate::fibonacci::fibonacci;
    use rstest::rstest;

    #[rstest]
    #[case(0, 0)]
    #[case(1, 1)]
    #[case(2, 1)]
    #[case(3, 2)]
    #[case(4, 3)]
    #[case(5, 5)]
    #[case(6, 8)]
    #[case(7, 13)]
    #[case(8, 21)]
    #[case(9, 34)]
    #[case(10, 55)]
    fn fibonacci_test(#[case] input: u64, #[case] expected: u64) {
        assert_eq!(expected, fibonacci(input))
    }
}
